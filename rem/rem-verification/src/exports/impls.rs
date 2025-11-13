use std::ffi::OsStr;
use std::path::PathBuf;

use crate::verify::coq_verification;

use super::utils::*;
use super::create_crates::*;
use super::create_tempdir::*;
use super::run_charon::*;
use super::run_aeneas::*;

#[derive(Debug)]
pub struct VerificationError {
    pub msg: String,
}

impl std::fmt::Display for VerificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VerificationError: {}", self.msg)
    }
}

impl std::error::Error for VerificationError {}

impl From<Box<dyn std::error::Error>> for VerificationError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        VerificationError {
            msg: err.to_string(),
        }
    }
}


// We don't necessarily have a success if we get a VerificationReturn
#[derive(Debug)]
pub struct VerificationReturn {
    pub success: bool,
}

#[derive(Debug)]
pub struct ProgramPaths {
    pub charon: PathBuf,
    pub aeneas: PathBuf,
}

impl std::fmt::Display for ProgramPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Charon Path: {}, Aeneas Path: {}", self.charon.display(), self.aeneas.display())
    }
}

impl ProgramPaths {
    #[allow(dead_code)]
    pub fn new(charon: PathBuf, aeneas: PathBuf) -> Self {
        Self { charon, aeneas }
    }

    #[allow(dead_code)]
    pub fn new_from_directory(base_dir: &PathBuf) -> Result<Self, VerificationError> {
        let charon = base_dir.join("charon");
        let aeneas = base_dir.join("aeneas");

        // Ensure the paths exist?
        if !charon.exists() {
            return Err(VerificationError { msg: format!("Charon path does not exist: {}", charon.display()) });
        }
        if !aeneas.exists() {
            return Err(VerificationError { msg: format!("Aeneas path does not exist: {}", aeneas.display()) });
        }

        Ok( Self { charon, aeneas } )
    }
}


#[derive(Debug)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}

impl FileContent {
    #[allow(dead_code)]
    pub fn new(path: PathBuf, content: String) -> Self {
        Self { path, content }
    }

    #[allow(dead_code)]
    pub fn from_path(path: PathBuf) -> Result<Self, VerificationError> {
        let content = std::fs::read_to_string(&path)
            .map_err(|e| VerificationError { msg: format!("Failed to read file {}: {}", path.display(), e) })?;
        Ok(Self { path, content })
    }
}

#[derive(Debug)]
pub struct VerificationInput {
    original_file: FileContent,
    refactored_file: FileContent,
    fn_name: String,
    programs: ProgramPaths,
}

impl VerificationInput {
    pub fn new( original_file: FileContent, refactored_file: FileContent, fn_name: String, programs: ProgramPaths ) -> Self {
        Self {
            original_file,
            refactored_file,
            fn_name,
            programs,
        }
    }
}

pub fn call_verifier(
    input: VerificationInput,
) -> Result<VerificationReturn, VerificationError> {

    // We will need to gather the following logic from around this crate:
    // 1) Create the two virtual crates we are operating on - work out what the
    //    crate is based on the file paths (the paths should be the same its
    //    just the content that is different) - but we need to create two "fake"
    //    crates that are as minimal as possible
    // 2) Invoke charon on both crates to get the llbc representation
    // 3) Invoke aeneas on the two llbc files to get the two coq files
    // 4) Build out the coq project and proof scripts to verify equivalence of
    //    the two functions
    // 5) Run coq to check the proof
    // 6) Return success/failure based on the proof result

    // Get the minmal crate for the path
    let input_path = &input.original_file.path;
    let manifest_path = find_manifest_for(input_path)
        .ok_or_else(|| VerificationError {
            msg: format!(
                "Could not find Cargo.toml for path: {}",
                input_path.display()
            ),
        })?;

    // Now that we know what the crate is, we can create the virtual crates
    // with the modified file contents. One will be an exact copy of the
    // original crate, the other will have the refactored file content swapped
    // in.
    // We do need physical copies of these crates on disk for charon to work
    // with them, so we will create temporary directories for them, and copy
    // them over.
    let refactored_content = &input.refactored_file.content;
    let twin_crates: TwinCrates = TwinCrates::new(
        &manifest_path,
        &input.original_file.path,
        refactored_content,
    )?;

    // create a shared temp dir for all the verification artifacts
    let temp_dir: SharedTempDir = SharedTempDir::new("rem_verification")?;
    let _llbc_dir: PathBuf = temp_dir.subdir("llbc")?;
    let coq_dir: PathBuf = temp_dir.subdir("coq")?;

    let empty_args: [&OsStr; 0] = [];

    // invoke charon on both crates to get the llbc files
    let (orig_llbc_path, _o1) = run_charon_on_crate(
        &input.programs,
        &twin_crates.original.manifest,
        &temp_dir,
        "original",
        &empty_args,
    )?;
    let (ref_llbc_path, _o2) = run_charon_on_crate(
        &input.programs,
        &twin_crates.refactored.manifest,
        &temp_dir,
        "refactored",
        &empty_args,
    )?;

    // invoke aeneas on both llbc files to get the coq files
    let (orig_v_path, _a1) = run_aeneas_llbc_to_coq(
        &input.programs,
        &orig_llbc_path,
        &temp_dir,
        &empty_args
    )?;

    let (ref_v_path, _a2) = run_aeneas_llbc_to_coq(
        &input.programs,
        &ref_llbc_path,
        &temp_dir,
        &empty_args
    )?;

    // ensure we have a Primitives.v file in the coq dir as well
    ensure_primitives_file(&coq_dir)?;

    // now run the coq verification (this also generates the coq project and
    // equivcheck files)
    let (_coq_project, _equivcheck, _primitives, success) = coq_verification(
        &orig_v_path,
        &ref_v_path,
        &input.fn_name,
    )?;

    // Ensure the crates are alive until the end of the function. The crates
    // will be `dropped` at the end of their scope.
    let _ = twin_crates;
    let _ = temp_dir;

    Ok( VerificationReturn { success } )
}