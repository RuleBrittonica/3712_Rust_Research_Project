use std::path::PathBuf;

use anyhow::Result;
use anyhow::anyhow;
use serde_json::json;

use ra_ap_ide::{AnalysisHost, RootDatabase, Semantics};
use ra_ap_syntax::SourceFile;
use ra_ap_hir::FileRangeWrapper;
use ra_ap_vfs::{AbsPathBuf, FileId, Vfs};
use ra_ap_ide_assists::Assist;
use ra_ap_ide_db::EditionedFileId;


use rem_extract::extract::extraction_utils::{
    apply_extract_function, check_braces, check_comment, check_file_exists, check_idx, convert_to_abs_path_buf, filter_extract_function_assist, generate_frange, get_assists, trim_range
};

use rem_extract::extract::extraction::{
    parent_method,
};

use crate::stdio::Response;
use crate::stdio::JsonResp;

/// Thin stdio wrapper: uses persistent AnalysisHost & Workspace & Vfs.
/// Returns `{ output, callsite }` on success.
pub fn handle_extract(
    host: &AnalysisHost,
    vfs: &Vfs,
    file: PathBuf,
    new_fn_name: String,
    start: u32,
    end: u32,
) -> JsonResp {
    match extract_via_host(
        host,
        vfs,
        &file,
        &new_fn_name,
        start,
        end
    ) {
        Ok((modified_code, caller_method)) => {
            JsonResp::ok(json!({ "output": modified_code, "callsite": caller_method }))
        }
        Err(e) => JsonResp::err(format!("{e:#}")),
    }
}

/// Core extraction pipeline using the already-initialized host/vfs.
pub(crate) fn extract_via_host(
    host: &AnalysisHost,
    vfs: &Vfs,
    file: &PathBuf,
    new_fn_name: &str,
    start_idx: u32,
    end_idx: u32,
) -> Result<(String, String)> {
    // Inputs / basic conversions
    let input_abs_path: AbsPathBuf = convert_to_abs_path_buf(file.to_str().unwrap())
        .map_err(|bad| anyhow!("failed to canonicalize file path: {}", bad))?;

    // Parse the cursor positions into the range
    let range_: (u32, u32) = (start_idx, end_idx);

    // Semantics & source retrieval
    let db: &RootDatabase = host.raw_database(); // &RootDatabase
    let sema: Semantics<'_, ra_ap_ide::RootDatabase> = Semantics::new(db);

    let frange: FileRangeWrapper<FileId> = generate_frange(&input_abs_path, vfs, range_.clone());
    let edition: EditionedFileId = EditionedFileId::current_edition(frange.file_id);
    let source_file: SourceFile = sema.parse(edition);

    // Pre-checks
    let range: (u32, u32) = trim_range(&source_file, &range_);
    check_file_exists(input_abs_path.as_str())?;
    check_idx(&start_idx, &end_idx)?;
    check_comment(&source_file, &range)?;
    check_braces(&source_file, &range)?;

    // Analysis & assists
    let analysis = host.analysis();

    let assists: Vec<Assist> = get_assists(&analysis, vfs, &input_abs_path, range);
    let assist: Assist = filter_extract_function_assist(assists)?;

    let callee_name= new_fn_name;

    // Apply assist to produce new code + callsite/caller text
    let modified_code: String = apply_extract_function(
        &assist,
        &input_abs_path,
        vfs,
        callee_name,
    )?;

    let parent_method_str: String = parent_method(&source_file, range)?;

    Ok((modified_code, parent_method_str))
}
