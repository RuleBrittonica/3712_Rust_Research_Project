use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use anyhow::Result;
use notify::{event::ModifyKind, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use sha2::{Digest, Sha256};

use crate::daemon::daemon_init::FileRepr;

/// Keep the watcher alive for the daemon lifetime.
pub struct WatchHandle {
    _watcher: RecommendedWatcher,
    _pump: thread::JoinHandle<()>,
}

/// Start a recursive watcher at `root`.
/// - `seed` initializes the (path -> sha256) cache from DaemonCore.hashed_files.
/// - Only prints when a file’s **content hash changes**, or when a file is created/removed.
pub fn start_watcher(root: PathBuf, seed: Vec<FileRepr>) -> Result<WatchHandle> {
    // Seed the hash cache from daemon
    let mut cache: HashMap<PathBuf, String> = HashMap::with_capacity(seed.len());
    for FileRepr { path, hash } in seed {
        // AbsPathBuf -> PathBuf
        cache.insert(PathBuf::from(path.as_str()), hash);
    }

    // Channel for watcher -> pump thread
    let (tx, rx) = mpsc::channel::<notify::Result<Event>>();

    // Configure watcher (no extra config; compatible across notify versions)
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(&root, RecursiveMode::Recursive)?;

    // Pump thread: debounce, filter, hash-compare, print
    let _pump = thread::spawn(move || {
        let debounce_window = Duration::from_millis(150);

        loop {
            // Block for first event
            let first = match rx.recv() {
                Ok(Ok(ev)) => ev,
                Ok(Err(e)) => {
                    eprintln!("[watcher] error: {e}");
                    continue;
                }
                Err(_) => break, // channel closed
            };

            // Collect a short burst of events
            let start = Instant::now();
            let mut burst: Vec<Event> = vec![first];
            while start.elapsed() < debounce_window {
                match rx.try_recv() {
                    Ok(Ok(ev)) => burst.push(ev),
                    Ok(Err(e)) => eprintln!("[watcher] error: {e}"),
                    Err(mpsc::TryRecvError::Empty) => thread::sleep(Duration::from_millis(10)),
                    Err(mpsc::TryRecvError::Disconnected) => break,
                }
            }

            // Flatten to a unique set of candidate paths
            let mut candidates: HashSet<PathBuf> = HashSet::new();
            for ev in burst.into_iter() {
                // We accept all paths in the event; rename will provide old+new
                for p in ev.paths {
                    // Skip directories and noise early
                    if p.is_dir() {
                        continue;
                    }
                    // Only consider .rs files and skip noisy dirs (target, dot dirs, node_modules)
                    if !is_relevant_rust_file(&p) {
                        continue;
                    }
                    candidates.insert(p);
                }

                // Additionally, if this was a rename, we want both old and new paths considered.
                if let EventKind::Modify(ModifyKind::Name(_)) = ev.kind {
                    // already added above – nothing else to do
                }
            }

            // For each candidate, decide Created / Modified / Removed by cache+exists+hash
            for path in candidates {
                let existed_before = cache.contains_key(&path);
                let exists_now = path.exists();

                match (existed_before, exists_now) {
                    (true, false) => {
                        // Removed
                        cache.remove(&path);
                        println!("[watcher] Removed  {}", path.display());
                    }
                    (_, true) => {
                        // Created or Modified (or rename target)
                        if let Some(new_hash) = try_hash(&path) {
                            match cache.get(&path) {
                                None => {
                                    cache.insert(path.clone(), new_hash);
                                    println!("[watcher] Created  {}", path.display());
                                }
                                Some(old_hash) if *old_hash != new_hash => {
                                    cache.insert(path.clone(), new_hash);
                                    println!("[watcher] Modified {}", path.display());
                                }
                                _ => {
                                    // Same content → suppress noise
                                }
                            }
                        } else {
                            // Could not read/hash; ignore silently
                        }
                    }
                    (false, false) => {
                        // Neither in cache nor on disk — nothing to report.
                    }
                }
            }
        }
    });

    Ok(WatchHandle {
        _watcher: watcher,
        _pump,
    })
}

fn is_relevant_rust_file(p: &Path) -> bool {
    // Extension must be .rs
    if p.extension().and_then(|s| s.to_str()) != Some("rs") {
        return false;
    }

    // Skip files in noisy directories: target/, dot dirs, node_modules/
    if let Some(mut anc) = p.parent() {
        while let Some(dir) = anc.file_name().and_then(|s| s.to_str()) {
            let low = dir.to_ascii_lowercase();
            if low == "target" || low == "node_modules" {
                return false;
            }
            if dir.starts_with('.') {
                return false;
            }
            anc = match anc.parent() {
                Some(a) => a,
                None => break,
            };
        }
    }

    true
}

fn try_hash(path: &Path) -> Option<String> {
    let Ok(bytes) = fs::read(path) else { return None; };
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    Some(format!("{:x}", hasher.finalize()))
}
