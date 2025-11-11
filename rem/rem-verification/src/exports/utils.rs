use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

use crate::exports::VerificationError;

/// Finds the nearest `Cargo.toml` from `path` upward.
pub fn find_manifest_for(path: &Path) -> Option<PathBuf> {
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

/// Ensure `Primitives.v` exists in `dir`.
pub fn ensure_primitives_file(dir: &Path) -> Result<(), VerificationError> {
    let primitives = dir.join("Primitives.v");
    if primitives.exists() {
        return Ok(());
    }
    fs::write(&primitives, include_str!("../Primitives.v")).map_err(|e| VerificationError {
        msg: format!("Failed to write {}: {}", primitives.display(), e),
    })?;
    Ok(())
}

#[derive(Debug)]
pub struct CmdOutput {
    pub command: String,
    pub status: std::process::ExitStatus,
    #[allow(dead_code)]
    pub stdout: String,
    #[allow(dead_code)]
    pub stderr: String,
}

pub fn run_command(
    program: &OsStr,
    args: &[impl AsRef<OsStr>],
    cwd: Option<&Path>,
    stdin_bytes: Option<&[u8]>,
    extra_env: &[(&str, &str)],
) -> Result<CmdOutput, VerificationError> {
    let mut cmd = Command::new(program);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }
    for (k, v) in extra_env {
        cmd.env(k, v);
    }

    // Construct the full command line for reporting
    let mut command_str = format!("{}", program.to_string_lossy());
    for arg in args {
        let a = arg.as_ref().to_string_lossy();
        // Quote arguments with spaces or special chars for readability
        if a.contains(' ') || a.contains('"') || a.contains('\'') {
            command_str.push(' ');
            command_str.push('"');
            command_str.push_str(&a.replace('"', "\\\""));
            command_str.push('"');
        } else {
            command_str.push(' ');
            command_str.push_str(&a);
        }
    }

    let mut child = cmd.spawn().map_err(|e| VerificationError {
        msg: format!(
            "Failed to spawn command `{}`: {}",
            program.to_string_lossy(),
            e
        ),
    })?;

    // write stdin (optional)
    if let Some(input) = stdin_bytes {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input).map_err(|e| VerificationError {
                msg: format!(
                    "Failed to write stdin to `{}`: {}",
                    program.to_string_lossy(),
                    e
                ),
            })?;
        }
    } else {
        // Close explicitly to avoid tools waiting on stdin
        drop(child.stdin.take());
    }

    // Read both pipes fully
    let mut out = String::new();
    let mut err = String::new();
    if let Some(mut so) = child.stdout.take() {
        so.read_to_string(&mut out).ok();
    }
    if let Some(mut se) = child.stderr.take() {
        se.read_to_string(&mut err).ok();
    }

    let status = child.wait().map_err(|e| VerificationError {
        msg: format!(
            "Failed to wait for `{}`: {}",
            program.to_string_lossy(),
            e
        ),
    })?;

    Ok(CmdOutput {
        command: command_str,
        status,
        stdout: out,
        stderr: err,
    })
}

#[cfg(test)]
mod tests {
    use super::find_manifest_for;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn write_file(path: &std::path::Path, contents: &str) {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).unwrap();
        }
        let mut f = File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn returns_self_when_given_manifest_file() {
        let dir = tempdir().unwrap();
        let manifest = dir.path().join("Cargo.toml");
        write_file(&manifest, "[package]\nname = \"demo\"");

        let found = find_manifest_for(&manifest);
        assert_eq!(found.as_deref(), Some(manifest.as_path()));
    }

    #[test]
    fn finds_manifest_from_child_directory() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let manifest = root.join("Cargo.toml");
        write_file(&manifest, "[package]\nname = \"demo\"");

        let child = root.join("src/bin/nested");
        fs::create_dir_all(&child).unwrap();

        let found = find_manifest_for(&child);
        assert_eq!(found.as_deref(), Some(manifest.as_path()));
    }

    #[test]
    fn finds_manifest_when_starting_from_a_file() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let manifest = root.join("Cargo.toml");
        write_file(&manifest, "[package]\nname = \"demo\"");

        let file_path = root.join("src/main.rs");
        write_file(&file_path, "fn main() {}");

        let found = find_manifest_for(&file_path);
        assert_eq!(found.as_deref(), Some(manifest.as_path()));
    }

    #[test]
    fn returns_none_when_no_manifest_up_the_tree() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let deep = root.join("a/b/c");
        fs::create_dir_all(&deep).unwrap();

        let found = find_manifest_for(&deep);
        assert!(found.is_none());
    }

    #[test]
    fn walks_up_multiple_levels() {
        let dir = tempdir().unwrap();
        let level0 = dir.path().join("workspace");
        let level1 = level0.join("crates");
        let level2 = level1.join("foo");
        fs::create_dir_all(&level2).unwrap();

        let manifest = level0.join("Cargo.toml");
        write_file(&manifest, "[workspace]");

        let found = find_manifest_for(&level2);
        assert_eq!(found.as_deref(), Some(manifest.as_path()));
    }

    #[test]
    fn nonexistent_leaf_path_still_finds_ancestor_manifest() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        let manifest = root.join("Cargo.toml");
        write_file(&manifest, "[package]\nname = \"demo\"");

        // Construct a *nonexistent* deeper path inside `root`.
        let ghost = root.join("not/actually/there/deeper");
        // We intentionally do not create `ghost`.

        let found = find_manifest_for(&ghost);
        assert_eq!(found.as_deref(), Some(manifest.as_path()));
    }

    #[test]
    fn given_manifest_named_but_is_directory_not_file_returns_none_or_parent() {
        // Guard against a false positive when a directory named Cargo.toml exists.
        let dir = tempdir().unwrap();
        let root = dir.path();
        let fake = root.join("Cargo.toml");
        fs::create_dir_all(&fake).unwrap();

        // Also place a real manifest one level up from a child path.
        let real_manifest = root.join("real/Cargo.toml");
        write_file(&real_manifest, "[package]\nname = \"real\"");

        let child = real_manifest.parent().unwrap().join("src");
        fs::create_dir_all(&child).unwrap();

        // Starting at the directory with a real manifest up its tree should find that one.
        let found = super::find_manifest_for(&child);
        assert_eq!(found.as_deref(), Some(real_manifest.as_path()));

        // Starting at the directory named "Cargo.toml" should *not* treat it as a file.
        let found2 = super::find_manifest_for(&fake);
        assert!(found2.is_none());
    }
}