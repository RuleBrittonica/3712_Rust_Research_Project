use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use crate::vscode::{Envelope, Timing};

/// Global recorder
static REC: Lazy<Mutex<Recorder>> = Lazy::new(|| Mutex::new(Recorder::default()));

#[derive(Default)]
struct Recorder {
    /// Ordered list of (id, name, time).
    markers: Vec<Marker>,
    /// First-seen id per marker name (stable).
    name_to_id: HashMap<String, u32>,
    /// Next id to assign.
    next_id: u32,

    /// Ad-hoc spans recorded explicitly (label -> seconds).
    spans: Vec<(String, f64)>,
}

#[derive(Clone)]
struct Marker {
    id: u32,
    name: String,
    t: Instant,
}

impl Recorder {
    fn assign_id(&mut self, name: &str) -> u32 {
        if let Some(&id) = self.name_to_id.get(name) {
            return id;
        }
        let id = self.next_id;
        self.next_id += 1;
        self.name_to_id.insert(name.to_string(), id);
        id
    }

    fn find_marker_index_by_name(&self, name: &str) -> Option<usize> {
        self.markers.iter().position(|m| m.name == name)
    }
}

/* ============================ Public API ============================ */

/// Record a timing marker with a human-readable name.
/// The first call is considered the "start" marker for cumulative timings.
pub fn mark(name: impl Into<String>) {
    let name = name.into();
    let mut r = REC.lock().unwrap();
    let id = r.assign_id(&name);
    r.markers.push(Marker { id, name, t: Instant::now() });
}

/// Start an ad-hoc span; recorded when the guard is dropped.
pub fn span(label: impl Into<String>) -> SpanGuard {
    SpanGuard {
        label: label.into(),
        start: Instant::now(),
        committed: false,
    }
}

/// Compute (and optionally record) a span between two named markers.
/// Returns `Some(seconds)` if both markers exist and end is after start.
pub fn span_between(start_name: &str, end_name: &str, record_phase: bool) -> Option<f64> {
    let mut r = REC.lock().unwrap();
    let si = r.find_marker_index_by_name(start_name)?;
    let ei = r.find_marker_index_by_name(end_name)?;
    if ei <= si {
        return None;
    }
    let s = r.markers[si].t;
    let e = r.markers[ei].t;
    let secs = (e - s).as_secs_f64();
    if record_phase {
        r.spans.push((format!("span:{start_name}->{end_name}"), secs));
    }
    Some(secs)
}

/// List marker names in order of recording
pub fn list_markers() -> Vec<String> {
    let r = REC.lock().unwrap();
    r.markers.iter().map(|m| m.name.clone()).collect()
}

/// Record a custom-named span between two *named* markers (first occurrence of each)
/// Returns seconds if both markers exist and order is valid.
pub fn span_between_markers(
    start_name: &str,
    end_name: &str,
    label: impl Into<String>,
) -> Option<f64> {
    let mut r = REC.lock().unwrap();
    let si = r.find_marker_index_by_name(start_name)?;
    let ei = r.find_marker_index_by_name(end_name)?;
    if ei <= si { return None; }
    let secs = (r.markers[ei].t - r.markers[si].t).as_secs_f64();
    r.spans.push((label.into(), secs));
    Some(secs)
}

/// Record a custom-named span between two markers by *index* in the recorded order.
/// Example: 0-based indices; span_between_indices(3, 5, "apply_total")
pub fn span_between_indices(
    start_idx: usize,
    end_idx: usize,
    label: impl Into<String>,
) -> Option<f64> {
    let mut r = REC.lock().unwrap();
    if start_idx >= r.markers.len() || end_idx >= r.markers.len() || end_idx <= start_idx {
        return None;
    }
    let secs = (r.markers[end_idx].t - r.markers[start_idx].t).as_secs_f64();
    r.spans.push((label.into(), secs));
    Some(secs)
}

/// Drain the current recorder as a list of `Timing` entries.
/// Includes:
///  - cumulative phases: `cum:<first>-><marker_i>`
///  - incremental phases: `inc:<marker_i-1>-><marker_i>`
///  - explicit spans: `span:<label>` and any `span:<A->B>` recorded
pub fn take_as_timings() -> Vec<Timing> {
    let mut r = REC.lock().unwrap();

    let mut out: Vec<Timing> = Vec::new();

    if !r.markers.is_empty() {
        // Cumulative: first -> each
        let first = &r.markers[0];
        for (i, m) in r.markers.iter().enumerate() {
            let secs = (m.t - first.t).as_secs_f64();
            let name = if i == 0 {
                format!("cum:{}->{}", m.name, m.name)
            } else {
                format!("cum:{}->{}", first.name, m.name)
            };
            out.push(Timing { name, seconds: secs });
        }

        // Incremental: prev -> current
        for w in r.markers.windows(2) {
            let a = &w[0];
            let b = &w[1];
            let secs = (b.t - a.t).as_secs_f64();
            out.push(Timing {
                name: format!("inc:{}->{}", a.name, b.name),
                seconds: secs,
            });
        }
    }

    // Ad-hoc spans
    for (label, secs) in r.spans.drain(..) {
        out.push(Timing { name: label, seconds: secs });
    }

    // Reset for next run
    r.markers.clear();
    r.name_to_id.clear();
    r.next_id = 0;

    out
}

/// Convenience: attach current metrics to an Envelope (appends to `timings`)
pub fn attach_to<T>(mut env: Envelope<T>) -> Envelope<T> {
    let timings = take_as_timings();
    if !timings.is_empty() {
        if env.timings.is_empty() {
            env.timings = timings;
        } else {
            env.timings.extend(timings);
        }
    }
    env
}

/* ============================ RAII guard ============================ */

pub struct SpanGuard {
    label: String,
    start: Instant,
    committed: bool,
}

impl SpanGuard {
    /// Manually end the span now and record it.
    pub fn end(mut self) {
        if !self.committed {
            let secs = self.start.elapsed().as_secs_f64();
            REC.lock().unwrap().spans.push((format!("span:{}", self.label), secs));
            self.committed = true;
        }
    }
}

impl Drop for SpanGuard {
    fn drop(&mut self) {
        if !self.committed {
            let secs = self.start.elapsed().as_secs_f64();
            REC.lock().unwrap().spans.push((format!("span:{}", self.label), secs));
            self.committed = true;
        }
    }
}
