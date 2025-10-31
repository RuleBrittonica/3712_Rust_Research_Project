use std::{fs::File, path::PathBuf, path::Path};
use walkdir::WalkDir;
use sha2::{Sha256, Digest};

use crate::{daemon::daemon_init::FileRepr, extraction_utils::convert_to_abs_path_buf};

/// Enumerate all `.rs` files under the workspace root (excluding target/ and hidden dirs).
pub fn enumerate_rust_files(root: &PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for e in WalkDir::new(root)
        .into_iter()
        .filter_entry(|de| should_descend(de.path()))
        .filter_map(|e| e.ok())
    {
        let p = e.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p.to_path_buf());
        }
    }
    out
}

pub fn hash_file_list(list: Vec<PathBuf>) -> Vec<FileRepr> {
    let mut out = Vec::new();

    for path in list {
        let file = File::open(&path).expect("Failed to open file for hashing");
        let contents = std::fs::read_to_string(&path).expect("Failed to read file for hashing");
        let mut hasher = Sha256::new();
        hasher.update(contents.as_bytes());
        let hash_result = hasher.finalize();
        let hash_string = format!("{:x}", hash_result);
        out.push(FileRepr {
            path: convert_to_abs_path_buf(path.to_str().unwrap()).unwrap(),
            hash: hash_string,
        });
    }
    out
}

fn should_descend(p: &Path) -> bool {
    // Skip target/, .git/, .hg/, .svn/, node_modules/, and hidden dirs to reduce noise.
    if let Some(name) = p.file_name().and_then(|s| s.to_str()) {
        let lower = name.to_ascii_lowercase();
        if lower == "target"
            || lower == ".git"
            || lower == ".hg"
            || lower == ".svn"
            || lower == "node_modules"
        {
            return false;
        }
        // Skip other dot-directories at top levels
        if name.starts_with('.') && p.is_dir() {
            return false;
        }
    }
    true
}