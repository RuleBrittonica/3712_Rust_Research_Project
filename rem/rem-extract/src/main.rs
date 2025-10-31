mod logging;
mod messages;
mod extraction_utils;

mod extraction;
use extraction::{
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

mod extract_tests;
mod test_details;
use extract_tests::{
    test,
    test_verbose,
    test_spammy,
};

mod error;

use clap::Parser;

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
        } => {
            info!("Running 'run' subcommand");
            info!("File Path: {:?}", file_path);
            info!("New Function Name: {}", new_fn_name);
            info!("Start Index: {}", start_index);
            info!("End Index: {}", end_index);
            info!("Verbose: {}", if *verbose { "yes" } else { "no" });

            let input = ExtractionInput::new(
                file_path.to_str().unwrap(),
                new_fn_name,
                *start_index as u32,
                *end_index as u32,
            );

            let extraction_output: Result<(String, String), error::ExtractionError> = extract_method(input);
            let (output_code, _caller_method) = match extraction_output {
                Ok((output_code, caller_method)) => {
                    info!("Output Code: {}", output_code);
                    info!("Caller Method: {}", caller_method);
                    (output_code, caller_method)
                },
                Err(e) => {
                    info!("Error: {}", e);
                    return;
                }
            };

            println!("{}", output_code);
            println!("Extraction Successful");
        }

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

        }

    }
}