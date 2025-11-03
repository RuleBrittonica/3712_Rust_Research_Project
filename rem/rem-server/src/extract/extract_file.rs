use std::path::PathBuf;

use anyhow::anyhow;
use serde_json::json;

use rem_extract::extract::extraction::extract_method_file;
use rem_extract::extract::extraction::ExtractionInput;

use crate::stdio::JsonResp;

pub fn handle_extract_file(
    path: PathBuf,
    new_fn_name: String,
    start: u32,
    end: u32,
) -> JsonResp {
    let input: ExtractionInput = ExtractionInput {
        file_path: path.to_str().unwrap().to_string() ,
        new_fn_name,
        start_idx: start,
        end_idx: end,
    };

    match extract_method_file(input) {
        Ok( (modified_code, caller_method) ) => {
            JsonResp::ok(json!({"output": modified_code, "callsite": caller_method}))
        }
        Err(e) => JsonResp::err(format!("{e:#}")),
    }
}