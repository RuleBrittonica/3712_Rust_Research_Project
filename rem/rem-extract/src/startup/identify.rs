use ra_ap_base_db::{CrateGraph, CrateId, CrateName, Dependency};
use triomphe::Arc;

pub fn find_crate_by_name(graph: &CrateGraph, name: &str) -> Option<CrateId> {
    graph.iter().find(|&cid| {
        graph[cid]
            .display_name
            .as_ref()
            .map(|n| n.canonical_name().as_str() == name)
            .unwrap_or(false)
    })
}

pub fn add_sysroot_deps(graph: &mut CrateGraph, my_crate: CrateId) {
    // core
    if let Some(core_id) = find_crate_by_name(graph, "core") {
        let dep = Dependency::with_prelude(
            CrateName::new("core".into()).expect("Unable to find crate core"), // adjust ctor to your version
            core_id,
            true,  // include in extern prelude
            true,  // mark as sysroot dep
        );
        let _ = graph.add_dep(my_crate, dep);
    }

    // std
    if let Some(std_id) = find_crate_by_name(graph, "std") {
        let dep = Dependency::with_prelude(
            CrateName::new("std".into()).expect("Unable to find crate std"),
            std_id,
            true,
            true,
        );
        let _ = graph.add_dep(my_crate, dep);
    }
}

use rustc_hash::FxHashMap;

pub fn build_ws_data(graph: &CrateGraph) -> FxHashMap<CrateId, Arc<CrateWorkspaceData>> {
    let mut map = FxHashMap::default();

    for crate_id in graph.iter() {
        map.insert(crate_id, default_ws_data());
    }

    map
}

use ra_ap_base_db::CrateWorkspaceData;

fn default_ws_data() -> Arc<CrateWorkspaceData> {
    Arc::new(CrateWorkspaceData {
        proc_macro_cwd: None,
        data_layout: Err("not available".into()),
        toolchain: None,
    })
}