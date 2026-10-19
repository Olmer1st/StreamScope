//! StreamScope: bounded SSE framing, monotonic timing, safe reports, and capture.
pub mod capture;
pub mod sse;
use serde::Serialize;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};
use url::Url;

/// Maximum retained event previews. Capture continues when older entries are evicted.
pub const EVENT_CAP: usize = 256;
/// Maximum retained gap samples; the median describes this rolling window.
pub const GAP_CAP: usize = 2048;
/// Shared state locks are held only for bounded in-memory updates, never across awaits.
pub type Shared = Arc<Mutex<State>>;

/// A display-only, bounded event preview. Never serialized in a report.
#[derive(Clone, Debug)]
pub struct Preview {
    pub ordinal: u64,
    pub event_type: String,
    pub data: String,
    pub elapsed_ms: u64,
    pub gap_ms: Option<u64>,
    pub frame_bytes: usize,
}

/// Aggregated client observations. All durations start before request establishment.
#[derive(Clone, Default, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub event_count: u64,
    pub total_bytes: u64,
    pub headers_ms: Option<u64>,
    pub first_byte_ms: Option<u64>,
    pub first_event_ms: Option<u64>,
    pub latest_gap_ms: Option<u64>,
    pub min_observed_gap_ms: Option<u64>,
    pub max_observed_gap_ms: Option<u64>,
    pub median_recent_gap_ms: Option<u64>,
    pub comments: u64,
    pub evicted_events: u64,
    pub dropped_findings: u64,
}

/// Evidence contains numeric observations only; no remote strings or payloads.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Finding {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_ordinal: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_ms: Option<u64>,
}

/// Stream lifecycle reason, independent of verdict evaluation.
#[derive(Clone, Copy, Debug, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamEndReason {
    #[default]
    Running,
    Eof,
    Cancelled,
    Timeout,
    Transport,
    Protocol,
}

/// Mutable capture state with strictly bounded retained data.
#[derive(Clone, Default)]
pub struct State {
    pub metrics: Metrics,
    pub events: VecDeque<Preview>,
    pub findings: Vec<Finding>,
    pub http_status: Option<u16>,
    pub valid_content_type: bool,
    pub end: StreamEndReason,
    pub last_type: String,
    pub last_ms: Option<u64>,
    gaps: VecDeque<u64>,
    pub assertion_failed: bool,
}
impl State {
    /// Record a completed frame and retain only a short terminal-safe preview.
    pub fn event(
        &mut self,
        event: sse::Event,
        elapsed_ms: u64,
        max_gap: Option<u64>,
        tolerance: u64,
    ) {
        let gap = self.last_ms.map(|last| elapsed_ms.saturating_sub(last));
        self.metrics.event_count += 1;
        self.metrics.first_event_ms.get_or_insert(elapsed_ms);
        if let Some(gap) = gap {
            self.metrics.latest_gap_ms = Some(gap);
            self.metrics.min_observed_gap_ms =
                Some(self.metrics.min_observed_gap_ms.map_or(gap, |v| v.min(gap)));
            self.metrics.max_observed_gap_ms =
                Some(self.metrics.max_observed_gap_ms.map_or(gap, |v| v.max(gap)));
            if self.gaps.len() == GAP_CAP {
                self.gaps.pop_front();
            }
            self.gaps.push_back(gap);
            if let Some(limit) = max_gap {
                if gap > limit.saturating_add(tolerance) {
                    self.assertion_failed = true;
                    self.finding(
                        "GAP_EXCEEDED",
                        Some(self.metrics.event_count),
                        Some(gap),
                        Some(limit),
                    );
                }
            }
        }
        self.last_ms = Some(elapsed_ms);
        self.last_type = event.event_type.clone();
        if self.events.len() == EVENT_CAP {
            self.events.pop_front();
            self.metrics.evicted_events += 1;
        }
        self.events.push_back(Preview {
            ordinal: self.metrics.event_count,
            event_type: safe_preview(&event.event_type, 64),
            data: safe_preview(&event.data, 512),
            elapsed_ms,
            gap_ms: gap,
            frame_bytes: event.frame_bytes,
        });
    }

    /// Add bounded diagnostic evidence; aggregate overflow explicitly.
    pub fn finding(
        &mut self,
        code: &str,
        event_ordinal: Option<u64>,
        observed_ms: Option<u64>,
        threshold_ms: Option<u64>,
    ) {
        if self.findings.len() == 64 {
            self.metrics.dropped_findings += 1;
            return;
        }
        self.findings.push(Finding {
            code: code.into(),
            event_ordinal,
            observed_ms,
            threshold_ms,
        });
    }

    /// Snapshot metrics and compute the median only when needed for output.
    pub fn metrics(&self) -> Metrics {
        let mut metrics = self.metrics.clone();
        let mut gaps: Vec<_> = self.gaps.iter().copied().collect();
        if !gaps.is_empty() {
            let mid = gaps.len() / 2;
            gaps.select_nth_unstable(mid);
            metrics.median_recent_gap_ms = Some(gaps[mid]);
        }
        metrics
    }
}

/// Remove control sequences from live content, with a bounded Unicode preview.
pub fn safe_preview(text: &str, cap: usize) -> String {
    text.chars()
        .filter(|c| !c.is_control() || *c == '\n')
        .take(cap)
        .collect()
}

/// Validate the target; only HTTP(S), no embedded credentials, host required.
pub fn validate_url(input: &str) -> Result<Url, &'static str> {
    let url = Url::parse(input).map_err(|_| "URL must be an absolute HTTP(S) URL")?;
    if !matches!(url.scheme(), "http" | "https")
        || url.host_str().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err("Use HTTP(S) with a host and without embedded credentials");
    }
    Ok(url)
}

/// Exclude credentials, query, and fragment. URL paths may still be sensitive.
pub fn sanitize_url(url: &Url) -> String {
    let mut clean = url.clone();
    let _ = clean.set_username("");
    let _ = clean.set_password(None);
    clean.set_query(None);
    clean.set_fragment(None);
    clean.to_string()
}

/// Request/response header values are never retained; redact all rather than guessing secret names.
pub fn redact_header(_name: &str, _value: &str) -> &'static str {
    "[REDACTED]"
}

/// Stable report/v1. Events, IDs, arbitrary response headers, and error bodies are excluded.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub schema_version: &'static str,
    pub mode: &'static str,
    pub target: String,
    pub verdict: &'static str,
    pub exit_code: u8,
    pub transport: Transport,
    pub metrics: Metrics,
    pub findings: Vec<Finding>,
    pub end_reason: StreamEndReason,
    pub notes: Vec<&'static str>,
}
/// Only allowlisted transport metadata is emitted.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub http_status: Option<u16>,
    pub content_type: Option<&'static str>,
}

/// Finalize a report after capture. Empty streams are inconclusive, never a silent PASS.
pub fn report(state: &State, url: &Url) -> Report {
    let (verdict, exit_code) = match state.end {
        StreamEndReason::Running => ("INCONCLUSIVE", 4),
        StreamEndReason::Cancelled => ("INTERRUPTED", 130),
        StreamEndReason::Timeout | StreamEndReason::Transport | StreamEndReason::Protocol => {
            ("TRANSPORT_ERROR", 4)
        }
        _ if state.metrics.event_count == 0 => ("INCONCLUSIVE", 4),
        _ if state.assertion_failed => ("FAIL", 2),
        _ => ("PASS", 0),
    };
    Report {
        schema_version: "streamscope-report/v1",
        mode: "check",
        target: sanitize_url(url),
        verdict,
        exit_code,
        transport: Transport {
            http_status: state.http_status,
            content_type: state.valid_content_type.then_some("text/event-stream"),
        },
        metrics: state.metrics(),
        findings: state.findings.clone(),
        end_reason: state.end,
        notes: vec![
            "Timings are client observations, not server execution timings",
            "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded",
        ],
    }
}
