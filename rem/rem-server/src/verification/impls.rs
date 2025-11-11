use std::path::PathBuf;

use crate::stdio::JsonResp;

pub fn handle_verification(
    crate_path: PathBuf,
    file: PathBuf,
    fn_name: String,
    new_fn_name: String,
    charon_path: PathBuf,
    aeneas_path: PathBuf,
) -> JsonResp {
    // Define the input struct (VerificationInput)
    // Run the verifyer from the verifyer crate
    todo!()
}