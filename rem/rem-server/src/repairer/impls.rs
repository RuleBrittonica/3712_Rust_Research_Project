use std::path::PathBuf;
use rem_repairer::RepairerInput;
use rem_repairer::repairer_main;
use serde_json::json;

use crate::stdio::JsonResp;

pub fn handle_repair_file(
    file: PathBuf,
    new_fn_name: String,
) -> JsonResp{
    // Define the input struct (RepairerInput)
    let input: RepairerInput = RepairerInput::new(
        file,
        new_fn_name,
    );

    match repairer_main(input) {
        Ok( repair_return ) => {
            let idx = repair_return.idx;
            let system_name: String = repair_return.system_name.into();
            let repair_count = repair_return.repair_count;
            let changed_files: String = repair_return.changed_files.join(", ");
            JsonResp::ok(json!({
                "idx": idx,
                "system_name": system_name,
                "repair_count": repair_count,
                "changed_files": changed_files
            }))
        }
        Err( e ) => JsonResp::err(format!("{e:#}"))
    }
}