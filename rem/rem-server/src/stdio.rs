use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use anyhow::Result;
use ra_ap_ide::AnalysisHost;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::Vfs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

use crate::extract::extract_file::handle_extract_file;
// handlers & utils
use crate::init::{init_daemon, FileRepr};
use crate::extract::create::handle_create;
use crate::extract::change::handle_change;
use crate::extract::delete::handle_delete;
use crate::extract::extract::handle_extract;
use crate::repairer::handle_repair_file;
use crate::verification::handle_verification;

#[derive(Deserialize)]
#[serde(tag = "op")]
pub enum Request {
    #[serde(rename = "init")]
    Init { manifest_path: PathBuf },

    #[serde(rename = "create")]
    Create { path: PathBuf, #[serde(default)] text: Option<String> },

    #[serde(rename = "change")]
    Change { path: PathBuf, #[serde(default)] text: Option<String> },

    #[serde(rename = "delete")]
    Delete { path: PathBuf },

    #[serde(rename = "extract")]
    Extract { file: PathBuf, new_fn_name: String, start: u32, end: u32 },

    #[serde(rename = "extract_file")]
    ExtractFile { file: PathBuf, new_fn_name: String, start: u32, end: u32 },

    #[serde(rename = "repair")]
    RepairFile { file: PathBuf, new_fn_name: String },

    #[serde(rename = "verify")]
    VerifyFile {
        file_path: PathBuf,
        original_content: String,
        refactored_content: String,
        fn_name: String,
        charon_path: PathBuf,
        aeneas_path: PathBuf,
     },
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response<T = JsonValue> {
    // { "ok": true,  "data": ... }
    Ok { ok: bool, data: T },

    // { "ok": false, "error": "..." }
    Err { ok: bool, error: String },
}


// unified JSON response type used everywhere
pub type JsonResp = Response<JsonValue>;

impl<T: serde::Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Response::Ok { ok: true, data }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Response::Err { ok: false, error: msg.into() }
    }
}

struct State {
    workspace: ProjectWorkspace,
    host: AnalysisHost,
    vfs: Vfs,
    hashes: HashMap<PathBuf, String>,
}

pub fn run_stdio_server() -> Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut lines = stdin.lock().lines();

    let mut state: Option<State> = None;

    while let Some(Ok(line)) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }

        let resp: JsonResp = match serde_json::from_str::<Request>(&line) {
            Err(e) => JsonResp::err(format!("invalid json: {e}")),
            Ok(req) => handle_request(req, &mut state),
        };

        // exactly one JSON line on stdout per request; logs go to stderr
        let s = serde_json::to_string(&resp).unwrap();
        writeln!(stdout, "{s}")?;
        stdout.flush()?;
    }

    eprintln!("[daemon] stdin closed; exiting.");
    Ok(())
}

fn handle_request(req: Request, state: &mut Option<State>) -> JsonResp {
    use Request as R;
    match req {
        R::Init { manifest_path } => match init_daemon(&manifest_path) {
            Ok(core) => {
                let mut hashes = HashMap::new();
                for FileRepr { path, hash } in core.hashed_files {
                    hashes.insert(PathBuf::from(path.as_str()), hash);
                }
                let st = State {
                    workspace: core.workspace,
                    host: core.ah, // AnalysisHost returned by init
                    vfs: core.vfs,
                    hashes,
                };
                *state = Some(st);
                JsonResp::ok(json!({ "status": "initialized" }))
            }
            Err(e) => JsonResp::err(format!("{e:#}")),
        },

        R::Create { path, text } => with_state(state, |st| {
            handle_create(&mut st.host, &mut st.vfs, &mut st.hashes, path, text)
        }),

        R::Change { path, text } => with_state(state, |st| {
            handle_change(&mut st.host, &mut st.vfs, &mut st.hashes, path, text)
        }),

        R::Delete { path } => with_state(state, |st| {
            handle_delete(&mut st.host, &mut st.vfs, &mut st.hashes, path)
        }),

        R::Extract { file, new_fn_name, start, end } => with_state(state, |st| {
            handle_extract(&st.host, &st.vfs, file, new_fn_name, start, end)
        }),

        R::ExtractFile { file, new_fn_name, start, end } => {
            handle_extract_file(file, new_fn_name, start, end)
        },

        R::RepairFile { file, new_fn_name } => {
            handle_repair_file(file, new_fn_name)
        },

        R::VerifyFile { file_path, original_content, refactored_content, fn_name, charon_path, aeneas_path  } => {
            handle_verification(
                file_path,
                original_content,
                refactored_content,
                fn_name,
                charon_path,
                aeneas_path,
            )
        },

        // _ => JsonResp::err("unimplemented operation"),
    }
}

fn with_state<F>(st: &mut Option<State>, f: F) -> JsonResp
where
    F: FnOnce(&mut State) -> JsonResp,
{
    match st.as_mut() {
        Some(s) => f(s),
        None => JsonResp::err(r#"not initialized; send {"op":"init"} first"#),
    }
}
