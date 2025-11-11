use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use crate::exports::{ProgramPaths, VerificationError};
use crate::exports::utils::{run_command, CmdOutput};
use crate::exports::create_tempdir::SharedTempDir;

/// TODO work out how to use the --include flag to limit to the specific crate
/// we want to verify, rather than all workspace members.
/// Apparently this is rought at the moment so we may need to filter later.

/// Run Charon on a crate, writing outputs to `out_dir`.
/// `extra_args` lets you pass through additional Charon flags (e.g., verbosity).
pub fn run_charon_on_crate(
    programs: &ProgramPaths,           // VSCode-supplied path (absolute or just "charon")
    manifest_path: &Path,              // the crate's Cargo.toml
    out_root: &SharedTempDir,          // shared temp workspace (kept alive by caller)
    run_name: &str,                    // e.g., "original" or "refactored"
    extra_args: &[impl AsRef<OsStr>],  // passthrough flags (optional)
) -> Result<(PathBuf /* llbc_dir */, CmdOutput), VerificationError> {
    // charon dir should exist already
    let llbc_out_dir: PathBuf = out_root.path().join("llbc");
    if !llbc_out_dir.exists() {
        return Err(VerificationError { msg: format!("Charon output dir does not exist: {}", llbc_out_dir.display()) });
    }

    let llbc_out_path = llbc_out_dir
        .join(run_name)
        .with_extension("llbc");

    let mut args: Vec<OsString> = Vec::new();
    args.push("cargo".into());
    args.push("--preset=aeneas".into());
    args.push("--dest-file".into());
    args.push(llbc_out_path.as_os_str().to_os_string());
    // args.push("--skip-borrowck".into());

    for a in extra_args {
        args.push(a.as_ref().to_os_string());
    }

    // Run *from* the crate root
    let crate_root = manifest_path.parent().ok_or_else(|| VerificationError {
        msg: format!("Manifest has no parent: {}", manifest_path.display()),
    })?;
    let output = run_command(
        programs.charon.as_os_str(),
        &args,
        Some(crate_root),
        None,
        &[],
    )?;

    if !output.status.success() {
        let tail = output.stderr.lines().rev().take(30).collect::<Vec<_>>()
            .into_iter().rev().collect::<Vec<_>>().join("\n");
        return Err(VerificationError {
            msg: format!(
                "Charon failed (exit={}):\n{}\n\
                Charon was run using: {}",
                output.status.code().unwrap_or(-1), tail,
                output.command
            ),
        });
    }

    Ok((llbc_out_path, output))


}