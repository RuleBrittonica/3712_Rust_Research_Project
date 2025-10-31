use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use anyhow::Result;
use ra_ap_ide::AnalysisHost;
use ra_ap_project_model::ProjectWorkspace;
use ra_ap_vfs::Vfs;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

// handlers & utils
use super::init::{init_daemon, FileRepr};
use super::create::handle_create;
use super::change::handle_change;
use super::delete::handle_delete;
use super::extract::handle_extract;

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
}

#[derive(Serialize)]
pub struct Response<T = JsonValue> {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// unified JSON response type used everywhere
pub type JsonResp = Response<JsonValue>;

impl<T: serde::Serialize> Response<T> {
    pub fn ok(data: T) -> Self {
        Self { ok: true, data: Some(data), error: None }
    }
    pub fn err(msg: impl Into<String>) -> Self {
        Self { ok: false, data: None, error: Some(msg.into()) }
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
    match req {
        Request::Init { manifest_path } => match init_daemon(&manifest_path) {
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

        Request::Create { path, text } => with_state(state, |st| {
            handle_create(&mut st.host, &mut st.vfs, &mut st.hashes, path, text)
        }),

        Request::Change { path, text } => with_state(state, |st| {
            handle_change(&mut st.host, &mut st.vfs, &mut st.hashes, path, text)
        }),

        Request::Delete { path } => with_state(state, |st| {
            handle_delete(&mut st.host, &mut st.vfs, &mut st.hashes, path)
        }),

        Request::Extract { file, new_fn_name, start, end } => with_state(state, |st| {
            handle_extract(&st.host, &st.vfs, file, new_fn_name, start, end)
        }),
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
