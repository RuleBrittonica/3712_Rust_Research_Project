use std::{fs, path::PathBuf, collections::HashMap};
use serde_json::json;
use ra_ap_ide::AnalysisHost;
use ra_ap_vfs::Vfs;

use crate::utils::{to_vfs_path, flush_vfs_into_host, hash_bytes};
use crate::stdio::JsonResp;

// TODO
pub fn handle_create(
    host: &mut AnalysisHost,
    vfs: &mut Vfs,
    hashes: &mut HashMap<PathBuf, String>,
    path: PathBuf,
    text: Option<String>,
) -> JsonResp {
    let bytes = match text {
        Some(t) => t.into_bytes(),
        None => match fs::read(&path) {
            Ok(b) => b,
            Err(e) => return JsonResp::err(format!("read {}: {e:#}", path.display())),
        },
    };

    let new_hash = hash_bytes(&bytes);
    hashes.insert(path.clone(), new_hash);

    match to_vfs_path(&path) {
        Ok(vp) => vfs.set_file_contents(vp, Some(bytes)),
        Err(e)  => return JsonResp::err(format!("{e:#}")),
    };

    flush_vfs_into_host(host, vfs);
    JsonResp::ok(json!({"status":"applied","path": path}))
}
