use clap::{ArgAction, Parser, Subcommand};
use std::path::PathBuf;

// Local modules
use crate::messages::{about::ABOUT, author::AUTHOR, version::VERSION};

#[derive(Parser)]
#[command(
    version = VERSION,
    about = ABOUT,
    author = AUTHOR,
)]
pub struct CLIArgs {
    #[command(subcommand)]
    pub command: CLICommands,
}

#[derive(Subcommand)]
pub enum CLICommands {
    /// Run the whole process. Converts the LLBC files to CoQ, then
    /// runs verification checks on the CoQ files.
    Run {
        #[arg(help = "Path to the LLBC of the original program.")]
        original_llbc: PathBuf,

        #[arg(help = "Path to the LLBC of the refactored program.")]
        refactored_llbc: PathBuf,

        #[arg(help = "Top Level Function Name")]
        top_level_function: String,

        /// Optional output directory for the generated `.v` files
        #[arg(short, long, help = "Output directory for generated Coq files")]
        out_dir: Option<PathBuf>,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,

        #[arg(short, long, help = "Cleanup all files other than .llbc", action= ArgAction::SetTrue)]
        cleanup: bool,

        /// Optional path to the Aeneas binary.
        #[arg(long, help = "Optional path to the Aeneas binary")]
        aeneas_path: Option<PathBuf>,
    },

    /// Run various verification checks on the CoQ files.
    Verify {
        #[arg(help = "Path to the CoQ of the original program.")]
        original_coq: PathBuf,

        #[arg(help = "Path to the CoQ of the refactored program.")]
        refactored_coq: PathBuf,

        #[arg(help = "Top Level Function Name")]
        top_level_function: String,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,

        /// Optional path to the Aeneas binary.
        #[arg(long, help = "Optional path to the Aeneas binary")]
        aeneas_path: Option<PathBuf>,
    },

    /// Convert the LLBC files to CoQ using AENEAS.
    Convert {
        #[arg(help = "Path to the LLBC of the original program.")]
        original_llbc: PathBuf,

        #[arg(help = "Path to the LLBC of the refactored program.")]
        refactored_llbc: PathBuf,

        /// Optional output directory for the generated `.llbc` files
        #[arg(short, long, help = "Output directory for generated llbc files")]
        out_dir: Option<PathBuf>,

        #[arg(short, long, help = "Enable verbose output", action = ArgAction::SetTrue)]
        verbose: bool,

        /// Optional path to the Aeneas binary.
        #[arg(long, help = "Optional path to the Aeneas binary")]
        aeneas_path: Option<PathBuf>,
    },

    /// Run the crate tests.
    Test {
        #[arg(short, long, help = "Enable verbose test output", action = ArgAction::SetTrue)]
        verbose: bool,

        /// Optional path to the Aeneas binary.
        #[arg(long, help = "Optional path to the Aeneas binary")]
        aeneas_path: Option<PathBuf>,
    },
}
