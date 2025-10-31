use anyhow::{Context, Result};
use dashmap::DashMap;
use ra_ap_ide::{AnalysisHost, Change};
use ra_ap_project_model::{CargoConfig, ProjectManifest, ProjectWorkspace};
use ra_ap_vfs::{Vfs, VfsPath, VfsChange};
use std::{collections::HashSet, fs, path::{Path, PathBuf}};

