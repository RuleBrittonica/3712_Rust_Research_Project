pub mod error;
pub mod convert;
pub mod verify;
mod parser;

mod exports;
pub use exports::{
    VerificationError,
    VerificationInput,
    VerificationReturn,
    FileContent,
    ProgramPaths,
    call_verifier,
};