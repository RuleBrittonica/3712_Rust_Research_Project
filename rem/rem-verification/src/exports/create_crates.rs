use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::exports::VerificationError;

#[derive(Debug)]
struct TempDirGuard {
    path: PathBuf,
}
impl TempDirGuard {
    fn new(prefix: &str) -> io::Result<Self> {
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
        fs::create_dir_all(&p)?;
        Ok(Self { path: p })
    }
    fn path(&self) -> &Path {
        &self.path
    }
}
impl Drop for TempDirGuard {
    fn drop(&mut self) {
        // best effort cleanup
        let _ = fs::remove_dir_all(&self.path);
    }
}

// Copy the crate tree except for heavy/irrelevant dirs.
fn copy_crate_tree(src_root: &Path, dst_root: &Path) -> io::Result<()> {
    fn should_skip(p: &Path) -> bool {
        // FIXME better skip logic
        p.components().any(|c| {
            let ok = c.as_os_str();
            ok == "target" ||
            ok == ".git" || ok == ".github" ||
            ok == "node_modules" ||
            ok == "dist" ||
            ok == "docs" ||
            ok == "examples" ||
            ok == "tests" ||
            ok == "images" ||
            ok == "ci" ||
            ok == "etc"
        })
    }

    let mut stack = vec![src_root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let rel = path.strip_prefix(src_root).unwrap();

            if should_skip(rel) {
                continue;
            }
            let dst = dst_root.join(rel);

            let ft = entry.file_type()?;
            if ft.is_dir() {
                fs::create_dir_all(&dst)?;
                stack.push(path);
            } else if ft.is_file() {
                if let Some(parent) = dst.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &dst)?;
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
pub struct MinimalCrate {
    guard: TempDirGuard,
    pub manifest: PathBuf,
}

impl MinimalCrate {
    /// Copy the entire crate (crate_root inferred from manifest_path)
    pub fn new(manifest_path: &Path, prefix: &str) -> Result<Self, VerificationError> {
        let crate_root = manifest_path.parent().ok_or_else(|| VerificationError {
            msg: format!("Manifest has no parent: {}", manifest_path.display()),
        })?;

        let guard = TempDirGuard::new(prefix).map_err(|e| VerificationError {
            msg: format!("Failed to create temp dir for crate copy: {e}"),
        })?;

        copy_crate_tree(crate_root, guard.path()).map_err(|e| VerificationError {
            msg: format!("Failed to copy crate contents: {e}"),
        })?;

        let manifest = guard.path().join("Cargo.toml");
        Ok(Self { guard, manifest })
    }

    /// Replace one file (relative to the original crate) with new contents.
    pub fn replace_file(
        &self,
        original_crate_root: &Path,
        file_path_in_original: &Path,
        new_contents: &str,
    ) -> Result<(), VerificationError> {
        let rel_path = file_path_in_original.strip_prefix(original_crate_root).map_err(|e| {
            VerificationError {
                msg: format!(
                    "File is not under crate root ({} vs {}): {e}",
                    file_path_in_original.display(),
                    original_crate_root.display()
                ),
            }
        })?;
        let dst_path = self.guard.path().join(rel_path);
        if let Some(parent) = dst_path.parent() {
            fs::create_dir_all(parent).map_err(|e| VerificationError {
                msg: format!("Failed to create parent dirs for {}: {e}", dst_path.display()),
            })?;
        }
        fs::write(&dst_path, new_contents).map_err(|e| VerificationError {
            msg: format!(
                "Failed to write new contents to {}: {e}",
                dst_path.display()
            ),
        })?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn root(&self) -> &Path {
        self.guard.path()
    }
}

#[derive(Debug)]
pub struct TwinCrates {
    pub original: MinimalCrate,
    pub refactored: MinimalCrate,
}

impl TwinCrates {
    pub fn new(
        manifest_path: &Path,
        original_file_path: &Path,
        refactored_content: &str,
    ) -> Result<Self, VerificationError> {
        let crate_root = manifest_path.parent().ok_or_else(|| VerificationError {
            msg: format!("Manifest has no parent: {}", manifest_path.display()),
        })?;

        let original = MinimalCrate::new(manifest_path, "remv-orig")?;
        let refactored = MinimalCrate::new(manifest_path, "remv-refac")?;

        refactored.replace_file(crate_root, original_file_path, refactored_content)?;

        Ok(Self { original, refactored })
    }
}