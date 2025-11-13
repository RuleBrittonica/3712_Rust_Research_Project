use core::panic;
use config::{
    Config,
    File,
};
use clap::Parser;
use log::{
    // error,
    info
};

use rem_utils::local_config::Settings;

// use std::path::PathBuf;

// Local modules
mod args;
use args::{
    CLIArgs,
    CLICommands,
};
mod error;
mod logging;
mod messages;

mod exports;


mod convert;
use convert::coq_conversion;

mod verify;
use verify::coq_verification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    logging::init_logging();

    info!("Application Started");

    let config = Config::builder()
        // "Config" here means it will look for a file named "Config.toml" by default.
        .add_source(File::with_name("Config")
        .required(true))
        .build()?;

    // log the settings
    // The settings shouldn't be acessed after this point unless we do not
    // provide an aeneas path in the CLI (or have a AENEAS_PATH in the environment).
    let s: Settings = config.try_deserialize()?;
    info!("AENEAS path: {}", s.programs.aeneas);
    info!("CHARON path: {}", s.programs.charon);
    info!("Primitives path: {}", s.files.primitives);

    let args: CLIArgs = CLIArgs::parse();
    match &args.command {
        CLICommands::Run {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose,
            top_level_function,
            cleanup,
            aeneas_path,
        } => {
            // First we need to convert the files!
            let (original_coq, refactored_coq) = coq_conversion(
                &original_llbc,
                &refactored_llbc,
                &out_dir,
                aeneas_path,
            )?;

            info!("LLBC to CoQ conversion completed successfully.");

            // Now we can run the verification.
            let (coq_project, equivcheck, primitives, success) = coq_verification(
                &original_coq,
                &refactored_coq,
                &top_level_function
            )?;

            if success {
                info!("Verification completed successfully.");
            } else {
                info!("Verification failed.");
            }

            if *cleanup {
                // Cleanup the files
                std::fs::remove_file(&original_coq)?;
                std::fs::remove_file(&refactored_coq)?;
                std::fs::remove_file(&coq_project)?;
                std::fs::remove_file(&equivcheck)?;
                std::fs::remove_file(&primitives)?;
            }

            eprintln!("COQ_PROJECT: {}", coq_project.display());
            eprintln!("EQUIVCHECK: {}", equivcheck.display());
            eprintln!("PRIMITIVES: {}", primitives.display());
            eprintln!("SUCCESS: {}", success);
            eprintln!("OUTPUT_END");

            // If successful, exit with 0. Otherwise, exit with 1.
            if success {
                std::process::exit(0);
            } else {
                std::process::exit(1);
            }
        },

        CLICommands::RunAll {
            original_file,
            refactored_file,
            top_level_function,
            charon_path,
            aeneas_path
        } => {
            let charon_path = match charon_path {
                Some(path) => path.clone(),
                None => panic!("CHARON path must be provided for RunAll command."),
            };
            let aeneas_path = match aeneas_path {
                Some(path) => path.clone(),
                None => panic!("AENEAS path must be provided for RunAll command."),
            };
            let fn_name = top_level_function.clone();
            let programs = exports::ProgramPaths {
                charon: charon_path,
                aeneas: aeneas_path,
            };
            let original_file = exports::FileContent::from_path(original_file.to_path_buf())?;
            let refactored_file = exports::FileContent::from_path(refactored_file.to_path_buf())?;
            let input = exports::VerificationInput::new(
                original_file,
                refactored_file,
                fn_name,
                programs,
            );
            let output: Result<exports::VerificationReturn, exports::VerificationError> = exports::call_verifier(input);
            match output {
                Ok(result) => {
                    info!("RunAll completed successfully: {:?}", result);
                    Ok(())
                },
                Err(e) => {
                    info!("RunAll failed: {:?}", e);
                    Err(Box::new(e))
                }
            }
        }

        CLICommands::Verify {
            original_coq,
            refactored_coq,
            top_level_function,
            verbose: _,
            aeneas_path,
        } => {
            let paths: (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf, bool) = coq_verification(
                &original_coq,
                &refactored_coq,
                &top_level_function,
            )?;

            Ok(())
        },

        CLICommands::Convert {
            original_llbc,
            refactored_llbc,
            out_dir,
            verbose,
            aeneas_path,
        } => {
            if *verbose {
                info!("Starting LLBC to Coq conversion...");
                info!("Original LLBC: {:?}", original_llbc);
                info!("Refactored LLBC: {:?}", refactored_llbc);
                info!("Output directory: {:?}", out_dir);
            }

            // Call the conversion function.
            // Clone the paths because they are borrowed from the CLIArgs.
            let coq_converstion_res: (std::path::PathBuf, std::path::PathBuf) = coq_conversion(
                &original_llbc,
                &refactored_llbc,
                &out_dir,
                &aeneas_path,
            )?;

            info!("LLBC to Coq conversion completed successfully.");
            Ok(())
        },

        CLICommands::Test {
            verbose: _,
            aeneas_path: _,
        } => {
            todo!("Implement the test command.");
        },
    }
}