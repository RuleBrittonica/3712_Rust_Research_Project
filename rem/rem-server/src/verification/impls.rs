use std::path::PathBuf;
use rem_verification::VerificationInput;
use rem_verification::FileContent;
use rem_verification::VerificationReturn;
use rem_verification::ProgramPaths;
use rem_verification::call_verifier;
use serde_json::json;

use crate::stdio::JsonResp;

pub fn handle_verification(
    file_path: PathBuf,
    original_content: String,
    refactored_content: String,
    fn_name: String,
    charon_path: PathBuf,
    aeneas_path: PathBuf,
) -> JsonResp {
    let original_file = FileContent::new(file_path.clone(), original_content);
    let refactored_file = FileContent::new(file_path, refactored_content);
    let programs = ProgramPaths {
        charon: charon_path,
        aeneas: aeneas_path,
    };

    let input: VerificationInput = VerificationInput::new(
        original_file,
        refactored_file,
        fn_name,
        programs
    );

    match call_verifier(input) {
        Ok( verification_result ) => {

            JsonResp::ok(json!({
                "success": verification_result.success,
            }))
        }
        Err( e ) => JsonResp::err(format!("{e:#}"))
    }
}