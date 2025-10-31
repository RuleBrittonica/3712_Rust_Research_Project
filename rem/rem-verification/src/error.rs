//! Implements specific error handling for the Verification System

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum AENEASError {
    RuntimeError,
    InvalidPathConversion
}

impl fmt::Display for AENEASError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AENEASError::RuntimeError => { write!(f, "AENEAS Failed to run. Terminating verification") }
            AENEASError::InvalidPathConversion => { write!(f, "Failed to convert LLBC path to a CoQ path") }
        }
    }
}

impl Error for AENEASError {}
