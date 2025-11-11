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
    Analysis, AssistConfig, AssistResolveStrategy, TextSize
};

use ra_ap_syntax::{
    algo, ast::HasName, AstNode, SourceFile
};

use ra_ap_hir::Semantics;

use ra_ap_ide_assists::Assist;

use ra_ap_vfs::AbsPathBuf;

use crate::{
    error::ExtractionError,
    extract::extraction_utils::{
        apply_edits, apply_extract_function, check_braces, check_comment, convert_to_abs_path_buf, filter_extract_function_assist, fixup_controlflow, generate_frange, generate_frange_from_fileid, get_assists, get_cargo_config, get_cargo_toml, get_manifest_dir, load_project_manifest, load_project_workspace, load_workspace_data, rename_function, trim_range
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

pub fn extract_method_file(input: ExtractionInput) -> Result<(String, String), ExtractionError> {
    mx::mark("Extraction Start");

    // Extract the struct information
    let input_path: &str = &input.file_path;
    let callee_name: &str = &input.new_fn_name;
    let start_idx: u32 = input.start_idx;
    let end_idx: u32 = input.end_idx;

    let text: String = fs::read_to_string(&input.file_path).unwrap();

    // Verify the input data
    verify_input(&input)?;

    mx::mark("Load the analysis");

    let (analysis,file_id) = Analysis::from_single_file(text.clone());

    mx::mark("Analysis Loaded");

    let assist_config: AssistConfig = super::extraction_utils::generate_assist_config();
    let diagnostics_config = super::extraction_utils::generate_diagnostics_config();
    let resolve: AssistResolveStrategy = super::extraction_utils::generate_resolve_strategy();
    let range: (u32, u32) = (start_idx, end_idx);

    let frange = generate_frange_from_fileid(file_id, range);

    mx::mark("Get the assists");

    let assists: Vec<Assist> = analysis.assists_with_fixes(
        &assist_config,
        &diagnostics_config,
        resolve,
        frange
    ).unwrap();

    mx::mark("Filter for extract function assist");

    let assist: Assist = filter_extract_function_assist( assists )?;

    mx::mark("Apply extract function assist");

    let src_change = assist.source_change
        .as_ref()
        .unwrap()
        .clone();

    let (text_edit, maybe_snippet_edit) =
        src_change.get_source_and_snippet_edit(
            file_id,
        ).unwrap();

    let edited_text: String = apply_edits(
        text.clone(),
        text_edit.clone(),
        maybe_snippet_edit.clone(),
    );

    let renamed_text: String = rename_function(
        edited_text,
        "fun_name",
        callee_name,
    );

    // Ensure that the output file imports std::ops::ControlFlow if it uses it
    let fixed_cf_text: String = fixup_controlflow( renamed_text );

    mx::mark("Extraction End");

    let parent_method: String = parent_method_from_text(
        text,
        &range,
    );

    Ok( (fixed_cf_text, parent_method) )
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

    // Convert the input path to an `AbsPathBuf`
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

    // let analysis_host: AnalysisHost = AnalysisHost::with_database( db );
    // let analysis: Analysis = run_analysis( analysis_host );

    // MARKER: Get the assists and filter for extract function assist
    mx::mark("Get the assists");

    let assists: Vec<Assist> = get_assists( &db, &vfs, &input_abs_path, range );

    // mx::mark("1");
    // let assists_2: Vec<Assist> = get_assists(&analysis, &vfs, &input_abs_path, range);

    mx::mark("Filter for extract function assist");

    let assist: Assist = filter_extract_function_assist( assists )?;

    mx::mark("Apply extract function assist");

    let modified_code: String = apply_extract_function(
        &assist,
        &input_abs_path,
        &vfs,
        &callee_name,
    )?;

    mx::mark("Get parent method");

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

use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::visit::Visit;

/// Return the name of the function/method that contains the given [start, end)
/// byte range in `text`. Returns empty string if none found.
///
/// NOTE: Requires `proc-macro2` with the "span-locations" feature enabled.
pub fn parent_method_from_text(text: String, range: &(u32, u32)) -> String {
    let Ok(file) = syn::parse_file(&text) else {
        return String::new();
    };

    let line_starts = compute_line_starts(&text);
    let selection = (range.0 as usize, range.1 as usize);

    let mut visitor = FnCollector {
        text: &text,
        line_starts: &line_starts,
        fns: Vec::new(),
    };
    visitor.visit_file(&file);

    // Find the *innermost* function that contains the selection.
    let mut best: Option<(&str, usize, usize)> = None;
    for (name, start, end) in visitor.fns {
        if contains((start, end), selection) {
            match best {
                None => best = Some((name, start, end)),
                Some((_, b_start, b_end)) => {
                    if (end - start) < (b_end - b_start) {
                        best = Some((name, start, end));
                    }
                }
            }
        }
    }

    best.map(|(name, _, _)| name.to_string()).unwrap_or_default()
}

/// Collect function spans (name, start_byte, end_byte).
struct FnCollector<'a> {
    text: &'a str,
    line_starts: &'a [usize],
    fns: Vec<(&'a str, usize, usize)>,
}

impl<'a, 'ast> Visit<'ast> for FnCollector<'a> {
    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        // Free function
        let name = node.sig.ident.to_string();
        let (start, end) = span_to_offsets(node.block.span(), self.line_starts, self.text);
        self.fns.push((self.leak(name), start, end));
        // Recurse into the function in case there are nested modules, etc.
        syn::visit::visit_item_fn(self, node);
    }

    fn visit_item_impl(&mut self, node: &'ast syn::ItemImpl) {
        for item in &node.items {
            if let syn::ImplItem::Fn(m) = item {
                let name = m.sig.ident.to_string();
                let (start, end) = span_to_offsets(m.block.span(), self.line_starts, self.text);
                self.fns.push((self.leak(name), start, end));
            }
        }
        syn::visit::visit_item_impl(self, node);
    }

    fn visit_item_trait(&mut self, node: &'ast syn::ItemTrait) {
        for item in &node.items {
            if let syn::TraitItem::Fn(f) = item {
                if let Some(block) = &f.default {
                    let name = f.sig.ident.to_string();
                    let (start, end) = span_to_offsets(block.span(), self.line_starts, self.text);
                    self.fns.push((self.leak(name), start, end));
                }
            }
        }
        syn::visit::visit_item_trait(self, node);
    }

    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        // For inline modules (`mod m { ... }`) the content is present; recurse.
        if let Some((_brace, items)) = &node.content {
            for it in items {
                self.visit_item(it);
            }
        }
        // For `mod m;` (file modules) we can't see into another file from this text.
    }
}

impl<'a> FnCollector<'a> {
    /// Leak a `String` into a `'static` str so we can store &str in self.fns without lifetimes hell.
    /// This is fine for short-lived analysis in a tool; if you prefer, store `String` instead.
    fn leak(&self, s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
}

/// Compute the starting byte offset of each line (1-based line numbers).
fn compute_line_starts(text: &str) -> Vec<usize> {
    let mut starts = vec![0]; // line 1 starts at 0
    for (i, b) in text.bytes().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}

/// Convert a Span to byte start/end offsets within `text`.
///
/// This relies on proc_macro2's "span-locations" to get (line, column).
fn span_to_offsets(span: Span, line_starts: &[usize], text: &str) -> (usize, usize) {
    let start = span.start();
    let end = span.end();

    // Line numbers are 1-based; columns are (effectively) byte offsets within the line.
    let start_off = lc_to_offset(start.line, start.column, line_starts, text);
    let end_off = lc_to_offset(end.line, end.column, line_starts, text);

    (start_off.min(text.len()), end_off.min(text.len()))
}

fn lc_to_offset(line: usize, column: usize, line_starts: &[usize], text: &str) -> usize {
    if line == 0 || line > line_starts.len() {
        return text.len();
    }
    let base = line_starts[line - 1];
    base.saturating_add(column)
}

fn contains(outer: (usize, usize), inner: (usize, usize)) -> bool {
    let (o_start, o_end) = outer;
    let (i_start, i_end) = inner;
    o_start <= i_start && i_end <= o_end && i_start <= i_end
}
