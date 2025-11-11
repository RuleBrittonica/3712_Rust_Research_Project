use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;

use crate::exports::{ProgramPaths, VerificationError};
use crate::exports::utils::{run_command, CmdOutput};
use crate::exports::create_tempdir::SharedTempDir;

/// Convert a single LLBC file to Coq using Aeneas, writing into:
///   <out_root>/coq/<run_name>/
/// Returns the full path to the generated .v file.
pub fn run_aeneas_llbc_to_coq(
    programs: &ProgramPaths,           // VSCode-provided path to aeneas (absolute or on PATH)
    llbc_path: &Path,                  // path to the single .llbc(.json) file
    out_root: &SharedTempDir,          // shared temp workspace
    extra_args: &[impl AsRef<OsStr>],  // passthrough flags (optional)
) -> Result<(PathBuf /* coq_file */, CmdOutput), VerificationError> {
    // Sanity + canonicalize
    if !llbc_path.is_file() {
        return Err(VerificationError {
            msg: format!("LLBC file does not exist: {}", llbc_path.display()),
        });
    }
    let llbc_abs = llbc_path.canonicalize().map_err(|e| VerificationError {
        msg: format!("Failed to canonicalize LLBC path {}: {}", llbc_path.display(), e),
    })?;

    // output dir should exist already (it is temp/coq)
    let coq_out_dir: PathBuf = out_root.path().join("coq");
    if !coq_out_dir.exists() {
        return Err(VerificationError { msg: format!("Aeneas output dir does not exist: {}", coq_out_dir.display()) });
    }

    let mut args: Vec<OsString> = Vec::new();
    args.push("-backend".into());
    args.push("coq".into());
    args.push(llbc_abs.as_os_str().to_owned());
    args.push("-dest".into());
    args.push(coq_out_dir.as_os_str().to_owned());
    args.push("-abort-on-error".into());
    args.push("-soft-warnings".into());
    for a in extra_args {
        args.push(a.as_ref().to_owned());
    }

    // Run from the coq_dir (not strictly required, but nice & local)
    let output = run_command(
        programs.aeneas.as_os_str(),
        &args,
        Some(&coq_out_dir),
        None,
        &[],
    )?;

    if !output.status.success() {
        let tail = output.stderr.lines().rev().take(30).collect::<Vec<_>>()
            .into_iter().rev().collect::<Vec<_>>().join("\n");
        let tail2 = output.stdout.lines().rev().take(30).collect::<Vec<_>>()
            .into_iter().rev().collect::<Vec<_>>().join("\n");
        return Err(VerificationError {
            msg: format!(
                "Aeneas failed converting {} (exit={}):\n{}\n{}",
                llbc_abs.display(),
                output.status.code().unwrap_or(-1),
                tail, tail2
            ),
        });
    }

    // Compute the expected Coq filename (as aeneas generates it but won't let
    // us specify it directly).
    let stem = llbc_abs
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| VerificationError {
            msg: format!("Could not read file stem for {}", llbc_abs.display()),
    })?;

    let coq_name = convert_to_coq_filename(stem);
    let coq_path = coq_out_dir.join(coq_name).with_extension("v");

    // Best-effort existence check (Aeneas should have created it)
    if !coq_path.is_file() {
        return Err(VerificationError {
            msg: format!(
                "Aeneas reported success but Coq file not found: {}",
                coq_path.display()
            ),
        });
    }

    Ok((coq_path, output))

}

/// Convert a file stem from the llbc form (e.g. "main_ref")
/// to the Coq form (e.g. "MainRef").
fn convert_to_coq_filename(file_stem: &str) -> String {
    file_stem
        .split('_')
        .map(|s| {
            let mut chars = s.chars();
            // Capitalize the first letter and append the rest
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}