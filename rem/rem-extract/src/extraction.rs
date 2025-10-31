use std::{
    fs,
    io::{
        self,
        ErrorKind
    },
    path::PathBuf
};

use ra_ap_ide_db::EditionedFileId;
use ra_ap_project_model::{
    CargoConfig,
    ProjectWorkspace,
    ProjectManifest,
};

use ra_ap_ide::{
    Analysis,
    AnalysisHost,
    TextSize,
};

use ra_ap_syntax::{
    algo, ast::HasName, AstNode, SourceFile
};

use ra_ap_hir::Semantics;

use ra_ap_ide_assists::Assist;

use ra_ap_vfs::AbsPathBuf;

use crate::{
    error::ExtractionError,
    extraction_utils::{
        apply_extract_function,
        convert_to_abs_path_buf,
        filter_extract_function_assist,
        get_assists,
        get_cargo_config,
        get_cargo_toml,
        get_manifest_dir,
        load_project_manifest,
        load_project_workspace,
        load_workspace_data,
        run_analysis,
        check_braces,
        check_comment,
        trim_range,
        generate_frange,
    },
};

use rem_interface::metrics as mx;

#[derive(Debug, PartialEq, Clone)]
pub struct ExtractionInput {
    pub file_path: String,
    pub new_fn_name: String,
    pub start_idx: u32,
    pub end_idx: u32,
}

impl ExtractionInput {
    pub fn new(
        file_path: &str,
        new_fn_name: &str,
        start_idx: u32,
        end_idx: u32,
    ) -> Self { ExtractionInput {
            file_path: file_path.to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_idx,
            end_idx,
        }
    }

    #[allow(dead_code)]
    pub fn new_absolute(
        file_path: &str,
        new_fn_name: &str,
        start_idx: u32,
        end_idx: u32,
    ) -> Self { ExtractionInput {
            file_path: convert_to_abs_path_buf(file_path).unwrap().as_str().to_string(),
            new_fn_name: new_fn_name.to_string(),
            start_idx,
            end_idx,
        }
    }
}

// ========================================
// Checks for the validity of the input
// ========================================

// Check if the file exists and is readable
fn check_file_exists(file_path: &str) -> Result<(), ExtractionError> {
    if fs::metadata(file_path).is_err() {
        return Err(ExtractionError::Io(io::Error::new(
            ErrorKind::NotFound,
            format!("File not found: {}", file_path),
        )));
    }
    Ok(())
}

// Check if the idx pair is valid
fn check_idx(input: &ExtractionInput) -> Result<(), ExtractionError> {
    if input.start_idx == input.end_idx {
        return Err(ExtractionError::SameIdx);
    } else if input.start_idx > input.end_idx {
        return Err(ExtractionError::InvalidIdxPair);
    }
    if input.start_idx == 0 {
        return Err(ExtractionError::InvalidStartIdx);
    }
    if input.end_idx == 0 {
        return Err(ExtractionError::InvalidEndIdx);
    }
    Ok(())
}

fn verify_input(input: &ExtractionInput) -> Result<(), ExtractionError> {
    // Execute each input validation step one by one
    check_file_exists(&input.file_path)?;
    check_idx(input)?;

    Ok(())
}

// ========================================
// Performs the method extraction
// ========================================

/// Function to extract the code segment based on cursor positions
/// If successful, returns the `String` of the output code, followed by a
/// `String` of the caller method
pub fn extract_method(input: ExtractionInput) -> Result<(String, String), ExtractionError> {

    mx::mark("Extraction Start");

    // Extract the struct information
    let input_path: &str = &input.file_path;
    let callee_name: &str = &input.new_fn_name;
    let start_idx: u32 = input.start_idx;
    let end_idx: u32 = input.end_idx;

    // Convert the input and output path to an `AbsPathBuf`
    let input_abs_path: AbsPathBuf = convert_to_abs_path_buf(input_path).unwrap();

    // Verify the input data
    verify_input(&input)?;

    let manifest_dir: PathBuf = get_manifest_dir(
        &PathBuf::from(input_abs_path.as_str())
    )?;
    let cargo_toml: AbsPathBuf = get_cargo_toml( &manifest_dir );
    // println!("Cargo.toml {:?}", cargo_toml);

    mx::mark("Load the project workspace");

    let project_manifest: ProjectManifest = load_project_manifest( &cargo_toml );
    // println!("Project Manifest {:?}", project_manifest);

    // MARKER: Load the cargo config
    mx::mark("Load the cargo config");

    let cargo_config: CargoConfig = get_cargo_config( &project_manifest );
    // println!("Cargo Config {:?}", cargo_config);

    // MARKER: Load the project workspace
    mx::mark("Load the project workspace");

    let workspace: ProjectWorkspace = load_project_workspace( &project_manifest, &cargo_config );
    // println!("Project Workspace {:?}", workspace);

    // MARKER: Load the analysis database and VFS
    mx::mark("Load the analysis database and VFS");

    let (db, vfs) = load_workspace_data(workspace, &cargo_config);

    // Parse the cursor positions into the range
    let range_: (u32, u32) = (
        start_idx,
        end_idx,
    );

    // MARKER: Database Loaded
    mx::mark("Database Loaded");

    // Before we go too far, lets do few more quick checks now that we have the
    // analysis
    // 1. Check if the function to extract is not just a comment
    // 2. Check if the function to extract has matching braces
    // 3. Convert the range to a trimmed range.
    let sema: Semantics<'_, ra_ap_ide::RootDatabase> = Semantics::new( &db );
    let frange_: ra_ap_hir::FileRangeWrapper<ra_ap_vfs::FileId> = generate_frange( &input_abs_path, &vfs, range_.clone() );
    let edition: EditionedFileId = EditionedFileId::current_edition( frange_.file_id );
    let source_file: SourceFile = sema.parse( edition );
    let range: (u32, u32) = trim_range( &source_file, &range_ );
    check_comment( &source_file, &range )?;
    check_braces( &source_file, &range )?;

    // MARKER: Run the analysis
    mx::mark("Run the analysis");

    let analysis_host: AnalysisHost = AnalysisHost::with_database( db );
    let analysis: Analysis = run_analysis( analysis_host );

    // MARKER: Get the assists and filter for extract function assist
    mx::mark("Get the assists and filter for extract function assist");

    let assists: Vec<Assist> = get_assists( &analysis, &vfs, &input_abs_path, range );
    let assist: Assist = filter_extract_function_assist( assists )?;

    let modified_code: String = apply_extract_function(
        &assist,
        &input_abs_path,
        &vfs,
        &callee_name,
    )?;

    let parent_method: String = parent_method(
        &source_file,
        range,
    )?;

    // MARKER: Extraction End
    mx::mark("Extraction End");

    Ok( (modified_code, parent_method) )
}

/// Gets the caller method, based on the input code and the cursor positions
/// If successful, returns the `String` of the caller method
/// If unsuccessful, returns an `ExtractionError`
pub fn parent_method(
    source_file: &SourceFile,
    range: (u32, u32),
) -> Result<String, ExtractionError> {
    let start: TextSize = TextSize::new(range.0);

    // We want the last function that occurs before the start of the range
    let node: Option<ra_ap_syntax::ast::Fn> = algo::find_node_at_offset::<ra_ap_syntax::ast::Fn>(
        source_file.syntax(),
        start,
    );

    let fn_name: String = match node {
        Some(n) => n.name().map_or("".to_string(), |name| name.text().to_string()),
        None => "".to_string(),
    };

    if fn_name.is_empty() {
        return Err(ExtractionError::ParentMethodNotFound);
    }

    Ok( fn_name.trim().to_string() )

}