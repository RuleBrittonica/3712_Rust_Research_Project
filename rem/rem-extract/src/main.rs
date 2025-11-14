mod logging;
mod messages;
mod startup;

use extract::extraction::{
    extract_method,
    ExtractionInput
};

use log::{
    // error,
    info
};

mod args;
use args::{
    EXTRACTArgs,
    EXTRACTCommands
};

mod extract;

mod test_details;
use extract::extract_tests::{
    test,
    test_verbose,
    test_spammy,
};

use ra_ap_ide::Analysis;
use ra_ap_vfs::FileId as VfsFileId;
use rem_interface::metrics as mx;
use rem_interface::vscode::{
    self as wire, Timing, Diagnostic, Severity,
    ok_extract, err_extract,
};

use serde_json;

mod error;

use clap::Parser;

use crate::extract::extraction::{analysis_from_single_file_std, extract_method_file};

fn main() {
    logging::init_logging();

    info!("Application Started");

    let args: EXTRACTArgs = EXTRACTArgs::parse();

    match &args.command {
        EXTRACTCommands::Extract {
            file_path,
            new_fn_name,
            start_index,
            end_index,
            verbose,
            metrics,
            json,
        } => {
            info!("Running 'extract' subcommand");
            info!("File Path: {:?}", file_path);
            info!("New Function Name: {}", new_fn_name);
            info!("Start Index: {}", start_index);
            info!("End Index: {}", end_index);
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });
            info!("Metrics: {}", if *metrics { "yes" } else { "no" });
            info!("JSON: {}", if *json { "yes" } else { "no" });

            let input = ExtractionInput::new(
                file_path.to_str().unwrap(),
                new_fn_name,
                *start_index as u32,
                *end_index as u32,
            );

            // Run extraction
            let extraction_output: Result<(String, String), error::ExtractionError> =
                // extract_method(input);
                extract_method_file(input);

            match extraction_output {
                Ok((output_code, caller_method)) => {
                    if *json {
                        // Build JSON envelope; attach timings if requested
                        let mut env = ok_extract(output_code.clone(), caller_method.clone(), vec![]);
                        if *metrics {
                            // pull all recorded timings from the global recorder
                            let timings = mx::take_as_timings();
                            if !timings.is_empty() {
                                env.timings = timings;
                            }
                        }
                        println!("{}", serde_json::to_string(&env).unwrap());
                    } else {
                        // Plain output to stdout
                        println!("{}", output_code);
                        println!("Extraction Successful");
                        if *metrics {
                            let timings = mx::take_as_timings();
                            print_timings_table(timings);
                        }
                    }
                }
                Err(e) => {
                    if *json {
                        let rem_err = map_error(e);
                        // You can add a diagnostic pointing at the provided selection
                        let diags: Vec<Diagnostic> = vec![Diagnostic {
                            file: Some(file_path.to_string_lossy().to_string()),
                            range: Some(wire::Range {
                                start: *start_index as u32,
                                end: *end_index as u32,
                            }),
                            severity: Severity::Error,
                            message: "Extraction failed at the provided selection".into(),
                            related: vec![],
                        }];
                        let mut env = err_extract(rem_err, diags);
                        if *metrics {
                            let timings = mx::take_as_timings();
                            if !timings.is_empty() {
                                env.timings = timings;
                            }
                        }
                        println!("{}", serde_json::to_string(&env).unwrap());
                    } else {
                        eprintln!("Error: {}", e);
                        if *metrics {
                            let timings = mx::take_as_timings();
                            print_timings_table(timings);
                        }
                    }
                }
            }
        },

        EXTRACTCommands::Test {
            verbose,
            spammy
        } => {
            if verbose.clone() || spammy.clone() {assert_ne!(verbose.clone(), spammy.clone(), "Verbose and Spammy cannot be run at the same time");}
            info!("Running 'test' subcommand");
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });
            if *verbose {
                test_verbose();
            } else if *spammy {
                test_spammy();
            }else {
                test();
            }

        },

        EXTRACTCommands::Ctx {} => {
            println!("Running 'ctx' subcommand");
            // Quickly test that we are building the startup context properly.
            let ctx = startup::single_file_std_context();
            for crate_id in ctx.base_graph.iter() {
                let name = ctx.base_graph[crate_id].display_name.as_ref()
                    .map(|n| n.canonical_name().to_string())
                    .unwrap_or_else(|| "<unnamed>".to_string());
                eprintln!("crate {crate_id:?} -> {name}");
            }
        },


        EXTRACTCommands::StdAnalysis {} => {
            println!("Running 'std-analysis' subcommand");

            // 1) Some code that actually uses std / core so we can see if inference works.
            let src = r#"
pub fn sum_vec(xs: Vec<i32>) -> i32 {
    xs.iter().sum()
}

pub fn use_result() -> Result<i32, std::io::Error> {
    Ok(sum_vec(vec![1, 2, 3]))
}"#.to_string();

            // 2) Run your new single-file+std analysis helper.
            let (analysis, file_id): (Analysis, VfsFileId) = analysis_from_single_file_std(src.clone());
            println!("File ID: {:?}", file_id);


            // 3) Print status for this file.
            match analysis.status(Some(file_id)) {
                Ok(status) => {
                    println!("=== analysis.status(Some(file_id)) ===");
                    println!("{status}");
                }
                Err(_) => {
                    println!("=== analysis.status(Some(file_id)) ===");
                    println!("<canceled>");
                }
            }

            // 4) Print a short crate graph (dot syntax).
            match analysis.view_crate_graph(false) {
                Ok(Ok(dot)) => {
                    println!();
                    println!("=== analysis.view_crate_graph(false) ===");
                    println!("{dot}");
                }
                Ok(Err(err)) => {
                    println!();
                    println!("=== analysis.view_crate_graph(false) ===");
                    println!("error: {err}");
                }
                Err(_) => {
                    println!();
                    println!("=== analysis.view_crate_graph(false) ===");
                    println!("<canceled>");
                }
            }

            // 5) Print the syntax tree for /main.rs so you can eyeball it.
            match analysis.view_syntax_tree(file_id) {
                Ok(tree) => {
                    println!();
                    println!("=== analysis.view_syntax_tree(file_id) ===");
                    println!("{tree}");
                }
                Err(_) => {
                    println!();
                    println!("=== analysis.view_syntax_tree(file_id) ===");
                    println!("<canceled>");
                }
            }

        },

    }
}

fn print_timings_table(mut timings: Vec<Timing>) {
    if timings.is_empty() {
        println!("No timings recorded.");
        return;
    }

    // Extract total to print last
    let mut total = None;
    timings.retain(|t| {
        if t.name == "cum:Extraction Start->Extraction End" || t.name == "total" {
            total = Some(Timing { name: t.name.clone(), nanos: t.nanos });
            false
        } else {
            true
        }
    });

    println!("\n=== Metrics (timings in nanoseconds) ===");
    for t in &timings {
        println!("{:<40} {:>15} ns", t.name, t.nanos);
    }

    if let Some(t) = total {
        println!("{:-<63}", "");
        println!("{:<40} {:>15} ns", t.name, t.nanos);
    }

    println!();
}

fn map_error(e: error::ExtractionError) -> rem_interface::vscode::RemError {
    use rem_interface::vscode::RemError;
    use serde_json::json;

    match e {
        error::ExtractionError::Io(inner) => RemError {
            code: "io_error".into(),
            message: inner.to_string(),
            details: None,
        },
        error::ExtractionError::Parse(inner) => RemError {
            code: "parse_error".into(),
            message: inner.to_string(),
            details: None,
        },
        error::ExtractionError::InvalidManifest => RemError {
            code: "invalid_manifest".into(),
            message: "Could not find a manifest file for the given path".into(),
            details: None,
        },
        error::ExtractionError::InvalidStartIdx => RemError {
            code: "invalid_start_idx".into(),
            message: "Invalid start index".into(),
            details: None,
        },
        error::ExtractionError::InvalidEndIdx => RemError {
            code: "invalid_end_idx".into(),
            message: "Invalid end index".into(),
            details: None,
        },
        error::ExtractionError::SameIdx => RemError {
            code: "same_idx".into(),
            message: "Start and end indices are the same".into(),
            details: None,
        },
        error::ExtractionError::InvalidIdxPair => RemError {
            code: "invalid_idx_pair".into(),
            message: "Invalid pair of start and end indices".into(),
            details: None,
        },
        error::ExtractionError::NoExtractFunction(assists) => RemError {
            code: "no_extract_function".into(),
            message: "No Extract Function Assist found for selection".into(),
            details: Some(json!({
                "assists": assists.iter().map(|a| format!("{a:?}")).collect::<Vec<_>>()
            })),
        },
        error::ExtractionError::CommentNotApplicable => RemError {
            code: "comment_not_applicable".into(),
            message: "Extraction not applicable for comment".into(),
            details: None,
        },
        error::ExtractionError::BracesNotApplicable => RemError {
            code: "braces_not_applicable".into(),
            message: "Extraction not applicable for braces".into(),
            details: None,
        },
        error::ExtractionError::ParentMethodNotFound => RemError {
            code: "parent_method_not_found".into(),
            message: "Parent method not found".into(),
            details: None,
        },
    }
}