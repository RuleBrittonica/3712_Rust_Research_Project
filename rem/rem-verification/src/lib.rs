pub mod error;
pub mod convert;
mod parser;
pub mod verify;

mod exports;
pub use exports::{
    VerificationError,
    VerificationInput,
    VerificationReturn,
    FileContent,
    ProgramPaths,
    call_verifier,
};