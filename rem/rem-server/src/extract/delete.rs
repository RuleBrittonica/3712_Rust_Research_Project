use std::{path::PathBuf, collections::HashMap};
use serde_json::json;
use ra_ap_ide::AnalysisHost;
use ra_ap_vfs::Vfs;

use crate::utils::{to_vfs_path, flush_vfs_into_host};
use crate::stdio::JsonResp;

// TODO
pub fn handle_delete(
    host: &mut AnalysisHost,
    vfs: &mut Vfs,
    hashes: &mut HashMap<PathBuf, String>,
    path: PathBuf,
) -> JsonResp {
    hashes.remove(&path);

    match to_vfs_path(&path) {
        Ok(vp) => vfs.set_file_contents(vp, None),
        Err(e)  => return JsonResp::err(format!("{e:#}")),
    };

    flush_vfs_into_host(host, vfs);
    JsonResp::ok(json!({"status":"applied","path": path}))
}
