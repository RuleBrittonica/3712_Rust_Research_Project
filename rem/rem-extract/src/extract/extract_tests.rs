use std::{
    fs,
    time::Instant,
    time::Duration,
    path::PathBuf,
    process::Command,
};
use syn::{
    parse_file,
    File
};
use colored::*;
use quote::ToTokens;
use log::info;
use regex::Regex;

use crate::{
    extract::extraction::extract_method,
    extract::extraction::ExtractionInput,
    error::ExtractionError,
    test_details::TEST_FILES, // Import Test Files Information from test_details.rs
};

pub struct TestFile<'a> {
    /// The name of the input file (without path)
    /// The input is in ./testdata/input/<input_file>/src/main.rs
    /// The correct output is in ./testdata/correct_output/<input_file>.rs
    pub input_file: &'a str,
    pub start_idx: u32,
    pub end_idx: u32,
}

impl TestFile<'_> {
    pub fn new(input_file: &str, start_idx: u32, end_idx: u32) -> TestFile<'_> {
        TestFile {
            input_file,
            start_idx,
            end_idx,
        }
    }
}

/// A TestInput needs a Path to write to, as part of the testing process is
/// writing the output to a file. This is not needed for the actual extraction
pub struct TestInput{
    pub file_path: String,
    pub output_path: String,
    pub new_fn_name: String,
    pub start_idx: u32,
    pub end_idx: u32,
}

// Helper function to convert a TestFile into an ExtractionInput
impl From<&TestFile<'_>> for TestInput {
    fn from(test_file: &TestFile<'_>) -> TestInput {
        // The file path is constructed as ./testdata/<input_file>/input/src/main.rs
        let file_path: String = PathBuf::from("testdata")
            .join(&test_file.input_file)
            .join("input")
            .join("src")
            .join("main.rs")
            .to_string_lossy()
            .to_string();
        let output_path: String = PathBuf::from("output")
            .join(&test_file.input_file)
            .with_extension("rs")
            .to_string_lossy()
            .to_string();

        TestInput {
            file_path,
            output_path,
            new_fn_name: "fun_name".to_string(),
            start_idx: test_file.start_idx,
            end_idx: test_file.end_idx,
        }
    }
}

/// Because the TestInput contains an output path, it also needs to be converted
/// into an ExtractionInput for the actual extraction process
impl From<&TestInput> for ExtractionInput {
    fn from(test_input: &TestInput) -> ExtractionInput {
        ExtractionInput {
            file_path: test_input.file_path.clone(),
            new_fn_name: test_input.new_fn_name.clone(),
            start_idx: test_input.start_idx,
            end_idx: test_input.end_idx,
        }
    }
}

/// Strips ANSI color codes from a string using a regex
/// This is useful for comparing strings with ANSI color codes to strings without
#[allow(dead_code)]
fn strip_ansi_codes(s: &str) -> String {
    let ansi_regex = Regex::new(r"\x1b\[([0-9]{1,2}(;[0-9]{0,2})*)m").unwrap();
    ansi_regex.replace_all(s, "").to_string()
}

/// Parse and subsequently compare the ASTs of two files. One file is provided
/// as a String, with the other being a reference to a file path (of the
/// expected file)
#[allow(dead_code)]
fn parse_and_compare_ast(output_content: &String, expected_file_path: &str) -> Result<bool, ExtractionError> {
    let expected_content: String = fs::read_to_string(expected_file_path)?;

    let output_ast: File = parse_file(&output_content)?;
    let expected_ast: File = parse_file(&expected_content)?;

    // Convert both ASTs back into token stres for comparison
    // FIXME this is sometimes buggy and is convinced that the two files are
    // different when they are infact the same
    let output_tokens: String = output_ast.into_token_stream().to_string();
    let expected_tokens: String = expected_ast.into_token_stream().to_string();

    Ok(output_tokens == expected_tokens)
}

// Helper function to show differences between two files
#[allow(dead_code)]
fn print_file_diff(expected_file_path: &str, output_file_path: &str) -> Result<(), std::io::Error> {
    let expected_content = fs::read_to_string(expected_file_path)?;
    let output_content = fs::read_to_string(output_file_path)?;

    if expected_content != output_content {
        println!("Differences found between expected and output:");
        for diff in diff::lines(&expected_content, &output_content) {
            match diff {
                diff::Result::Left(l) => println!("{}", format!("- {}", l).red()), // Expected but not in output
                diff::Result::Right(r) => println!("{}", format!("+ {}", r).green()), // In output but not in expected
                diff::Result::Both(b, _) => println!("{}", format!("  {}", b)), // Same in both
            }
        }
    } else {
        println!("{}", "No differences found.".green());
    }

    Ok(())
}

/// Removes all files in a given directory
#[allow(dead_code)]
fn remove_all_files(dir: &PathBuf) -> () {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            fs::remove_file(path).unwrap();
        }
    }
}

#[allow(dead_code)]
pub fn test() {
    // Clear the output directory before running tests
    let output_dir = PathBuf::from("./output");
    remove_all_files(&output_dir);

    // Measure total time at the start
    let overall_start_time: Instant = Instant::now();

    info!("Starting tests...");

    // Initialize counters and time trackers
    let mut total_tests: i32 = 0;
    let mut passed_stage_1: i32 = 0;
    let mut passed_tests: i32 = 0;
    let mut failed_tests: i32 = 0;
    let mut total_test_time: Duration = Duration::new(0, 0);
    let mut min_test_time: Option<Duration> = None;
    let mut max_test_time: Option<Duration> = None;

    for (index, test_file) in TEST_FILES.iter().enumerate() {
        let test_start_time: Instant = Instant::now();

        total_tests += 1;

        let input: TestInput = TestInput::from(test_file);
        // The expected file is now in ./testdata/<testname>/output/src/main.rs
        let expected_file_path: String = PathBuf::new()
            .join("testdata")
            .join(&test_file.input_file)
            .join("output")
            .join("main.rs")
            .to_string_lossy()
            .to_string();
        let output_path: String = input.output_path.clone();

        let extraction_input: ExtractionInput = ExtractionInput::from(&input);

        // Call the extraction method and handle errors
        let extraction_result: Result<(String, _), ExtractionError> = extract_method(extraction_input);

        // Measure time taken for extraction
        let test_elapsed_time: Duration = test_start_time.elapsed();
        total_test_time += test_elapsed_time;

        // Update min and max times
        if let Some(min_time) = min_test_time {
            if test_elapsed_time < min_time {
                min_test_time = Some(test_elapsed_time);
            }
        } else {
            min_test_time = Some(test_elapsed_time);
        }

        if let Some(max_time) = max_test_time {
            if test_elapsed_time > max_time {
                max_test_time = Some(test_elapsed_time);
            }
        } else {
            max_test_time = Some(test_elapsed_time);
        }

        let test_elapsed_time_secs: f64 = test_elapsed_time.as_secs_f64();
        let test_elapsed_time_str: String = if test_elapsed_time_secs < 1.0 {
            format!("{:.2}ms", test_elapsed_time_secs * 1000.0)
        } else {
            format!("{:.2}s", test_elapsed_time_secs)
        };

        let test_name: &str = test_file.input_file.trim_end_matches(".rs");
        let mut extraction_status: String = "FAILED".red().to_string();
        let mut comparison_status: String = "N/A".to_string(); // Default to not applicable

        if extraction_result.is_ok() {
            // Unwrap the result to get the output code (as we know that it is
            // successful)
            // Also write the output code to the output file for later viewing
            let (extraction_result, _)  = extraction_result.unwrap();
            fs::write(&output_path, &extraction_result).unwrap();
            extraction_status = "PASSED".green().to_string();
            passed_stage_1 += 1;

            // Compare the output file with the expected file's AST
            match parse_and_compare_ast(&extraction_result, &expected_file_path) {
                Ok(is_identical) => {
                    if is_identical {
                        comparison_status = "PASSED".green().to_string();
                        passed_tests += 1;
                    } else {
                        comparison_status = "FAILED".red().to_string();
                        failed_tests += 1;
                    }
                }
                Err(e) => {
                    comparison_status = format!("Error: {}", e).red().to_string();
                    failed_tests += 1;
                }
            }
        } else if let Err(e) = extraction_result {
            extraction_status = format!("FAILED: {}", e).red().to_string();
            failed_tests += 1;
        }

        println!("Test {} | {} | {}: {} in {}", index + 1, extraction_status, comparison_status, test_name, test_elapsed_time_str);
        // Strip ANSI color codes before logging
        let clean_extraction_status = strip_ansi_codes(&extraction_status);
        let clean_comparison_status = strip_ansi_codes(&comparison_status);

        info!("Test {} | {} | {}: {} in {}", index + 1, clean_extraction_status, clean_comparison_status, test_name, test_elapsed_time_str);

    }

    // Total elapsed time
    let total_elapsed_time: Duration = overall_start_time.elapsed();
    let total_elapsed_time_secs: f64 = total_elapsed_time.as_secs_f64();
    let total_elapsed_time_str: String = if total_elapsed_time_secs < 1.0 {
        format!("{:.2}ms", total_elapsed_time_secs * 1000.0)
    } else {
        format!("{:.2}s", total_elapsed_time_secs)
    };

    // Calculate average time per test
    let average_time_per_test: f64 = if total_tests > 0 {
        total_test_time.as_secs_f64() / total_tests as f64
    } else {
        0.0
    };

    let average_time_str: String = if average_time_per_test < 1.0 {
        format!("{:.2}ms", average_time_per_test * 1000.0)
    } else {
        format!("{:.2}s", average_time_per_test)
    };

    // Print overall statistics
    println!("------------------------------------------------------------------");
    println!("Total tests run: {}", total_tests);
    println!("Tests passed stage 1: {}", passed_stage_1);
    println!("Tests passed: {}", passed_tests);
    println!("Tests failed: {}", failed_tests);
    println!("Total time: {}", total_elapsed_time_str);
    println!("Average time per test: {}", average_time_str);

    // Log overall statistics
    info!("------------------------------------------------------------------");
    info!("Total tests run: {}", total_tests);
    info!("Tests passed stage 1: {}", passed_stage_1);
    info!("Tests passed: {}", passed_tests);
    info!("Tests failed: {}", failed_tests);
    info!("Total time: {}", total_elapsed_time_str);
    info!("Average time per test: {}", average_time_str);

    if let Some(min_time) = min_test_time {
        let min_time_secs: f64 = min_time.as_secs_f64();
        let min_time_str: String = if min_time_secs < 1.0 {
            format!("{:.2}ms", min_time_secs * 1000.0)
        } else {
            format!("{:.2}s", min_time_secs)
        };
        println!("Shortest test time: {}", min_time_str);
        info!("Shortest test time: {}", min_time_str);
    }

    if let Some(max_time) = max_test_time {
        let max_time_secs: f64 = max_time.as_secs_f64();
        let max_time_str: String = if max_time_secs < 1.0 {
            format!("{:.2}ms", max_time_secs * 1000.0)
        } else {
            format!("{:.2}s", max_time_secs)
        };
        println!("Longest test time: {}", max_time_str);
        info!("Longest test time: {}", max_time_str);
    }
}

#[allow(dead_code)]
pub fn test_verbose() {
    // Clear the output directory before running tests
    let output_dir = PathBuf::from("./output");
    remove_all_files(&output_dir);

    // Measure total time at the start
    let overall_start_time: Instant = Instant::now();

    info!("Starting tests...");

    // Initialize counters and time trackers
    let mut total_tests: i32 = 0;
    let mut passed_stage_1: i32 = 0;
    let mut passed_tests: i32 = 0;
    let mut failed_tests: i32 = 0;
    let mut total_test_time: Duration = Duration::new(0, 0);
    let mut min_test_time: Option<Duration> = None;
    let mut max_test_time: Option<Duration> = None;

    let allowed_tests: Vec<&'static str> = vec![
        // "break_loop_nested",
        // "comments_in_block_expr",
    ];

    for (index, test_file) in TEST_FILES.iter().enumerate() {

        // Take a snapshot of all files in the environment
        let snapshot = fs::read_dir("./").unwrap().map(|entry| entry.unwrap().path()).collect::<Vec<PathBuf>>();

        let test_start_time: Instant = Instant::now();

        total_tests += 1;

        let input: TestInput = TestInput::from(test_file);
        let output_path: String = input.output_path.clone();
        let expected_file_path: String = PathBuf::new()
            .join("testdata")
            .join(&test_file.input_file)
            .join("output")
            .join("main.rs")
            .to_string_lossy()
            .to_string();

        // Skip tests not in the allowed_tests list
        // if the allowed_tests list is not empty
        if !allowed_tests.is_empty() && !allowed_tests.contains(&test_file.input_file) {
            continue;
        }

        let extraction_input: ExtractionInput = ExtractionInput::from(&input);

        // Call the extraction method and handle errors
        let extraction_result: Result<(String, _), ExtractionError> = extract_method(extraction_input);

        // Measure time taken for extraction
        let test_elapsed_time: Duration = test_start_time.elapsed();
        total_test_time += test_elapsed_time;

        // Update min and max times
        if let Some(min_time) = min_test_time {
            if test_elapsed_time < min_time {
                min_test_time = Some(test_elapsed_time);
            }
        } else {
            min_test_time = Some(test_elapsed_time);
        }

        if let Some(max_time) = max_test_time {
            if test_elapsed_time > max_time {
                max_test_time = Some(test_elapsed_time);
            }
        } else {
            max_test_time = Some(test_elapsed_time);
        }

        let test_elapsed_time_secs: f64 = test_elapsed_time.as_secs_f64();
        let test_elapsed_time_str: String = if test_elapsed_time_secs < 1.0 {
            format!("{:.2}ms", test_elapsed_time_secs * 1000.0)
        } else {
            format!("{:.2}s", test_elapsed_time_secs)
        };

        let test_name: &str = test_file.input_file.trim_end_matches(".rs");
        let mut extraction_status: String = "FAILED".red().to_string();
        let mut comparison_status: String = "N/A".cyan().to_string(); // Default to not applicable
        let mut compilation_status: String = "N/A".magenta().to_string(); // Default to not applicable

        if extraction_result.is_ok() {
            // Unwrap the result to get the output code (as we know that it is
            // successful)
            // Also write the output code to the output file for later viewing
            let (extraction_result, _)  = extraction_result.unwrap();
            fs::write(&output_path, &extraction_result).unwrap();
            extraction_status = "PASSED".green().to_string();
            passed_stage_1 += 1;

            // Compare the output file with the expected file's AST
            match parse_and_compare_ast(&extraction_result, &expected_file_path) {
                Ok(is_identical) => {
                    if is_identical {
                        comparison_status = "PASSED".green().to_string();
                        passed_tests += 1;
                    } else {
                        comparison_status = "FAILED".red().to_string();
                        failed_tests += 1;
                    }
                }
                Err(e) => {
                    comparison_status = format!("Error: {}", e).red().to_string();
                    failed_tests += 1;
                }
            }

            // Complilation check using rustc
            let compile_result = Command::new("rustc")
                .arg(&output_path)
                .output();

            match compile_result {
                Ok(output) => {
                    if output.status.success() {
                        compilation_status = "PASSED".green().to_string();
                    } else {
                        compilation_status = "FAILED".red().to_string();
                        // failed_tests += 1;
                    }
                }
                Err(e) => {
                    compilation_status = format!("Error: {}", e).red().to_string();
                    // failed_tests += 1;
                }
            }

        } else if let Err(e) = extraction_result {
            extraction_status = format!("FAILED: {}", e).red().to_string();
            failed_tests += 1;
        }

        println!(
            "Test {} | {} | {} | {}: {} in {}",
            index + 1,
            extraction_status,
            comparison_status,
            compilation_status,
            test_name,
            test_elapsed_time_str
        );

        let clean_extraction_status = strip_ansi_codes(&extraction_status);
        let clean_comparison_status = strip_ansi_codes(&comparison_status);
        let clean_compilation_status = strip_ansi_codes(&compilation_status);

        info!(
            "Test {} | {} | {} | {}: {} in {}",
            index + 1,
            clean_extraction_status,
            clean_comparison_status,
            clean_compilation_status,
            test_name,
            test_elapsed_time_str
        );
        // Print differences if the test failed
        if clean_comparison_status == "FAILED" || clean_extraction_status == "FAILED" {
            println!("==================================================================");
            println!("Differences or compilation errors found for test '{}':", test_name);
            print_file_diff(&expected_file_path, &output_path).unwrap();
            println!("==================================================================");
            println!("");
        }

        // Delete all files created by the test (i.e anything not in the
        // snapshot)
        let current_files = fs::read_dir("./").unwrap().map(|entry| entry.unwrap().path()).collect::<Vec<PathBuf>>();
        for file in current_files {
            if !snapshot.contains(&file) {
                fs::remove_file(file).unwrap();
            }
        }
    }

    // Total elapsed time
    let total_elapsed_time: Duration = overall_start_time.elapsed();
    let total_elapsed_time_secs: f64 = total_elapsed_time.as_secs_f64();
    let total_elapsed_time_str: String = if total_elapsed_time_secs < 1.0 {
        format!("{:.2}ms", total_elapsed_time_secs * 1000.0)
    } else {
        format!("{:.2}s", total_elapsed_time_secs)
    };

    // Calculate average time per test
    let average_time_per_test: f64 = if total_tests > 0 {
        total_test_time.as_secs_f64() / total_tests as f64
    } else {
        0.0
    };

    let average_time_str: String = if average_time_per_test < 1.0 {
        format!("{:.2}ms", average_time_per_test * 1000.0)
    } else {
        format!("{:.2}s", average_time_per_test)
    };

    // Print overall statistics
    println!("------------------------------------------------------------------");
    println!("Total tests run: {}", total_tests);
    println!("Tests passed stage 1: {}", passed_stage_1);
    println!("Tests passed: {}", passed_tests);
    println!("Tests failed: {}", failed_tests);
    println!("Total time: {}", total_elapsed_time_str);
    println!("Average time per test: {}", average_time_str);

    // Log overall statistics
    info!("------------------------------------------------------------------");
    info!("Total tests run: {}", total_tests);
    info!("Tests passed stage 1: {}", passed_stage_1);
    info!("Tests passed: {}", passed_tests);
    info!("Tests failed: {}", failed_tests);
    info!("Total time: {}", total_elapsed_time_str);
    info!("Average time per test: {}", average_time_str);

    if let Some(min_time) = min_test_time {
        let min_time_secs: f64 = min_time.as_secs_f64();
        let min_time_str: String = if min_time_secs < 1.0 {
            format!("{:.2}ms", min_time_secs * 1000.0)
        } else {
            format!("{:.2}s", min_time_secs)
        };
        println!("Shortest test time: {}", min_time_str);
        info!("Shortest test time: {}", min_time_str);
    }

    if let Some(max_time) = max_test_time {
        let max_time_secs: f64 = max_time.as_secs_f64();
        let max_time_str: String = if max_time_secs < 1.0 {
            format!("{:.2}ms", max_time_secs * 1000.0)
        } else {
            format!("{:.2}s", max_time_secs)
        };
        println!("Longest test time: {}", max_time_str);
        info!("Longest test time: {}", max_time_str);
    }
}

#[allow(dead_code)]
pub fn test_spammy() {
    // Clear the output directory before running tests
    let output_dir = PathBuf::from("./output");
    remove_all_files(&output_dir);

    // Measure total time at the start
    let overall_start_time: Instant = Instant::now();

    info!("Starting tests...");

    // Initialize counters and time trackers
    let mut total_tests: i32 = 0;
    let mut passed_stage_1: i32 = 0;
    let mut passed_tests: i32 = 0;
    let mut failed_tests: i32 = 0;
    let mut total_test_time: Duration = Duration::new(0, 0);
    let mut min_test_time: Option<Duration> = None;
    let mut max_test_time: Option<Duration> = None;

    let allowed_tests: Vec<&'static str> = vec![
        // "break_loop_nested",
        // "comments_in_block_expr",
    ];

    for (index, test_file) in TEST_FILES.iter().enumerate() {

        // Take a snapshot of all files in the environment
        let snapshot = fs::read_dir("./").unwrap().map(|entry| entry.unwrap().path()).collect::<Vec<PathBuf>>();

        let test_start_time: Instant = Instant::now();

        total_tests += 1;

        let input: TestInput = TestInput::from(test_file);
        let output_path: String = input.output_path.clone();
        let expected_file_path: String = PathBuf::new()
            .join("testdata")
            .join(&test_file.input_file)
            .join("output")
            .join("main.rs")
            .to_string_lossy()
            .to_string();

        // Skip tests not in the allowed_tests list
        // if the allowed_tests list is not empty
        if !allowed_tests.is_empty() && !allowed_tests.contains(&test_file.input_file) {
            continue;
        }

        let extraction_input: ExtractionInput = ExtractionInput::from(&input);

        // Call the extraction method and handle errors
        let extraction_result: Result<(String, _), ExtractionError> = extract_method(extraction_input);

        // Measure time taken for extraction
        let test_elapsed_time: Duration = test_start_time.elapsed();
        total_test_time += test_elapsed_time;

        // Update min and max times
        if let Some(min_time) = min_test_time {
            if test_elapsed_time < min_time {
                min_test_time = Some(test_elapsed_time);
            }
        } else {
            min_test_time = Some(test_elapsed_time);
        }

        if let Some(max_time) = max_test_time {
            if test_elapsed_time > max_time {
                max_test_time = Some(test_elapsed_time);
            }
        } else {
            max_test_time = Some(test_elapsed_time);
        }

        let test_elapsed_time_secs: f64 = test_elapsed_time.as_secs_f64();
        let test_elapsed_time_str: String = if test_elapsed_time_secs < 1.0 {
            format!("{:.2}ms", test_elapsed_time_secs * 1000.0)
        } else {
            format!("{:.2}s", test_elapsed_time_secs)
        };

        let test_name: &str = test_file.input_file.trim_end_matches(".rs");
        let mut extraction_status: String = "FAILED".red().to_string();
        let mut comparison_status: String = "N/A".cyan().to_string(); // Default to not applicable
        let mut compilation_status: String = "N/A".magenta().to_string(); // Default to not applicable

        if extraction_result.is_ok() {
            // Unwrap the result to get the output code (as we know that it is
            // successful)
            // Also write the output code to the output file for later viewing
            let (extraction_result, _)  = extraction_result.unwrap();
            fs::write(&output_path, &extraction_result).unwrap();
            extraction_status = "PASSED".green().to_string();
            passed_stage_1 += 1;

            // Compare the output file with the expected file's AST
            match parse_and_compare_ast(&extraction_result, &expected_file_path) {
                Ok(is_identical) => {
                    if is_identical {
                        comparison_status = "PASSED".green().to_string();
                        passed_tests += 1;
                    } else {
                        comparison_status = "FAILED".red().to_string();
                        failed_tests += 1;
                    }
                }
                Err(e) => {
                    comparison_status = format!("Error: {}", e).red().to_string();
                    failed_tests += 1;
                }
            }

            // Complilation check using rustc
            let compile_result = Command::new("rustc")
                .arg(&output_path)
                .output();

            match compile_result {
                Ok(output) => {
                    if output.status.success() {
                        compilation_status = "PASSED".green().to_string();
                    } else {
                        compilation_status = format!("FAILED: {}", String::from_utf8_lossy(&output.stderr)).red().to_string();
                        // failed_tests += 1;
                    }
                }
                Err(e) => {
                    compilation_status = format!("Error: {}", e).red().to_string();
                    // failed_tests += 1;
                }
            }

        } else if let Err(e) = extraction_result {
            extraction_status = format!("FAILED: {}", e).red().to_string();
            failed_tests += 1;
        }

        println!(
            "Test {} | {} | {} | {}: {} in {}",
            index + 1,
            extraction_status,
            comparison_status,
            compilation_status,
            test_name,
            test_elapsed_time_str
        );

        let clean_extraction_status = strip_ansi_codes(&extraction_status);
        let clean_comparison_status = strip_ansi_codes(&comparison_status);
        let clean_compilation_status = strip_ansi_codes(&compilation_status);

        info!(
            "Test {} | {} | {} | {}: {} in {}",
            index + 1,
            clean_extraction_status,
            clean_comparison_status,
            clean_compilation_status,
            test_name,
            test_elapsed_time_str
        );
        // Print differences if the test failed
        if clean_comparison_status == "FAILED" || clean_extraction_status == "FAILED" {
            println!("==================================================================");
            println!("Differences or compilation errors found for test '{}':", test_name);
            print_file_diff(&expected_file_path, &output_path).unwrap();
            println!("==================================================================");
            println!("");
        }

        // Delete all files created by the test (i.e anything not in the
        // snapshot)
        let current_files = fs::read_dir("./").unwrap().map(|entry| entry.unwrap().path()).collect::<Vec<PathBuf>>();
        for file in current_files {
            if !snapshot.contains(&file) {
                fs::remove_file(file).unwrap();
            }
        }
    }

    // Total elapsed time
    let total_elapsed_time: Duration = overall_start_time.elapsed();
    let total_elapsed_time_secs: f64 = total_elapsed_time.as_secs_f64();
    let total_elapsed_time_str: String = if total_elapsed_time_secs < 1.0 {
        format!("{:.2}ms", total_elapsed_time_secs * 1000.0)
    } else {
        format!("{:.2}s", total_elapsed_time_secs)
    };

    // Calculate average time per test
    let average_time_per_test: f64 = if total_tests > 0 {
        total_test_time.as_secs_f64() / total_tests as f64
    } else {
        0.0
    };

    let average_time_str: String = if average_time_per_test < 1.0 {
        format!("{:.2}ms", average_time_per_test * 1000.0)
    } else {
        format!("{:.2}s", average_time_per_test)
    };

    // Print overall statistics
    println!("------------------------------------------------------------------");
    println!("Total tests run: {}", total_tests);
    println!("Tests passed stage 1: {}", passed_stage_1);
    println!("Tests passed: {}", passed_tests);
    println!("Tests failed: {}", failed_tests);
    println!("Total time: {}", total_elapsed_time_str);
    println!("Average time per test: {}", average_time_str);

    // Log overall statistics
    info!("------------------------------------------------------------------");
    info!("Total tests run: {}", total_tests);
    info!("Tests passed stage 1: {}", passed_stage_1);
    info!("Tests passed: {}", passed_tests);
    info!("Tests failed: {}", failed_tests);
    info!("Total time: {}", total_elapsed_time_str);
    info!("Average time per test: {}", average_time_str);

    if let Some(min_time) = min_test_time {
        let min_time_secs: f64 = min_time.as_secs_f64();
        let min_time_str: String = if min_time_secs < 1.0 {
            format!("{:.2}ms", min_time_secs * 1000.0)
        } else {
            format!("{:.2}s", min_time_secs)
        };
        println!("Shortest test time: {}", min_time_str);
        info!("Shortest test time: {}", min_time_str);
    }

    if let Some(max_time) = max_test_time {
        let max_time_secs: f64 = max_time.as_secs_f64();
        let max_time_str: String = if max_time_secs < 1.0 {
            format!("{:.2}ms", max_time_secs * 1000.0)
        } else {
            format!("{:.2}s", max_time_secs)
        };
        println!("Longest test time: {}", max_time_str);
        info!("Longest test time: {}", max_time_str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_and_compare_ast_identical() -> Result<(), ExtractionError> {
        // Create temporary files for expected and output content
        let expected_file: NamedTempFile = NamedTempFile::new()?;

        // Write identical content to both files
        let content: &str = "fn example() -> i32 { 42 }";
        fs::write(expected_file.path(), content)?;

        // Run the function
        let result: bool = parse_and_compare_ast(
            &content.to_string(),
            expected_file.path().to_str().unwrap())?;

        // Assert that the result is true
        assert!(result, "The ASTs should be identical");

        Ok(())
    }

    #[test]
    fn test_parse_and_compare_ast_different() -> Result<(), ExtractionError> {
        // Create temporary files for expected and output content
        let expected_file: NamedTempFile = NamedTempFile::new()?;

        // Write different content to the files
        let output_content: &str = "fn example() -> i32 { 42 }";
        let expected_content: &str = "fn example() -> i32 { 43 }";
        fs::write(expected_file.path(), expected_content)?;

        // Run the function
        let result: bool = parse_and_compare_ast(
            &output_content.to_string(),
            expected_file.path().to_str().unwrap()
        )?;

        // Assert that the result is false
        assert!(!result, "The ASTs should be different");

        Ok(())
    }

    #[test]
    fn test_parse_and_compare_ast_file_not_found() -> Result<(), ExtractionError> {
        // Non-existent file paths
        let non_existent_file: &str = "non_existent_file.rs";

        // Run the function
        let result: Result<bool, ExtractionError> = parse_and_compare_ast(
            &"".to_string(),
             non_existent_file
        );

        // Assert that the result is an error
        assert!(result.is_err(), "The function should return an error for non-existent files");

        Ok(())
    }

    #[test]
    fn test_parse_and_compare_ast_empty_files() -> Result<(), ExtractionError> {
        // Create temporary files for empty content
        let expected_file: NamedTempFile = NamedTempFile::new()?;

        // Write empty content to both files
        fs::write(expected_file.path(), "")?;

        // Run the function
        let result: bool = parse_and_compare_ast(
            &"".to_string(),
            expected_file.path().to_str().unwrap(),
        )?;

        // Assert that the result is true
        assert!(result, "The ASTs for empty files should be identical");

        Ok(())
    }

    #[test]
    fn test_parse_and_compare_ast_invalid_content() -> Result<(), ExtractionError> {
        // Create temporary files with invalid content
        let expected_file: NamedTempFile = NamedTempFile::new()?;

        // Write invalid content to both files
        let invalid_content: &str = "fn example { 42 "; // Missing closing brace
        fs::write(expected_file.path(), invalid_content)?;

        // Run the function
        let result: Result<bool, ExtractionError> = parse_and_compare_ast(
            &invalid_content.to_string(),
            expected_file.path().to_str().unwrap()
        );

        // Assert that the result is an error
        assert!(result.is_err(), "The function should return an error for invalid content");

        Ok(())
    }

    #[test]
    fn test_parse_and_compare_ast_different_formatting() -> Result<(), ExtractionError> {
        // Create temporary files with the same logical content but different formatting
        let expected_file: NamedTempFile = NamedTempFile::new()?;

        // Write different formatting to the files
        let output_content: &str = "fn example() -> i32 { 42 }";
        let expected_content: &str = "fn example() -> i32 {\n    42\n}";
        fs::write(expected_file.path(), expected_content)?;

        // Run the function
        let result: bool = parse_and_compare_ast(
            &output_content.to_string(),
            expected_file.path().to_str().unwrap(),
        )?;

        // Assert that the result is true (assuming the formatting does not affect the AST)
        assert!(result, "The ASTs should be identical despite different formatting");

        Ok(())
    }
}
