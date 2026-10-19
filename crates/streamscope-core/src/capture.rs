//! Cancellable HTTP capture. No raw URL, body, or transport error is logged.
use crate::{sse::Decoder, Shared, StreamEndReason};
use futures_util::StreamExt;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE};
use std::time::{Duration, Instant};
use tokio::sync::watch;
use url::Url;

/// HTTP headers accepted at the capture boundary. Values are never reported.
pub type Headers = HeaderMap;

/// Validated capture configuration. Zero limits are rejected at the CLI boundary.
#[derive(Clone)]
pub struct Config {
    pub url: Url,
    pub headers: HeaderMap,
    pub max_gap_ms: Option<u64>,
    pub first_event_ms: Option<u64>,
    pub tolerance_ms: u64,
    pub require_last_event: Option<String>,
    pub timeout_ms: u64,
    pub idle_timeout_ms: u64,
    pub max_frame_bytes: usize,
}

/// Parse one request header without including its value in errors.
pub fn parse_header(input: &str) -> Result<(HeaderName, HeaderValue), &'static str> {
    let (name, value) = input
        .split_once(':')
        .ok_or("Header must use NAME: VALUE syntax")?;
    let name = HeaderName::from_bytes(name.trim().as_bytes()).map_err(|_| "Invalid header name")?;
    let value = HeaderValue::from_str(value.trim()).map_err(|_| "Invalid header value")?;
    Ok((name, value))
}

/// Capture until EOF, an overall deadline, idle timeout, parser failure, or cancellation.
/// The single shared state is bounded; presentation never applies network backpressure.
pub async fn run(config: Config, state: Shared, mut cancel: watch::Receiver<bool>) {
    let started = Instant::now();
    let work = collect(&config, &state, started);
    tokio::select! {
        biased;
        _ = async { if !*cancel.borrow() { let _ = cancel.changed().await; } } => {
            let mut state = state.lock().unwrap(); state.end = StreamEndReason::Cancelled;
            state.finding("INTERRUPTED", None, None, None);
        }
        result = tokio::time::timeout(Duration::from_millis(config.timeout_ms), work) => {
            if result.is_err() {
                let mut state = state.lock().unwrap(); state.end = StreamEndReason::Timeout;
                state.finding("TIMEOUT", None, None, Some(config.timeout_ms));
            }
        }
    }
    let mut state = state.lock().unwrap();
    if state.end == StreamEndReason::Eof {
        if state.metrics.event_count == 0 {
            state.finding("NO_EVENTS", None, None, None);
        }
        if let Some(required) = &config.require_last_event {
            if state.last_type != *required {
                state.assertion_failed = true;
                state.finding("MISSING_LAST_EVENT", None, None, None);
            }
        }
    }
}

fn fail(state: &Shared, end: StreamEndReason, code: &str) {
    let mut state = state.lock().unwrap();
    state.end = end;
    state.finding(code, None, None, None);
}
fn ms(started: Instant) -> u64 {
    started.elapsed().as_millis().min(u64::MAX as u128) as u64
}

async fn collect(config: &Config, state: &Shared, started: Instant) {
    let client = match reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(client) => client,
        Err(_) => {
            fail(state, StreamEndReason::Transport, "CLIENT_INIT_FAILED");
            return;
        }
    };
    let response = match client
        .get(config.url.clone())
        .headers(config.headers.clone())
        .header("Accept", "text/event-stream")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => {
            fail(state, StreamEndReason::Transport, "CONNECTION_FAILED");
            return;
        }
    };
    {
        let mut state = state.lock().unwrap();
        state.http_status = Some(response.status().as_u16());
        state.metrics.headers_ms = Some(ms(started));
        state.valid_content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.split(';').next())
            .is_some_and(|v| v.trim().eq_ignore_ascii_case("text/event-stream"));
    }
    if !response.status().is_success() {
        fail(state, StreamEndReason::Transport, "HTTP_STATUS");
        return;
    }
    if !state.lock().unwrap().valid_content_type {
        fail(state, StreamEndReason::Transport, "WRONG_CONTENT_TYPE");
        return;
    }
    let mut stream = response.bytes_stream();
    let mut parser = Decoder::new(config.max_frame_bytes);
    loop {
        let next =
            tokio::time::timeout(Duration::from_millis(config.idle_timeout_ms), stream.next())
                .await;
        match next {
            Err(_) => {
                fail(state, StreamEndReason::Timeout, "READ_TIMEOUT");
                return;
            }
            Ok(None) => break,
            Ok(Some(Err(_))) => {
                fail(state, StreamEndReason::Transport, "ABRUPT_CLOSE");
                return;
            }
            Ok(Some(Ok(bytes))) => {
                let observed = ms(started);
                {
                    let mut state = state.lock().unwrap();
                    state.metrics.total_bytes += bytes.len() as u64;
                    if !bytes.is_empty() {
                        state.metrics.first_byte_ms.get_or_insert(observed);
                    }
                }
                // Yield between fixed-size slices even if the peer sends a very large chunk.
                for slice in bytes.chunks(8192) {
                    for &byte in slice {
                        match parser.push(byte) {
                            Ok(Some(event)) => {
                                let mut state = state.lock().unwrap();
                                if state.metrics.event_count == 0 {
                                    if let Some(limit) = config.first_event_ms {
                                        if observed > limit.saturating_add(config.tolerance_ms) {
                                            state.assertion_failed = true;
                                            state.finding(
                                                "FIRST_EVENT_EXCEEDED",
                                                Some(1),
                                                Some(observed),
                                                Some(limit),
                                            );
                                        }
                                    }
                                }
                                state.event(
                                    event,
                                    observed,
                                    config.max_gap_ms,
                                    config.tolerance_ms,
                                );
                            }
                            Ok(None) => {}
                            Err(error) => {
                                fail(state, StreamEndReason::Protocol, &error.to_string());
                                return;
                            }
                        }
                    }
                    state.lock().unwrap().metrics.comments = parser.comments;
                    tokio::task::yield_now().await;
                }
            }
        }
    }
    if let Err(error) = parser.finish() {
        fail(state, StreamEndReason::Protocol, &error.to_string());
        return;
    }
    state.lock().unwrap().end = StreamEndReason::Eof;
}
