use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::exports::VerificationError;

/// Single, shared temp workspace for verification artifacts (LLBC, Coq, etc.).
#[derive(Debug)]
pub struct SharedTempDir {
    path: PathBuf,
}
impl SharedTempDir {
    /// Create one shared temp dir; keep this alive until verification ends.
    pub fn new(prefix: &str) -> Result<Self, VerificationError> {
        let mut p = std::env::temp_dir();
        let unique = format!(
            "{}-{}-{}",
            prefix,
            std::process::id(),
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        p.push(unique);
        std::fs::create_dir_all(&p).map_err(|e| VerificationError {
            msg: format!("Failed to create temp dir {}: {e}", p.display()),
        })?;
        Ok(Self { path: p })
    }

    pub fn path(&self) -> &Path { &self.path }

    /// Make (and return) a named subdirectory (e.g., "charon/original").
    /// Does nothing if it already exists.
    pub fn subdir(&self, rel: &str) -> Result<PathBuf, VerificationError> {
        let sub_path = self.path.join(rel);
        if sub_path.exists() {
            return Ok(sub_path);
        }
        std::fs::create_dir_all(&sub_path).map_err(|e| VerificationError {
            msg: format!(
                "Failed to create subdir {} in temp dir: {e}",
                sub_path.display()
            ),
        })?;
        Ok(sub_path)
    }

    #[allow(dead_code)]
    pub fn list_subdirs(&self) -> Result<Vec<PathBuf>, VerificationError> {
        let mut subs = Vec::new();
        for entry in std::fs::read_dir(&self.path).map_err(|e| VerificationError {
            msg: format!("Failed to read temp dir {}: {e}", self.path.display()),
        })? {
            let entry = entry.map_err(|e| VerificationError {
                msg: format!("Failed to read entry in temp dir {}: {e}", self.path.display()),
            })?;
            let path = entry.path();
            if path.is_dir() {
                subs.push(path);
            }
        }
        Ok(subs)
    }
}
impl Drop for SharedTempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}