use std::{path::PathBuf, thread, time::Duration};
use anyhow::{Context, Result};

mod daemon_init;
mod watcher;
mod utils;

use watcher::start_watcher;
use daemon_init::init_daemon;


/// Start the background analysis daemon, then attach the hash-aware watcher.
/// For now, this only prints when a .rs file is Created/Modified/Removed *by content*.
pub fn run_daemon(manifest_path: PathBuf) -> Result<()> {
    let core = init_daemon(&manifest_path)
        .with_context(|| format!("failed to initialise daemon from {}", manifest_path.display()))?;

    let root_to_watch: &ra_ap_vfs::AbsPath = core.workspace.workspace_root();
    println!("[daemon] initialised. watching: {}", root_to_watch);

    // Seed with the daemon’s initial hashed_files
    let root_to_watch = PathBuf::from(root_to_watch.to_string());
    let _watch = start_watcher(root_to_watch, core.hashed_files)?;

    println!("[daemon] running. Press Ctrl-C to exit.");
    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

pub fn close_daemon(manifest_path: PathBuf) -> Result<()> {
    println!("[daemon] shutting down for manifest: {}", manifest_path.display());
    Ok(())
}