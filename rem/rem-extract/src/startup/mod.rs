//! This module is responsible for the startup information needed for the
//! extractor to have context about the binary being analyzed.
//! It build out a Workspace around std / core, and then we extend that
//! workspace with our single file to allow for 90% of the features of
//! Rust-Analyzer to work properly. Crucially this allows the extract method to
//! get much better type information about the code being analyzed.
//!

use std::sync::OnceLock;

pub mod types;
pub mod context;
pub mod identify; 

use types::SingleFileStdContext;
use context::build_single_file_std_context;

static SINGLE_FILE_STD_CTX: OnceLock<SingleFileStdContext> = OnceLock::new();

pub fn single_file_std_context() -> &'static SingleFileStdContext {
    SINGLE_FILE_STD_CTX.get_or_init(|| {
        build_single_file_std_context().expect("failed to init single-file std context")
    })
}