//! Utility functions for the rem-extract crate.
//! At some point these will be merged into rem-utils.

use crate::error::ExtractionError;

use std::{
    env,
    fs,
    path::PathBuf,
    io::ErrorKind,
};

use camino::Utf8PathBuf;

use ra_ap_project_model::{
    CargoConfig,
    ProjectWorkspace,
    ProjectManifest,
};

use ra_ap_ide::{
    Analysis,
    AnalysisHost,
    DiagnosticsConfig,
    FileRange,
    RootDatabase,
    SingleResolve,
    TextRange,
    TextSize,
    TextEdit,
    SnippetEdit,
    SourceChange,
};

use ra_ap_ide_db::{
    imports::insert_use::{
        ImportGranularity,
        InsertUseConfig,
        PrefixKind,
    },
    SnippetCap
};

use ra_ap_ide_assists::{
    Assist,
    AssistConfig,
    AssistKind,
    AssistResolveStrategy,
};

use ra_ap_vfs::{
    AbsPathBuf,
    VfsPath,
    Vfs,
    FileId,
};

use ra_ap_load_cargo::{
    LoadCargoConfig,
    ProcMacroServerChoice,
    load_workspace,
};

use ra_ap_parser::{
    T,
    SyntaxKind::COMMENT,
};

use ra_ap_syntax::{
    algo,
    AstNode,
    SourceFile
};


/// Returns the path to the manifest directory of the given file
/// The manifest directory is the directory containing the Cargo.toml file
/// for the project.
///
/// ## Example
/// Given a directory structure like:
/// ```plaintext
/// /path/to/project
/// ├── Cargo.toml
/// └── src
///    └── main.rs
/// ```
/// The manifest directory of `main.rs` is `/path/to/project`
pub fn get_manifest_dir( path: &PathBuf ) -> Result<PathBuf, ExtractionError> {
    // Start from the directory of the file
    let mut current_dir = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path.as_path()
    };

    // Check if the current directory is the root and contains a Cargo.toml file
    if fs::metadata(current_dir.join("Cargo.toml")).is_ok() {
        return Ok(current_dir.to_path_buf());
    }

    // Traverse up the directory tree until a Cargo.toml file is found
    while let Some(parent) = current_dir.parent() {
        if fs::metadata(current_dir.join("Cargo.toml")).is_ok() {
            return Ok(current_dir.to_path_buf());
        }
        current_dir = parent;
    }

    // Return an InvalidManifest error if no Cargo.toml file is found
    Err(ExtractionError::InvalidManifest)
}

/// Given an `&str` path to a file, returns the `AbsPathBuf` to the file.
/// The `AbsPathBuf` is used by the `ra_ap` crates to represent file paths.
/// If the input is not an absolute path, it resulves the path relative to the
/// current directory.
/// Will also canonicalize the path before returning it.
pub fn convert_to_abs_path_buf(path: &str) -> Result<AbsPathBuf, Utf8PathBuf> {
    if path.is_empty() {
        return Err(Utf8PathBuf::from_path_buf(PathBuf::new()).unwrap());
    }

    // Check if the path is valid for a file system
    if !path.is_ascii() {
        return Err(Utf8PathBuf::from_path_buf(PathBuf::new()).unwrap());
    }

    // Attempt to convert it as-is (absolute path).
    match AbsPathBuf::try_from(path) {
        Ok(abs_path_buf) => Ok(abs_path_buf),
        Err(_) => {
            // Resolve non-absolute path to the current working directory.
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let utf8_current_dir = Utf8PathBuf::from_path_buf(current_dir)
                .expect("Failed to convert current directory to Utf8PathBuf");

            // println!("Current dir: {:?}", utf8_current_dir);
            // println!("Current path: {:?}", path);
            let resolved_path = utf8_current_dir.join(path);

            // Normalize the path to eliminate unnecessary components
            let normalized_path = resolved_path.canonicalize().unwrap_or(resolved_path.clone().into());

            // Create directories leading to the resolved path if they don't exist
            if let Some(parent) = normalized_path.parent() {
                fs::create_dir_all(parent).expect("Failed to create directories");
            }

            // Attempt to convert the normalized path to AbsPathBuf
            let abs_path = AbsPathBuf::try_from(normalized_path.to_str().unwrap())
                .map_err(|e| e); // Return the error if the resolved path is still invalid
            // println!("Resolved path: {:?}", abs_path);

            // If the abs_path as a string starts with either a \ or a ? (or some
            // combination), strip it out

            let abs_path_str: String = abs_path.unwrap().to_string();
            let abs_path_str: String = abs_path_str
                .replace(r"\\?\", "");

            let new_abs_path = AbsPathBuf::try_from(abs_path_str.as_str())
                .map_err(|e| e);

            // println!("New abs path: {:?}", new_abs_path);
            new_abs_path
        }
    }
}

/// Given a `PathBuf` to a folder, returns the `AbsPathBuf` to the `Cargo.toml`
/// file in that folder.
pub fn get_cargo_toml( manifest_dir: &PathBuf ) -> AbsPathBuf {
    AbsPathBuf::try_from(
        manifest_dir
            .join( "Cargo.toml" )
            .to_str()
            .unwrap()
    ).unwrap()
}

/// Loads as `ProjectManifest` from the given `AbsPathBuf` to a `Cargo.toml` file.
pub fn load_project_manifest( cargo_toml: &AbsPathBuf ) -> ProjectManifest {
    ProjectManifest::from_manifest_file(
        cargo_toml.clone()
    ).unwrap()
}

/// Loads in the custom cargo configuration
/// TODO This is currently just the derived default.
pub fn get_cargo_config( _manifest: &ProjectManifest ) -> CargoConfig {
    CargoConfig::default()
}

pub fn progress( _message: String ) -> () {
    // println!( "{}", _message );
}

/// Loads a project workspace from a `ProjectManifest` and `CargoConfig`
pub fn load_project_workspace(
    project_manifest: &ProjectManifest,
    cargo_config: &CargoConfig,
) -> ProjectWorkspace {
    ProjectWorkspace::load(
        project_manifest.clone(),
        cargo_config,
        &progress
    ).unwrap()
}

/// Loads a `RootDatabase` containing from a `ProjectWorkspace` and `CargoConfig`
pub fn load_workspace_data(
    workspace: ProjectWorkspace,
    cargo_config: &CargoConfig,
) -> (
    RootDatabase,
    Vfs
) {
    let load_cargo_config: LoadCargoConfig = LoadCargoConfig {
        load_out_dirs_from_check: true,
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
    };

    let (db,
        vfs,
        _proc_macro
    ) = load_workspace(
        workspace,
        &cargo_config.extra_env,
        &load_cargo_config
    ).unwrap();

    (db, vfs)
}

/// Runs the analysis on an AnalysisHost. A wrapper around `AnalysisHost::analysis`
pub fn run_analysis( host: AnalysisHost ) -> Analysis {

    let analysis: Analysis = host.analysis();

    analysis
}

/// Verifies the input selection for extraction.
/// # Input
/// - analysis: The `&Analysis` object containing the analysis data
/// - vfs: The `&Vfs` object containing the virtual file system data
/// - input_path: The `&AbsPathBuf` to the input file
/// - range: The tuple of start and end offsets for the selection
///
/// # Returns
/// - Ok(()) if the input is valid
/// - Err(ExtractioError::CommentNotApplicable) if the selection is a comment
/// - Err(ExtractioError::BracesNotApplicable) if the selection is braces

/// Gets a list of available assists for a given file and range
pub fn get_assists (
    analysis: &Analysis,
    vfs: &Vfs,
    input_path: &AbsPathBuf,
    range: (u32, u32), // Tuple of start and end offsets
) -> Vec<Assist> {

    let assist_config: AssistConfig = generate_assist_config();
    let diagnostics_config: DiagnosticsConfig = generate_diagnostics_config();
    let resolve: AssistResolveStrategy = generate_resolve_strategy();
    let frange: FileRange = generate_frange(input_path, vfs, range);

    // Call the assists_with_fixes method
    let assists: Vec<Assist> = analysis.assists_with_fixes(
        &assist_config,
        &diagnostics_config,
        resolve,
        frange
    ).unwrap();

    assists
}

// Build out the AssistConfig Object
fn generate_assist_config() -> AssistConfig {
    let snippet_cap_: Option<SnippetCap> = None;
    let allowed_assists: Vec<AssistKind> = vec![
        // AssistKind::QuickFix,
        // AssistKind::Refactor,
        // AssistKind::RefactorInline,
        // AssistKind::RefactorRewrite,
        // AssistKind::Generate,
        AssistKind::RefactorExtract,
    ];

    let insert_use_: InsertUseConfig = InsertUseConfig {
        granularity: ImportGranularity::Preserve,
        enforce_granularity: false,
        prefix_kind: PrefixKind::ByCrate,
        group: false,
        skip_glob_imports: false,
    };

    let assist_config: AssistConfig = AssistConfig {
        snippet_cap: snippet_cap_,
        allowed: Some(allowed_assists),
        insert_use: insert_use_,
        prefer_no_std: false,
        prefer_prelude: false,
        prefer_absolute: false,
        assist_emit_must_use: false,
        term_search_fuel: 2048, // * NFI what this is
        term_search_borrowck: false,
    };
    assist_config
}

// Build out the DiagnosticsConfig
fn generate_diagnostics_config() -> DiagnosticsConfig {
    DiagnosticsConfig::test_sample()
}

// Build out the ResolveStrategy
fn generate_resolve_strategy() -> AssistResolveStrategy {
    // FIXME: This is currently bugged it seems - Both extract_variable and extract_function are being returned
    let single_resolve: SingleResolve = SingleResolve {
        assist_id: "extract_function".to_string(),
        assist_kind: AssistKind::RefactorExtract,
    };

    let resolve_strategy: AssistResolveStrategy = AssistResolveStrategy::Single(single_resolve);
    resolve_strategy
}

// Build out the FileRange object
pub fn generate_frange(
    input_path: &AbsPathBuf,
    vfs: &Vfs,
    range: (u32, u32)
) -> FileRange {
    let vfs_path: VfsPath = VfsPath::new_real_path(
        input_path
            .as_str()
            .to_string(),
    );

    let file_id_: FileId = vfs.file_id( &vfs_path ).unwrap();
    let range_: TextRange = TextRange::new(
        TextSize::try_from( range.0 ).unwrap(),
        TextSize::try_from( range.1 ).unwrap(),
    );

    let frange: FileRange = FileRange {
        file_id: file_id_,
        range: range_,
    };
    frange
}

/// Filter the list of assists to only be the extract_function assist
/// FIXME This is a hack to get around the fact that the resolve strategy is bugged
/// and is returning both extract_variable and extract_function
/// Throws ExtractionError::NoExtractFunction if no assist found
pub fn filter_extract_function_assist( assists: Vec<Assist> ) -> Result<Assist, ExtractionError> {
    if let Some(extract_assist) = assists
        .iter()
        .find(|assist| assist.label == "Extract into function")
        {
            // Return the found assist
            Ok(extract_assist.clone())
        } else {
            // Return the error
            Err(ExtractionError::NoExtractFunction( assists ))
        }
}

/// Applies the extract_function source change to the given code
/// Returns the String of the output code
/// Renames the function from `fun_name` to `callee_name`.
/// Requires the output path to be an `AbsPathBuf`.
pub fn apply_extract_function(
    assist: &Assist,
    input_path: &AbsPathBuf,
    vfs: &Vfs,
    callee_name: &str,
) -> Result<String, ExtractionError> {

    let vfs_in_path: VfsPath = VfsPath::new_real_path(
        input_path
            .as_str()
            .to_string(),
    );

    // From here, extract the source change, but apply it to the copied file
    let src_change: SourceChange = assist.source_change
        .as_ref()
        .unwrap()
        .clone();

    let in_file_id: FileId = vfs.file_id( &vfs_in_path ).unwrap();
    let (text_edit, maybe_snippet_edit) = src_change.get_source_and_snippet_edit(
        in_file_id
    ).unwrap();

    // Get the source from the input file
    let src_path: PathBuf = vfs_to_pathbuf( &vfs_in_path );
    let text: String = fs::read_to_string( &src_path ).unwrap();
    let edited_text: String = apply_edits(
        text,
        text_edit.clone(),
        maybe_snippet_edit.clone(),
    );

    // Rename the function from fun_name to NEW_FUNCTION_NAME using a search and
    // replace on the output file
    let renamed_text: String = rename_function(
        edited_text,
        "fun_name",
        callee_name,
    );

    // Ensure that the output file imports std::ops::ControlFlow if it uses it
    let fixed_cf_text: String = fixup_controlflow( renamed_text );

    Ok( fixed_cf_text )
}

/// Applies the edits to a given set of source code (as a String)
fn apply_edits(
    text: String,
    text_edit: TextEdit,
    maybe_snippet_edit: Option<SnippetEdit>,
) -> String {
    let mut text: String = text; // Make the text mutable
    text_edit.apply( &mut text );
    match maybe_snippet_edit {
        Some( snippet_edit ) => {
            snippet_edit.apply( &mut text );
        },
        None => (),
    }
    text
}

// Rename a function in a file using a search and replace
fn rename_function(
    text: String,
    old_name: &str,
    new_name: &str,
) -> String {
    let mut text = text; // Make the text mutable
    let old_name: String = old_name.to_string();
    let new_name: String = new_name.to_string();
    text = text.replace( &old_name, &new_name );
    text
}

/// Converts a `VfsPath` to a `PathBuf`
fn vfs_to_pathbuf( vfs_path: &VfsPath ) -> PathBuf {
    let path_str = vfs_path.to_string();
    // println!("{}", path_str);
    PathBuf::from( path_str )
}

/// Checks that there is some input to the function that isn't a comment
/// # Returns
/// - `Ok(())` if the input is not a comment
/// - `Err(ExtractionError::CommentNotApplicable)` if the input is a comment
pub fn check_comment(
    source_file: &SourceFile,
    range: &(u32, u32)
) -> Result<(), ExtractionError> {
    let frange: TextRange = TextRange::new(
        TextSize::new(range.0),
        TextSize::new(range.1),
    );
    let node = source_file
        .syntax()
        .covering_element(frange);

    if node.kind() == COMMENT {
        return Err(ExtractionError::CommentNotApplicable);
    }

    Ok(())
}

/// Checks that there is some input to the function that isn't a brace
/// For every:
/// - { there is a }
/// - [ there is a ]
/// - ( there is a )
/// # Returns
/// - `Ok(())` if the input is not a brace
/// - `Err(ExtractionError::BracesNotApplicable)` if the input is a brace
pub fn check_braces(
    source_file: &SourceFile,
    range: &(u32, u32)
) -> Result<(), ExtractionError> {
    let frange: TextRange = TextRange::new(
        TextSize::new(range.0),
        TextSize::new(range.1),
    );
    let node = source_file
        .syntax()
        .covering_element(frange);

    if matches!(node.kind(), T!['{'] | T!['}'] | T!['('] | T![')'] | T!['['] | T![']']) {
        return Err(ExtractionError::BracesNotApplicable);
    }

    Ok(())

}

/// Trims the selected range to remove any whitespace
pub fn trim_range(
    source_file: &SourceFile,
    range: &(u32, u32)
) -> (u32, u32) {
    let start = TextSize::new(range.0);
    let end = TextSize::new(range.1);
    let left = source_file
        .syntax()
        .token_at_offset( start )
        .right_biased()
        .and_then(|t| algo::skip_whitespace_token(t, rowan::Direction::Next))
        .map(|t| t.text_range().start().clamp(start, end));
    let right = source_file
        .syntax()
        .token_at_offset( end )
        .left_biased()
        .and_then(|t| algo::skip_whitespace_token(t, rowan::Direction::Prev))
        .map(|t| t.text_range().end().clamp(start, end));

    let trimmed_range = match (left, right) {
        (Some(left), Some(right)) if left <= right => TextRange::new(left, right),
        // Selection solely consists of whitespace so just fall back to the original
        _ => TextRange::new(start, end),
    };

    ( trimmed_range.start().into(), trimmed_range.end().into() )

}

/// Checks if a file contains a reference to ControlFlow::, and if so, adds  use
/// std::ops::ControlFlow;\n\n to the start of the file, saving it back to the input path
/// Returns the new text with the ControlFlow:: reference fixed up
fn fixup_controlflow( text: String, ) -> String {
    let mut text: String = text; // Make the text mutable
    let controlflow_ref: &str = "ControlFlow::";
    if text.contains( controlflow_ref ) {
        text = format!("use std::ops::ControlFlow;\n\n{}", text);
    }
    text
}

// Check if the idx pair is valid
pub fn check_idx(start: &u32, end: &u32) -> Result<(), ExtractionError> {
    let start = *start;
    let end = *end;
    if start == end {
        return Err(ExtractionError::SameIdx);
    } else if start > end {
        return Err(ExtractionError::InvalidIdxPair);
    }
    if end == 0 {
        return Err(ExtractionError::InvalidEndIdx);
    }
    Ok(())
}

// Check if the file exists and is readable
pub fn check_file_exists(file_path: &str) -> Result<(), ExtractionError> {
    if fs::metadata(file_path).is_err() {
        return Err(ExtractionError::Io(std::io::Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::env;
    use camino::Utf8Path;

    // Helper function to create a temporary directory with a Cargo.toml
    fn setup_temp_project() -> PathBuf {
        let temp_dir = env::temp_dir().join("test_project");
        let _ = fs::create_dir_all(&temp_dir);
        let cargo_toml = temp_dir.join("Cargo.toml");

        let mut file = File::create(cargo_toml).unwrap();
        writeln!(file, "[package]\nname = \"test_project\"\nversion = \"0.1.0\"").unwrap();

        temp_dir
    }

    // Test case when Cargo.toml exists
    #[test]
    fn test_get_manifest_dir_valid() {
        let temp_dir = setup_temp_project();
        let src_dir = temp_dir.join("src");
        let _ = fs::create_dir_all(&src_dir);
        let main_file = src_dir.join("main.rs");
        File::create(&main_file).unwrap();

        let result = get_manifest_dir(&main_file);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir);
    }

    // Test case when Cargo.toml does not exist
    #[test]
    fn test_get_manifest_dir_invalid_manifest() {
        let temp_dir = env::temp_dir().join("test_invalid_project");
        let _ = fs::create_dir_all(&temp_dir);
        let src_dir = temp_dir.join("src");
        let _ = fs::create_dir_all(&src_dir);
        let main_file = src_dir.join("main.rs");
        File::create(&main_file).unwrap();

        let result = get_manifest_dir(&main_file);
        assert!(result.is_err());

        // Check that the error is an InvalidManifest
        if let ExtractionError::InvalidManifest = result.unwrap_err() {
            // Correct error type
        } else {
            panic!("Expected InvalidManifest error");
        }
    }

    // Test case when the path is to a directory, not a file
    #[test]
    fn test_get_manifest_dir_directory() {
        let temp_dir = setup_temp_project();
        let src_dir = temp_dir.join("src");
        let _ = fs::create_dir_all(&src_dir);

        let result = get_manifest_dir(&src_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), temp_dir);
    }

    // Test case when the path points to a non-existent file
    #[test]
    fn test_get_manifest_dir_non_existent_file() {
        let temp_dir = env::temp_dir().join("test_non_existent_project");
        let src_dir = temp_dir.join("src");

        let non_existent_file = src_dir.join("does_not_exist.rs");
        let result = get_manifest_dir(&non_existent_file);
        assert!(result.is_err());

        // Check that the error is an InvalidManifest
        if let ExtractionError::InvalidManifest = result.unwrap_err() {
            // Correct error type
        } else {
            panic!("Expected InvalidManifest error");
        }
    }

    #[test]
    ///Only run this test on Windows as it tests Windows-specific paths
    /// This test is skipped on other platforms
    #[cfg(target_os = "windows")]
    fn test_absolute_path_windows() {
        // Test with an absolute path (Windows-style)
        let abs_path = r"C:\Windows\System32";
        let result = convert_to_abs_path_buf(abs_path);
        assert!(result.is_ok(), "Expected absolute path conversion to succeed");

        // Check if the path remains unchanged
        let abs_path_buf = result.unwrap();
        assert_eq!(<AbsPathBuf as AsRef<Utf8Path>>::as_ref(&abs_path_buf), Utf8Path::new(abs_path));
    }

    #[test]
    ///Only run this test on Windows as it tests Windows-specific paths
    /// This test is skipped on other platforms
    #[cfg(target_os = "windows")]
    fn test_relative_path_windows() {
        // Test with a relative path (Windows-style)
        let rel_path = r"src\main.rs";
        let result = convert_to_abs_path_buf(rel_path);
        assert!(result.is_ok(), "Expected relative path conversion to succeed");

        // Check if the relative path is resolved to an absolute path
        let current_dir = env::current_dir().unwrap();
        let expected_abs_path = Utf8PathBuf::from_path_buf(current_dir).unwrap().join(rel_path);
        let abs_path_buf = result.unwrap();

        // Compare the canonicalized paths
        let left_path = <AbsPathBuf as AsRef<Utf8Path>>::as_ref(&abs_path_buf).to_string().replace(r"\\?\", "");
        let right_path = expected_abs_path.as_path().to_string().replace(r"\\?\", "");
        assert_eq!(left_path, right_path);
    }

    #[test]
    ///Only run this test on Windows as it tests Windows-specific paths
    /// This test is skipped on other platforms
    #[cfg(target_os = "windows")]
    fn test_invalid_utf8_path_windows() {
        // Test with a path that cannot be converted to a valid UTF-8 path
        let invalid_utf8_path = r"C:\invalid\�path";
        let result = convert_to_abs_path_buf(invalid_utf8_path);
        assert!(result.is_err(), "Expected invalid UTF-8 path to fail conversion");
    }

    #[test]
    fn test_empty_path_windows() {
        // Test with an empty path
        let empty_path = "";
        let result = convert_to_abs_path_buf(empty_path);
        assert!(result.is_err(), "Expected empty path to fail conversion");
    }

    #[test]
    fn test_root_path_windows() {
        // Test with a root path (Windows-style)
        let root_path = r"C:\";
        let result = convert_to_abs_path_buf(root_path);
        assert!(result.is_ok(), "Expected root path conversion to succeed");

        let abs_path_buf = result.unwrap();
        assert_eq!(<AbsPathBuf as AsRef<Utf8Path>>::as_ref(&abs_path_buf), Utf8Path::new(root_path));
    }

    #[test]
    fn test_resolve_relative_path_windows() {
        // Test with a complex relative path (Windows-style)
        let complex_rel_path = r"src\..\Cargo.toml";
        let result = convert_to_abs_path_buf(complex_rel_path);
        assert!(result.is_ok(), "Expected complex relative path conversion to succeed");

        // Check if the relative path is resolved correctly
        let current_dir = env::current_dir().unwrap();
        let expected_abs_path = Utf8PathBuf::from_path_buf(current_dir)
            .unwrap()
            .join(complex_rel_path)
            .canonicalize_utf8()
            .unwrap();
        let abs_path_buf = result.unwrap();

        // Compare the canonicalized paths
        let left_path = <AbsPathBuf as AsRef<Utf8Path>>::as_ref(&abs_path_buf).to_string().replace(r"\\?\", "");
        let right_path = expected_abs_path.as_path().to_string().replace(r"\\?\", "");
        assert_eq!( left_path, right_path );

    }
}
