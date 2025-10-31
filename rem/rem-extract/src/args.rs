use clap::{ArgAction, Parser, Subcommand};

use std::path::PathBuf;

use crate::messages::{about::ABOUT, author::AUTHOR, version::VERSION};

#[derive(Parser)]
#[command(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
)]
pub struct EXTRACTArgs {
    #[command(subcommand)]
    pub command: EXTRACTCommands,
}

#[derive(Subcommand)]
pub enum EXTRACTCommands {
    // Run the extraction process with specific arguments
    Extract {
        #[arg(help = "The path to the file to refactor")]
        file_path: PathBuf,

        #[arg(help = "The name of the new function to create")]
        new_fn_name: String,

        #[arg(help = "Index to the start of the function to extract")]
        start_index: usize,

        #[arg(help = "Index to the end of the function to extract")]
        end_index: usize,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,

        #[arg(short, long, help = "Enable the metrics collection", action = ArgAction::SetTrue)]
        metrics: bool,

        #[arg(short, long, help = "Sent data to stdout using JSON format (interface::vscode)", action = ArgAction::SetTrue)]
        json: bool,
    },

    // Test the extraction process
    Test {
        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,

        # [arg(short, long, help = "Enable spammy output - rustc will yell at you", action = ArgAction::SetTrue)]
        spammy: bool,
    },
}
