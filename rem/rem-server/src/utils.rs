use std::path::PathBuf;

use anyhow::Error;
use ra_ap_ide::AnalysisHost;
use ra_ap_vfs::{Vfs, VfsPath};


pub fn to_vfs_path(
    path: &PathBuf,
) -> Result<VfsPath, Error> {
    todo!()
}

pub fn flush_vfs_into_host(
    host: &mut AnalysisHost,
    vfs: &Vfs,
) -> () {
    todo!()
}

pub fn hash_bytes(
    bytes: &[u8],
) -> String {
    todo!()
}