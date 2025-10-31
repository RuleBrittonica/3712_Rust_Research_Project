use serde::{Deserialize, Serialize};

/// Bump this on breaking changes to the wire format.
pub const SCHEMA_NAME: &str = "rem-interface";
pub const SCHEMA_VERSION: u32 = 1;

/// Top-level envelope every CLI command prints once to STDOUT (single JSON object).
#[derive(Debug, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub schema: Schema,
    pub op: Operation,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<RemError>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timings: Vec<Timing>,
    /// Free-form expansion point for future fields (CLI can insert anything here).
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub meta: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    pub name: &'static str,
    pub version: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Extract,
    Repair,
    Verify,
}

/// Current extraction payload: (extracted function, callsite replacement).
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractData {
    pub extracted_fn: String,
    pub callsite: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemError {
    /// Stable, machine-readable code (extension switches on this).
    pub code: String,
    /// Human-readable message (for UI).
    pub message: String,
    /// Optional extra info, e.g., assists, spans, raw tool errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Diagnostic {
    pub file: Option<String>,
    pub range: Option<Range>,       
    pub severity: Severity,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related: Vec<RelatedInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Range {
    pub start: u32,
    pub end: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedInformation {
    pub file: Option<String>,
    pub range: Option<Range>,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timing {
    pub name: String,
    pub seconds: f64,
}

pub fn ok_extract(extracted_fn: String, callsite: String, timings: Vec<Timing>) -> Envelope<ExtractData> {
    Envelope {
        schema: Schema { name: SCHEMA_NAME, version: SCHEMA_VERSION },
        op: Operation::Extract,
        ok: true,
        data: Some(ExtractData { extracted_fn, callsite }),
        error: None,
        diagnostics: vec![],
        timings,
        meta: Default::default(),
    }
}

pub fn err_extract(error: RemError, diagnostics: Vec<Diagnostic>) -> Envelope<ExtractData> {
    Envelope {
        schema: Schema { name: SCHEMA_NAME, version: SCHEMA_VERSION },
        op: Operation::Extract,
        ok: false,
        data: None,
        error: Some(error),
        diagnostics,
        timings: vec![],
        meta: Default::default(),
    }
}