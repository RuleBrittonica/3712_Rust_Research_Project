use std::path::{Path, PathBuf};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io;
use std::time::SystemTime;
use crate::common::RepairResult;
use crate::common::RepairSystem;
use crate::repair_lifetime_simple;
use crate::repair_rustfix;
use crate::repair_lifetime_tightest_bound_first;
use crate::repair_lifetime_loosest_bound_first;


#[derive(Debug)]
pub struct RepairError {
    pub msg: String,
}

impl std::fmt::Display for RepairError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RepairError: {}", self.msg)
    }
}

impl std::error::Error for RepairError {}

// If we get a return then by definition the repair succeeded
#[derive(Debug)]
pub struct RepairReturn<'a> {
    pub idx: u8,
    pub system_name: &'a str,
    pub repair_count: u8,
    pub changed_files: Vec<String>,
}

pub struct RepairerInput {
    file: PathBuf,
    new_fn_name: String,
}

impl RepairerInput {
    pub fn new(file: PathBuf, new_fn_name: String) -> Self {
        Self { file, new_fn_name }
    }
}

pub fn call_all_repairers(
    input: RepairerInput
) -> Result<RepairReturn<'static>, RepairError> {
    // The process will look something like this:
    // 1. Derive the (minimal) crate root from the file path (assuming the file
    //    is in a crate - return an error if it isn't).
    // 1.1 (Optional) Come up with a very lightweight way to check files before
    // and after so we can generate a list of repaired files!
    // 2. For each repairer in the list of repairers, run it
    // 3. If the repairer succeeds at any point, return success
    // 4. If all repairers fail, delete the temporary file and return an error
    // indicating failure (if we have returned an error then we know that the
    // original file was untouched)

    let manifest_path = find_manifest_for(&input.file)
        .ok_or_else(|| RepairError {
            msg: format!(
                "Could not find Cargo.toml for file: {}",
                input.file.display()
            ),
        })?;

    let manifest_path_str = manifest_path.to_str().ok_or_else(|| RepairError {
        msg: format!(
            "Could not convert manifest path to string: {}",
            manifest_path.display()
        ),
    })?;

    let before = snapshot_crate_files(manifest_path.parent().unwrap());

    let repair_systems: Vec<&dyn RepairSystem> = vec![
        &repair_lifetime_simple::Repairer {},
        &repair_rustfix::Repairer {},
        &repair_lifetime_tightest_bound_first::Repairer {},
        &repair_lifetime_loosest_bound_first::Repairer {},
    ];

    let src_path: &str = input.file.to_str().ok_or_else(|| RepairError {
        msg: format!(
            "Could not convert source file path to string: {}",
            input.file.display()
        ),
    })?;
    let src_content = std::fs::read_to_string(&src_path).map_err(|e| RepairError {
        msg: format!(
            "Could not read source file {}: {}",
            src_path,
            e
        ),
    })?; // Backup of the original content, will be used to restore if needed
    let fn_name: &str = input.new_fn_name.as_str(); // This is the (extracted) function's name

    for (idx, system) in repair_systems.iter().enumerate() {
        let result: RepairResult = system.repair_project(
            src_path,
            manifest_path_str,
            fn_name
        );

        if result.success {

            let after = snapshot_crate_files(manifest_path.parent().unwrap());
            let changed = diff_snapshots(&before, &after);

            let changed_files: Vec<String> = changed.iter()
                .map(|p| p.display().to_string())
                .collect();

            return Ok(
                RepairReturn {
                    idx: idx as u8,
                    system_name: system.name(),
                    repair_count: result.repair_count as u8,
                    changed_files,
                }
            );
        } else {
            // Repair failed, restore original content
            std::fs::write(&src_path, &src_content).map_err(|e| RepairError {
                msg: format!(
                    "Could not restore original content to source file {}: {}",
                    src_path,
                    e
                ),
            })?;
        }
    }

    // All repairers failed
    Err(RepairError {
        msg: "All repairers failed; original file unchanged".into(),
    })

}


/// Finds the nearest `Cargo.toml` from `path` upward.
fn find_manifest_for(path: &Path) -> Option<PathBuf> {
    // Fast path: the input is already a Cargo.toml file.
    if path.file_name().is_some_and(|n| n == "Cargo.toml") && path.is_file() {
        return Some(path.to_path_buf());
    }

    // Choose a starting directory: `path` if dir, else its parent.
    let start_dir = if path.is_dir() { path } else { path.parent()? };

    // Walk ancestors without allocating a mutable PathBuf we pop from.
    for dir in start_dir.ancestors() {
        let candidate = dir.join("Cargo.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

#[derive(Clone, Debug)]
struct FileStamp {
    len: u64,
    mtime: Option<SystemTime>,
}

fn snapshot_crate_files(crate_root: &Path) -> BTreeMap<PathBuf, FileStamp> {
    let mut map = BTreeMap::new();
    // Cheap heuristic: index files under src/ plus the exact file we’re touching
    let roots = [crate_root.join("src")];
    for root in roots {
        if root.is_dir() {
            let _ = walk_dir(&root, &mut |p| {
                if let Ok(md) = fs::metadata(p) {
                    if md.is_file() {
                        let mtime = md.modified().ok();
                        map.insert(p.to_path_buf(), FileStamp { len: md.len(), mtime });
                    }
                }
            });
        }
    }
    map
}

fn walk_dir(root: &Path, f: &mut impl FnMut(&Path)) -> io::Result<()> {
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let p = entry.path();
            if entry.file_type()?.is_dir() {
                stack.push(p);
            } else {
                f(&p);
            }
        }
    }
    Ok(())
}

fn diff_snapshots(
    before: &BTreeMap<PathBuf, FileStamp>,
    after: &BTreeMap<PathBuf, FileStamp>,
) -> BTreeSet<PathBuf> {
    let mut changed = BTreeSet::new();

    for (p, a) in after {
        match before.get(p) {
            None => { changed.insert(p.clone()); }
            Some(b) => {
                if b.len != a.len || b.mtime != a.mtime {
                    changed.insert(p.clone());
                }
            }
        }
    }
    for p in before.keys() {
        if !after.contains_key(p) {
            changed.insert(p.clone());
        }
    }
    changed
}