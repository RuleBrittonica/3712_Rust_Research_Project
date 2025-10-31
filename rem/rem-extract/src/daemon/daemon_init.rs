//! daemon/init.rs
//! Step 1: Initialise the Daemon (persistent RA state + file lists)

use anyhow::{anyhow, Context, Result};
use ra_ap_ide::RootDatabase;
use ra_ap_project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use ra_ap_vfs::{Vfs, AbsPathBuf};
use std::{
    fs,
    path::{Path, PathBuf},
};
use crate::{daemon::utils::hash_file_list, extraction_utils::{
    convert_to_abs_path_buf, get_cargo_config, load_project_manifest, load_project_workspace, load_workspace_data
}};

use crate::daemon::utils::enumerate_rust_files;

// Public daemon state (kept alive until shutdown)

pub struct DaemonCore {
    /// The resolved manifest directory that contains Cargo.toml
    pub manifest_dir: PathBuf,
    /// Absolute path to Cargo.toml
    pub cargo_toml: PathBuf,

    /// Loaded once, kept and incrementally updated later
    pub cargo_config: CargoConfig,
    pub workspace: ProjectWorkspace,
    pub db: RootDatabase,
    pub vfs: Vfs,

    /// Current snapshot of all .rs files under the workspace (excludes target/)
    pub hashed_files: Vec<FileRepr>,
}

/// Representation of a file - the absolute path and a hash of its contents (for
/// change detection).
pub struct FileRepr {
    pub path: AbsPathBuf,
    pub hash: String,
}

// Entry point

/// Initialise the daemon given either:
/// - a path to `Cargo.toml`, OR
/// - a path to a `.rs` file somewhere inside a Cargo workspace.
///
/// On success, returns a fully-initialised daemon core with persistent RA state.
pub fn init_daemon(entry: &Path) -> Result<DaemonCore> {

    // 1) Resolve a manifest (Cargo.toml)
    let cargo_toml: PathBuf = resolve_manifest(entry)
        .with_context(|| format!("failed to resolve Cargo.toml from {:?}", entry))?;
    let manifest_dir = cargo_toml
        .parent()
        .ok_or_else(|| anyhow!("Cargo.toml has no parent directory"))?
        .to_path_buf();

    // 2) Load ProjectManifest
    let cargo_toml_abs: ra_ap_vfs::AbsPathBuf = convert_to_abs_path_buf(cargo_toml.to_str().unwrap()).unwrap();
    let project_manifest: ProjectManifest = load_project_manifest(&cargo_toml_abs);

    // 3) Load CargoConfig
    let cargo_config: CargoConfig = get_cargo_config(&project_manifest);

    // 4) Load ProjectWorkspace (deps, sysroot, crate graph inputs)
    let workspace: ProjectWorkspace = load_project_workspace(&project_manifest, &cargo_config);

    // 5) Load Analysis DB + VFS (long-lived incremental state)
    let (db, vfs) = load_workspace_data(workspace.clone(), &cargo_config);

    let files = enumerate_rust_files(&manifest_dir);
    let hashed_files = hash_file_list(files);

    Ok(DaemonCore {
        manifest_dir,
        cargo_toml,
        cargo_config,
        workspace,
        db,
        vfs,
        hashed_files,
    })
}

pub fn close_daemon(_core: DaemonCore) -> Result<()> {
    // Currently no special action needed to close the daemon.
    Ok(())
}

// Helpers

/// Accept either a Cargo.toml path or a .rs file and climb upwards to find one.
/// Returns the absolute path to Cargo.toml or a sensible error (for VSCode to retry).
fn resolve_manifest(entry: &Path) -> Result<PathBuf> {
    let entry = entry
        .canonicalize()
        .with_context(|| format!("canonicalize({:?}) failed", entry))?;

    if entry
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| s == "Cargo.toml")
        .unwrap_or(false)
    {
        // Explicit manifest path
        return Ok(entry);
    }

    // If it's a Rust source file, walk upward
    if entry
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.eq_ignore_ascii_case("rs"))
        .unwrap_or(false)
    {
        if let Some(c) = climb_to_manifest(entry.parent()) {
            return Ok(c);
        }
        return Err(anyhow!(
            "No Cargo.toml found by walking up from the provided .rs file. \
             Is this file outside a Cargo workspace?"
        ));
    }

    // Otherwise treat as a directory: try that directory then walk up
    let start_dir = if entry.is_dir() {
        entry.clone()
    } else {
        entry
            .parent()
            .ok_or_else(|| anyhow!("path has no parent: {:?}", entry))?
            .to_path_buf()
    };

    if let Some(c) = climb_to_manifest(Some(&start_dir)) {
        return Ok(c);
    }

    Err(anyhow!(
        "Failed to find Cargo.toml from the supplied path. \
         Provide a manifest path or a .rs file inside a Cargo project."
    ))
}

fn climb_to_manifest(mut dir_opt: Option<&Path>) -> Option<PathBuf> {
    while let Some(dir) = dir_opt {
        let candidate = dir.join("Cargo.toml");
        if candidate.is_file() {
            return Some(candidate);
        }
        dir_opt = dir.parent();
    }
    None
}