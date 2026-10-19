use std::{
    process::{Command, Output, Stdio},
    time::Duration,
};
use streamscope_fixtures::{bind, serve, Scenario};
use tokio::sync::watch;

async fn run(scenario: Scenario, args: &[&str], route: &str) -> Output {
    let listener = bind(0).await.unwrap();
    let url = format!("http://{}/{route}", listener.local_addr().unwrap());
    let (stop, rx) = watch::channel(false);
    let server = tokio::spawn(serve(listener, scenario, rx));
    let args: Vec<_> = args.iter().map(|s| s.to_string()).collect();
    let output = tokio::task::spawn_blocking(move || {
        Command::new(env!("CARGO_BIN_EXE_streamscope"))
            .args(["check", "--url", &url, "--format", "json"])
            .args(args)
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    let _ = stop.send(true);
    if tokio::time::timeout(Duration::from_secs(6), server)
        .await
        .is_err()
    {
        panic!("fixture shutdown exceeded deadline");
    }
    output
}
fn json(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stdout).unwrap()
}
#[tokio::test(flavor = "multi_thread")]
async fn normal_and_burst_positive_controls() {
    for scenario in [Scenario::Normal, Scenario::Burst] {
        let out = run(
            scenario,
            &[
                "--max-gap-ms",
                "2000",
                "--first-event-ms",
                "5000",
                "--require-last-event",
                "done",
            ],
            "events",
        )
        .await;
        assert_eq!(
            out.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        let report = json(&out);
        assert_eq!(report["schemaVersion"], "streamscope-report/v1");
        assert_eq!(report["metrics"]["eventCount"], 4);
        assert_eq!(report["verdict"], "PASS");
    }
}
#[tokio::test(flavor = "multi_thread")]
async fn gap_negative_control() {
    let out = run(Scenario::Gap, &["--max-gap-ms", "1500"], "events").await;
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(json(&out)["findings"][0]["code"], "GAP_EXCEEDED");
    assert_eq!(json(&out)["findings"][0]["eventOrdinal"], 3);
}
#[tokio::test(flavor = "multi_thread")]
async fn missing_terminal_invalid_utf8_and_transport_failures() {
    for (scenario, route, args, code, finding) in [
        (
            Scenario::Truncated,
            "events",
            vec!["--require-last-event", "done"],
            2,
            "MISSING_LAST_EVENT",
        ),
        (Scenario::BadUtf8, "events", vec![], 4, "INVALID_UTF8"),
        (
            Scenario::Normal,
            "wrong-type",
            vec![],
            4,
            "WRONG_CONTENT_TYPE",
        ),
        (Scenario::Normal, "unavailable", vec![], 4, "HTTP_STATUS"),
        (
            Scenario::Gap,
            "events",
            vec!["--idle-timeout-ms", "1000"],
            4,
            "READ_TIMEOUT",
        ),
        (
            Scenario::Normal,
            "events",
            vec!["--timeout-ms", "1"],
            4,
            "TIMEOUT",
        ),
        (
            Scenario::Normal,
            "events",
            vec!["--max-frame-bytes", "8"],
            4,
            "FRAME_TOO_LARGE",
        ),
    ] {
        let out = run(scenario, &args, route).await;
        assert_eq!(
            out.status.code(),
            Some(code),
            "{finding}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert_eq!(json(&out)["findings"][0]["code"], finding);
    }
}
#[tokio::test(flavor = "multi_thread")]
async fn default_reports_exclude_query_header_and_payload() {
    let out = run(
        Scenario::Normal,
        &[
            "--header",
            "Authorization: Bearer test-secret-123",
            "--header",
            "Cookie: test-secret-cookie",
        ],
        "events?api_key=test-secret-456",
    )
    .await;
    let output = String::from_utf8(out.stdout).unwrap();
    assert!(!output.contains("test-secret"));
    assert!(!output.contains("synthetic"));
}
#[test]
fn help_version_and_bad_config_codes() {
    for args in [vec!["--help"], vec!["--version"], vec!["watch", "--help"]] {
        assert!(Command::new(env!("CARGO_BIN_EXE_streamscope"))
            .args(args)
            .output()
            .unwrap()
            .status
            .success());
    }
    for args in [
        vec!["check", "--url", "file:///tmp/test-secret"],
        vec!["check", "--url", "http://localhost", "--timeout-ms", "0"],
        vec![
            "check",
            "--url",
            "x",
            "--header",
            "test-secret",
            "--format",
            "bad",
        ],
    ] {
        let out = Command::new(env!("CARGO_BIN_EXE_streamscope"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(out.status.code(), Some(3));
        assert!(!String::from_utf8_lossy(&out.stderr).contains("test-secret"));
    }
}
#[tokio::test(flavor = "multi_thread")]
async fn cancellation_stops_an_idle_capture() {
    use std::sync::{Arc, Mutex};
    use streamscope_core::{
        capture::{self, Config},
        validate_url, State, StreamEndReason,
    };
    // A bound listener that never accepts gives a deterministic idle request.
    let listener = bind(0).await.unwrap();
    let url = validate_url(&format!("http://{}/events", listener.local_addr().unwrap())).unwrap();
    let state = Arc::new(Mutex::new(State::default()));
    let (tx, rx) = watch::channel(false);
    let config = Config {
        url,
        headers: Default::default(),
        max_gap_ms: None,
        first_event_ms: None,
        tolerance_ms: 0,
        require_last_event: None,
        timeout_ms: 30000,
        idle_timeout_ms: 10000,
        max_frame_bytes: 1048576,
    };
    let task = tokio::spawn(capture::run(config, state.clone(), rx));
    tx.send(true).unwrap();
    tokio::time::timeout(Duration::from_secs(1), task)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(state.lock().unwrap().end, StreamEndReason::Cancelled);
}
#[cfg(unix)]
#[test]
fn sigint_demo_exits_130() {
    use std::io::{BufRead, BufReader};
    let mut child = Command::new(env!("CARGO_BIN_EXE_streamscope"))
        .args(["demo", "--port", "0"])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut line = String::new();
    BufReader::new(child.stdout.take().unwrap())
        .read_line(&mut line)
        .unwrap();
    assert!(line.contains("Synthetic SSE"));
    assert!(Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap()
        .success());
    assert_eq!(child.wait().unwrap().code(), Some(130));
}

fn static_response(response: &'static [u8], args: &[&str]) -> Output {
    use std::io::{Read, Write};
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!(
        "http://{}/events?api_key=test-secret-url",
        listener.local_addr().unwrap()
    );
    let server = std::thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut request = [0; 8192];
        let _ = socket.read(&mut request).unwrap();
        socket.write_all(response).unwrap();
    });
    let out = Command::new(env!("CARGO_BIN_EXE_streamscope"))
        .args(["check", "--url", &url, "--format", "json"])
        .args(args)
        .output()
        .unwrap();
    server.join().unwrap();
    out
}

#[test]
fn raw_http_fault_taxonomy_and_response_redaction() {
    for (response, expected, finding) in [
        (&b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: 0\r\n\r\n"[..], 4, "NO_EVENTS"),
        (&b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nContent-Length: 1000\r\n\r\ndata: test-secret-body\n\n"[..], 4, "ABRUPT_CLOSE"),
        (&b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nConnection: close\r\n\r\ndata: test-secret-body"[..], 4, "INCOMPLETE_FRAME"),
        (&b"HTTP/1.1 302 Found\r\nLocation: https://example.invalid/?test-secret-redirect\r\nContent-Length: 0\r\n\r\n"[..], 4, "HTTP_STATUS"),
        (&b"HTTP/1.1 200 OK\r\nContent-Type: text/test-secret-header\r\nContent-Length: 0\r\n\r\n"[..], 4, "WRONG_CONTENT_TYPE"),
    ] {
        let out = static_response(response, &[]);
        assert_eq!(out.status.code(), Some(expected));
        assert_eq!(json(&out)["findings"][0]["code"], finding);
        assert!(!String::from_utf8_lossy(&out.stdout).contains("test-secret"));
        assert!(!String::from_utf8_lossy(&out.stderr).contains("test-secret"));
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn first_event_assertion_and_generic_eof_are_separate_contracts() {
    let out = run(Scenario::Normal, &["--first-event-ms", "1"], "events").await;
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(json(&out)["findings"][0]["code"], "FIRST_EVENT_EXCEEDED");
    let out = run(Scenario::Truncated, &[], "events").await;
    assert_eq!(out.status.code(), Some(0)); // generic SSE has no mandatory done event
}

#[test]
fn refused_connection_and_nonterminal_watch_fail_cleanly() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let url = format!("http://{}/events", listener.local_addr().unwrap());
    drop(listener);
    let out = Command::new(env!("CARGO_BIN_EXE_streamscope"))
        .args(["check", "--url", &url, "--format", "json"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(4));
    assert_eq!(json(&out)["findings"][0]["code"], "CONNECTION_FAILED");
    let out = Command::new(env!("CARGO_BIN_EXE_streamscope"))
        .args(["watch", "--url", &url])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(3));
}
