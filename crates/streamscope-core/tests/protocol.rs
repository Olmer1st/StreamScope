use streamscope_core::{
    sse::{Decoder, Event, ParseError},
    *,
};
fn decode(bytes: &[u8], chunks: &[usize]) -> Result<Vec<Event>, ParseError> {
    let mut parser = Decoder::new(1024 * 1024);
    let mut events = vec![];
    let mut offset = 0;
    for size in chunks.iter().copied().cycle() {
        if offset == bytes.len() {
            break;
        }
        let end = (offset + size).min(bytes.len());
        for &b in &bytes[offset..end] {
            if let Some(event) = parser.push(b)? {
                events.push(event);
            }
        }
        offset = end;
    }
    parser.finish()?;
    Ok(events)
}
#[test]
fn utf8_line_endings_and_seeded_partitions_preserve_exact_events() {
    for ending in ["\n", "\r\n", "\r"] {
        let input = format!("\u{feff}: heartbeat{ending}id: 7{ending}event: delta{ending}data: Привет 🦀{ending}data: two{ending}{ending}data{ending}{ending}id: invalid\0id{ending}data: final{ending}{ending}");
        let expected = decode(input.as_bytes(), &[input.len()]).unwrap();
        assert_eq!(expected.len(), 3);
        assert_eq!(expected[0].data, "Привет 🦀\ntwo");
        assert_eq!(expected[0].event_type, "delta");
        assert_eq!(expected[1].data, "");
        assert_eq!(expected[1].event_type, "message");
        assert_eq!(expected[2].id, "7");
        for boundary in 1..input.len() {
            assert_eq!(decode(input.as_bytes(), &[boundary, 1]).unwrap(), expected);
        }
        for seed in 1..=64u64 {
            let mut rng = seed;
            let sizes: Vec<_> = (0..32)
                .map(|_| {
                    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                    (rng % 17 + 1) as usize
                })
                .collect();
            assert_eq!(decode(input.as_bytes(), &sizes).unwrap(), expected);
        }
        assert_eq!(decode(input.as_bytes(), &[1]).unwrap(), expected);
    }
}
#[test]
fn comments_unknown_fields_and_empty_event_do_not_dispatch() {
    assert!(
        decode(b": heartbeat\n\nunknown: ignored\nevent:\nid:\n\n", &[1])
            .unwrap()
            .is_empty()
    );
}
#[test]
fn malformed_incomplete_and_oversized_are_distinct() {
    assert_eq!(
        decode(b"data: \xff\n\n", &[1]),
        Err(ParseError::InvalidUtf8)
    );
    assert_eq!(
        decode(b"data: pending\n", &[1]),
        Err(ParseError::IncompleteFrame)
    );
    assert_eq!(decode(b"data: \xf0", &[1]), Err(ParseError::InvalidUtf8));
    let mut parser = Decoder::new(8);
    for b in b"data: ok" {
        assert!(parser.push(*b).is_ok());
    }
    assert_eq!(parser.push(b'x'), Err(ParseError::FrameTooLarge));
    assert!(decode(b"data: ok\n\n", &[1]).is_ok());
}
#[test]
fn limits_and_threshold_boundaries_are_explicit() {
    let mut state = State::default();
    for n in 0..10000 {
        state.event(
            Event {
                data: "x".repeat(2000),
                id: String::new(),
                event_type: "delta".into(),
                frame_bytes: 2010,
            },
            n,
            Some(1),
            0,
        );
    }
    assert_eq!(state.events.len(), EVENT_CAP);
    assert_eq!(state.metrics.evicted_events, 10000 - EVENT_CAP as u64);
    assert_eq!(state.events.back().unwrap().data.len(), 512);
    assert!(!state.assertion_failed); // equality passes
    state.event(
        Event {
            data: String::new(),
            id: String::new(),
            event_type: "done".into(),
            frame_bytes: 1,
        },
        10002,
        Some(1),
        1,
    );
    assert!(state.assertion_failed); // 3 > 1+1
    assert_eq!(state.metrics().median_recent_gap_ms, Some(1));
}
#[test]
fn reports_exclude_all_secret_sources_and_terminal_escape_sequences() {
    let url =
        validate_url("https://example.invalid/events?api_key=test-secret-456#test-secret-000")
            .unwrap();
    let mut state = State::default();
    state.event(
        Event {
            data: "test-secret-789".into(),
            id: "test-secret-id".into(),
            event_type: "test-secret-type".into(),
            frame_bytes: 30,
        },
        1,
        None,
        0,
    );
    state.end = StreamEndReason::Eof;
    let json = serde_json::to_string(&report(&state, &url)).unwrap();
    assert!(!json.contains("test-secret"));
    assert_eq!(
        redact_header("Authorization", "Bearer test-secret-123"),
        "[REDACTED]"
    );
    assert_eq!(redact_header("Cookie", "test-secret-cookie"), "[REDACTED]");
    assert!(!safe_preview("\x1b[31m\x07test", 512).contains('\x1b'));
    assert!(validate_url("file:///etc/passwd").is_err());
    assert!(validate_url("https://user:secret@example.invalid/").is_err());
    assert!(capture::parse_header("Authorization: x\r\nInjected: true").is_err());
}

#[test]
fn report_v1_contract_and_unfinished_capture_cannot_pass() {
    let url = validate_url("http://localhost:8383/events?secret=hidden").unwrap();
    let mut state = State::default();
    assert_eq!(report(&state, &url).verdict, "INCONCLUSIVE");
    state.event(
        Event {
            event_type: "done".into(),
            id: "1".into(),
            data: "never serialized".into(),
            frame_bytes: 42,
        },
        150,
        None,
        0,
    );
    assert_eq!(report(&state, &url).exit_code, 4);
    state.http_status = Some(200);
    state.valid_content_type = true;
    state.metrics.total_bytes = 42;
    state.metrics.headers_ms = Some(10);
    state.metrics.first_byte_ms = Some(100);
    state.end = StreamEndReason::Eof;
    assert_eq!(
        serde_json::to_value(report(&state, &url)).unwrap(),
        serde_json::json!({
            "schemaVersion": "streamscope-report/v1", "mode": "check", "target": "http://localhost:8383/events",
            "verdict": "PASS", "exitCode": 0, "transport": { "httpStatus": 200, "contentType": "text/event-stream" },
            "metrics": { "eventCount": 1, "totalBytes": 42, "headersMs": 10, "firstByteMs": 100, "firstEventMs": 150,
                "latestGapMs": null, "minObservedGapMs": null, "maxObservedGapMs": null, "medianRecentGapMs": null,
                "comments": 0, "evictedEvents": 0, "droppedFindings": 0 },
            "findings": [], "endReason": "EOF", "notes": [
                "Timings are client observations, not server execution timings",
                "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
            ]
        })
    );
}
