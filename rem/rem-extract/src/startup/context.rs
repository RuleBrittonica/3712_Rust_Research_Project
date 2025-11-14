use anyhow::Context;
use ra_ap_paths::Utf8PathBuf;
use ra_ap_project_model::{
    ProjectManifest,
    RustLibSource,
    ProjectWorkspace,
    CargoConfig,
};

use ra_ap_base_db::{
    ProcMacroPaths,
    CrateGraph,
};

use ra_ap_paths::AbsPath;
use ra_ap_paths::AbsPathBuf;
use ra_ap_vfs::FileId;

use super::types::{
    SingleFileStdContext,
    SysrootFileMap,
    TempScriptAnchor,
};


pub(crate) fn build_single_file_std_context() -> anyhow::Result<SingleFileStdContext> {
    // 1) Create a temporary anchor to act as our CargoScript - in creation we
    //    also write out a minimal Rust file so RA doesn't choke.
    let temp_anchor = TempScriptAnchor::new()?;

    // 2) Convert the path to the anchor to a Utf8PathBuf and then AbsPathBuf.
    let utf8 = Utf8PathBuf::from_path_buf(temp_anchor.path().clone())
        .map_err(|_| anyhow::anyhow!("Temp path is not valid UTF-8: {:?}", temp_anchor.path()))?;
    let abs_path = ra_ap_paths::AbsPathBuf::assert(utf8);

    // 3) Treat this file as the "manifest" for a ProjectWorkspace. Construct
    //    the workspace around it
    let manifest = ProjectManifest::from_manifest_file(abs_path.clone())
        .context("Failed to parse temporary Cargo manifest")?;

    // 4) Configure Cargo to scan for sysroot sources.
    let mut cargo_config = CargoConfig::default();
    cargo_config.sysroot = Some(RustLibSource::Discover);

    let extra_env = cargo_config.clone().extra_env;

    let progress = |msg: String| {
        eprintln!("ra_ap_project_model: {msg}");
    };

    // 5) Load Project Workspace (discovers sysroot, and builds internal model)
    let ws = ProjectWorkspace::load(manifest, &cargo_config, &progress)
        .context("Failed to load ProjectWorkspace for single-file std context")?;

    // 6) Convert to a CrateGraph and ProcMacroPaths using a FileLoader.
    let mut file_map = SysrootFileMap::new();

    // FileLoader<'_> = &mut dyn FnMut(&AbsPath) -> Option<FileId>
    let mut loader = |path: &AbsPath| -> Option<FileId> {
        // AbsPathBuf::from(AbsPath) is not directly implemented; use to_path_buf()
        let owned: AbsPathBuf = path.to_path_buf();
        Some(file_map.file_id_for(owned))
    };

    let (base_graph, proc_macros):
        (CrateGraph, ProcMacroPaths) =
        ws.to_crate_graph(&mut loader, &extra_env);

    // 7) base_graph now contains: core/std/alloc/etc. + dummy file crate.
    //    proc_macros contains any proc-macro crates.
    //    file_map has AbsPathBuf -> FileId for all of those.

    Ok(SingleFileStdContext {
        base_graph,
        proc_macros,
        sysroot_files: file_map,
    })



}
