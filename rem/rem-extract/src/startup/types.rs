use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use anyhow::Context;

use ra_ap_base_db::{
    CrateGraph, FileSet, ProcMacroPaths
};
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::{FileId, VfsPath};

/// Shared context for analysis. Constructd once and cloned as needed.
pub struct SingleFileStdContext {
    pub base_graph: CrateGraph,
    pub proc_macros: ProcMacroPaths,
    pub sysroot_files: SysrootFileMap,
}


/// Dumb path→FileId map; good enough for a first cut.
#[derive(Default, Clone)]
pub struct SysrootFileMap {
    next_id: u32,
    paths: Vec<(AbsPathBuf, FileId)>,
}


impl SysrootFileMap {
    pub fn new() -> Self {
        Self { next_id: 0, paths: Vec::new() }
    }

    pub fn file_id_for(&mut self, path: AbsPathBuf) -> FileId {
        if let Some((_, id)) = self.paths.iter().find(|(p, _)| *p == path) {
            return *id;
        }

        let id = FileId::from_raw(self.next_id);
        self.next_id += 1;
        self.paths.push((path, id));
        id
    }

    /// Iterate all sysroot file IDs.
    pub fn file_ids(&self) -> impl Iterator<Item = FileId> + '_ {
        self.paths.iter().map(|(_, id)| *id)
    }

    /// Build a FileSet containing all sysroot files.
    pub fn to_file_set(&self) -> FileSet {
        let mut fs = FileSet::default();
        for (abs, id) in &self.paths {
            // In ra_ap, VfsPath usually has a From<AbsPathBuf> impl.
            let vfs_path = VfsPath::from(abs.clone());
            fs.insert(*id, vfs_path);
        }
        fs
    }

    pub fn entries(&self) -> &[(AbsPathBuf, FileId)] {
        &self.paths
    }
}

pub struct TempScriptAnchor {
    path: PathBuf,
}

impl TempScriptAnchor {
    pub fn new() -> anyhow::Result<Self> {
        let mut path = std::env::temp_dir();
        path.push("rem_dummy_detached.rs");

        // Create or truncate the file and write some minimal Rust so RA
        // doesn’t choke if it does try to read it.
        let mut file = File::create(&path)
            .with_context(|| format!("failed to create temp dummy script at {path:?}"))?;
        file.write_all(b"fn main() {}\n")?;

        Ok(Self { path })
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

// impl Drop for TempScriptAnchor {
//     fn drop(&mut self) {
//         if let Err(err) = fs::remove_file(&self.path) {
//             // Best-effort cleanup; log and move on.
//             eprintln!("warn: failed to remove temp dummy script {:?}: {err}", self.path);
//         }
//     }
// }