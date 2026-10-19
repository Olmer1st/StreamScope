# Executed validation

Initial full gate run on Linux x86_64, Rust 1.90.0. Commands use an isolated dependency cache because the execution sandbox restricts writes to the normal Cargo cache. The project toolchain is installed alongside the existing global toolchain.

## `CARGO_HOME=/tmp/streamscope-cargo cargo fmt --all -- --check`

Exit: 0

```text
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo clippy --locked --workspace --all-targets -- -D warnings`

Exit: 0

```text
    Checking streamscope-fixtures v0.1.0 (<checkout>/crates/streamscope-fixtures)
    Checking streamscope-core v0.1.0 (<checkout>/crates/streamscope-core)
    Checking streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
    Checking streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.56s
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo test --locked --offline --workspace`

Exit: 0

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test sigint_demo_exits_130 ... ok
test cancellation_stops_an_idle_capture ... ok
test refused_connection_and_nonterminal_watch_fail_cleanly ... ok
test help_version_and_bad_config_codes ... ok
test raw_http_fault_taxonomy_and_response_redaction ... ok
test default_reports_exclude_query_header_and_payload ... ok
test normal_and_burst_positive_controls ... ok
test first_event_assertion_and_generic_eof_are_separate_contracts ... ok
test missing_terminal_invalid_utf8_and_transport_failures ... ok
test gap_negative_control ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.57s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 5 tests
test comments_unknown_fields_and_empty_event_do_not_dispatch ... ok
test malformed_incomplete_and_oversized_are_distinct ... ok
test reports_exclude_all_secret_sources_and_terminal_escape_sequences ... ok
test utf8_line_endings_and_seeded_partitions_preserve_exact_events ... ok
test limits_and_threshold_boundaries_are_explicit ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
test tests::responsive_render_and_terminal_controls_are_safe ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
   Compiling streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.85s
     Running unittests src/main.rs (target/debug/deps/streamscope-90387c1b11771ac7)
     Running tests/end_to_end.rs (target/debug/deps/end_to_end-e721ac4394dfde59)
     Running unittests src/lib.rs (target/debug/deps/streamscope_core-a2d9850db7b0ec36)
     Running tests/protocol.rs (target/debug/deps/protocol-fa318e7bfbb234df)
     Running unittests src/lib.rs (target/debug/deps/streamscope_fixtures-d10f059fd6efc46c)
     Running unittests src/lib.rs (target/debug/deps/streamscope_tui-804910e432cbbef6)
   Doc-tests streamscope_core
   Doc-tests streamscope_fixtures
   Doc-tests streamscope_tui
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo build --locked --release --workspace`

Exit: 0

```text
    Finished `release` profile [optimized] target(s) in 0.07s
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo package --list --allow-dirty -p streamscope-core`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/capture.rs
src/lib.rs
src/sse.rs
tests/protocol.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo package --list --allow-dirty -p streamscope-fixtures`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo package --list --allow-dirty -p streamscope-tui`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```

## `CARGO_HOME=/tmp/streamscope-cargo cargo package --list --allow-dirty -p streamscope-cli`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/main.rs
tests/end_to_end.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```

# Clean-source onboarding and path installation

A source snapshot, excluding .git/target/local configuration, was copied to a new temporary directory because the repository has no commits to clone yet. No synthetic history was created. Build used a separate fresh target directory and the already-fetched dependency cache. This is a clean-source smoke, not a claimed committed clean-clone run.

## `cargo build --locked --offline --release --workspace`

Exit: 0

```text
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.27
   Compiling libc v0.2.191
   Compiling log v0.4.34
   Compiling itoa v1.0.18
   Compiling pin-project-lite v0.2.17
   Compiling futures-core v0.3.34
   Compiling bytes v1.12.1
   Compiling stable_deref_trait v1.2.1
   Compiling smallvec v1.16.3
   Compiling once_cell v1.21.4
   Compiling cfg-if v1.0.5
   Compiling equivalent v1.0.3
   Compiling slab v0.4.12
   Compiling shlex v2.0.1
   Compiling find-msvc-tools v0.1.15
   Compiling writeable v0.6.4
   Compiling percent-encoding v2.3.2
   Compiling litemap v0.8.3
   Compiling serde_core v1.0.229
   Compiling cc v1.9.0
   Compiling httparse v1.10.1
   Compiling icu_properties_data v2.3.0
   Compiling futures-task v0.3.34
   Compiling tower-service v0.3.3
   Compiling hashbrown v0.17.1
   Compiling tracing-core v0.1.36
   Compiling icu_normalizer_data v2.3.0
   Compiling futures-sink v0.3.34
   Compiling utf8_iter v1.0.4
   Compiling try-lock v0.2.5
   Compiling atomic-waker v1.1.2
   Compiling zeroize v1.9.1
   Compiling fnv v1.0.7
   Compiling want v0.3.2
   Compiling form_urlencoded v1.2.2
   Compiling sync_wrapper v1.0.2
   Compiling futures-channel v0.3.34
   Compiling rustversion v1.0.23
   Compiling httpdate v1.0.3
   Compiling tower-layer v0.3.3
   Compiling ryu v1.0.23
   Compiling rustls-pki-types v1.15.1
   Compiling serde v1.0.229
   Compiling untrusted v0.9.0
   Compiling base64 v0.23.1
   Compiling ipnet v2.12.2
   Compiling http v1.5.0
   Compiling rustls v0.23.46
   Compiling bitflags v2.13.2
   Compiling signal-hook v0.3.18
   Compiling heck v0.5.0
   Compiling tracing v0.1.44
   Compiling parking_lot_core v0.9.12
   Compiling strsim v0.11.1
   Compiling syn v3.0.9
   Compiling syn v2.0.119
   Compiling subtle v2.6.1
   Compiling ident_case v1.0.1
   Compiling rustix v0.38.44
   Compiling indexmap v2.14.2
   Compiling zmij v1.0.23
   Compiling scopeguard v1.2.0
   Compiling lock_api v0.4.14
   Compiling webpki-roots v1.0.9
   Compiling utf8parse v0.2.2
   Compiling thiserror v2.0.21
   Compiling foldhash v0.1.5
   Compiling allocator-api2 v0.2.21
   Compiling serde_json v1.0.154
   Compiling linux-raw-sys v0.4.15
   Compiling paste v1.0.15
   Compiling either v1.19.0
   Compiling anstyle-parse v1.0.0
   Compiling http-body v1.1.0
   Compiling hashbrown v0.15.5
   Compiling http-body-util v0.1.5
   Compiling castaway v0.2.4
   Compiling colorchoice v1.0.5
   Compiling itertools v0.13.0
   Compiling base64 v0.22.1
   Compiling errno v0.3.14
   Compiling mio v1.2.4
   Compiling socket2 v0.6.5
   Compiling signal-hook-registry v1.4.8
   Compiling getrandom v0.2.17
   Compiling parking_lot v0.12.5
   Compiling ring v0.17.14
   Compiling mime v0.3.17
   Compiling static_assertions v1.1.0
   Compiling anstyle v1.0.14
   Compiling signal-hook-mio v0.2.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling unicode-width v0.1.14
   Compiling anstyle-query v1.1.5
   Compiling unicode-segmentation v1.13.3
   Compiling memchr v2.8.3
   Compiling indoc v2.0.8
   Compiling anstream v1.0.0
   Compiling lru v0.12.5
   Compiling axum-core v0.5.6
   Compiling compact_str v0.8.2
   Compiling unicode-width v0.2.0
   Compiling matchit v0.8.4
   Compiling crossterm v0.28.1
   Compiling clap_lex v1.1.1
   Compiling serde_path_to_error v0.1.20
   Compiling cassowary v0.3.0
   Compiling clap_builder v4.6.7
   Compiling unicode-truncate v1.1.0
   Compiling strum_macros v0.26.4
   Compiling async-stream-impl v0.3.6
   Compiling async-stream v0.3.6
   Compiling synstructure v0.14.0
   Compiling darling_core v0.24.1
   Compiling zerofrom-derive v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling zerovec-derive v0.11.6
   Compiling displaydoc v0.2.7
   Compiling tokio-macros v2.7.2
   Compiling futures-macro v0.3.34
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling clap_derive v4.6.7
   Compiling strum v0.26.3
   Compiling tokio v1.53.3
   Compiling futures-util v0.3.34
   Compiling zerofrom v0.1.8
   Compiling yoke v0.8.3
   Compiling zerovec v0.11.8
   Compiling zerotrie v0.2.5
   Compiling darling_macro v0.24.1
   Compiling darling v0.24.1
   Compiling instability v0.3.14
   Compiling clap v4.6.7
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling icu_collections v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling ratatui v0.29.0
   Compiling icu_provider v2.3.1
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling serde_urlencoded v0.7.1
   Compiling idna_adapter v1.2.2
   Compiling idna v1.1.0
   Compiling url v2.5.8
   Compiling tokio-util v0.7.20
   Compiling tower v0.5.3
   Compiling h2 v0.4.20
   Compiling tower-http v0.6.11
   Compiling rustls-webpki v0.103.15
   Compiling tokio-rustls v0.26.6
   Compiling hyper v1.12.0
   Compiling hyper-util v0.1.21
   Compiling hyper-rustls v0.27.10
   Compiling axum v0.8.9
   Compiling reqwest v0.12.28
   Compiling streamscope-core v0.1.0 (<clean-source>/crates/streamscope-core)
   Compiling streamscope-tui v0.1.0 (<clean-source>/crates/streamscope-tui)
   Compiling streamscope-fixtures v0.1.0 (<clean-source>/crates/streamscope-fixtures)
   Compiling streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)
    Finished `release` profile [optimized] target(s) in 47.57s
```
## `cargo install --locked --offline --path crates/streamscope-cli --root /tmp/streamscope-install`

Exit: 0

```text
  Installing streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)
    Finished `release` profile [optimized] target(s) in 0.11s
  Installing /tmp/streamscope-install/bin/streamscope
   Installed package `streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)` (executable `streamscope`)
warning: be sure to add `/tmp/streamscope-install/bin` to your PATH to be able to run the installed binaries
```
## `/tmp/streamscope-install/bin/streamscope --help`

Exit: 0

```text
Inspect SSE framing and client-observed timing. No model API needed.

Usage: streamscope <COMMAND>

Commands:
  demo   Start a synthetic SSE server on localhost. No external service is used
  watch  Inspect an endpoint in a live terminal (j/k, space, q)
  check  Check declared assertions. JSON reports never contain event payloads or headers
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
## `python3 scripts/smoke.py /tmp/streamscope-install/bin/streamscope`

Exit: 0

```text
[
  {
    "scenario": "normal",
    "exitCode": 0,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:35247/events",
      "verdict": "PASS",
      "exitCode": 0,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 151,
        "minObservedGapMs": 150,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 151,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "gap",
    "exitCode": 2,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:34647/events",
      "verdict": "FAIL",
      "exitCode": 2,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 152,
        "minObservedGapMs": 152,
        "maxObservedGapMs": 4100,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "GAP_EXCEEDED",
          "eventOrdinal": 3,
          "observedMs": 4100,
          "thresholdMs": 1500
        }
      ],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "burst",
    "exitCode": 0,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:45665/events",
      "verdict": "PASS",
      "exitCode": 0,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 0,
        "firstEventMs": 0,
        "latestGapMs": 1,
        "minObservedGapMs": 1,
        "maxObservedGapMs": 1,
        "medianRecentGapMs": 1,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "truncated",
    "exitCode": 2,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:43517/events",
      "verdict": "FAIL",
      "exitCode": 2,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 3,
        "totalBytes": 129,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 150,
        "minObservedGapMs": 150,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "MISSING_LAST_EVENT"
        }
      ],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "bad-utf8",
    "exitCode": 4,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:41933/events",
      "verdict": "TRANSPORT_ERROR",
      "exitCode": 4,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 2,
        "totalBytes": 95,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 152,
        "minObservedGapMs": 152,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "INVALID_UTF8"
        }
      ],
      "endReason": "PROTOCOL",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  }
]

```

# Additional packaging and terminal checks

- `CARGO_HOME=/tmp/streamscope-cargo cargo package --locked --allow-dirty -p streamscope-core`: exit 0, packaged and verified. Warning: license metadata deliberately absent pending owner approval.
- `CARGO_HOME=/tmp/streamscope-cargo cargo package --locked --allow-dirty -p streamscope-cli`: exit 101, no matching streamscope-core package on crates.io. CLI registry packaging is HOLD until dependency publication planning; source installation above passes.
- `python3 scripts/record-demo.py target/release/streamscope docs/demo.cast`: exit 0, real normal/gap PTYs, j/freeze/resume/q, full terminal-attribute restoration and alternate-screen exit verified.
- Additional real 80x24 PTY Ctrl-C check: exit 0 from harness, application exit 130, terminal attributes restored. An initial harness attempt omitted PTY dimensions and produced no visible screen; corrected to 80x24, then passed.
- `python3 scripts/smoke.py target/release/streamscope --save docs/demo-results.json`: exit 0; normal/burst pass, gap/truncated fail expected assertions, bad UTF-8 exits 4.

# Findings investigated during final review

- One repeated full test run exited 101: `sigint_demo_exits_130` observed exit 0. This exposed a real race between graceful server completion and cancellation. Fixed synchronous Unix SIGINT registration plus cancellation-aware server return. Later full runs and targeted signal repetitions verify the fix; no sleeps were added.
- cargo-audit 0.21.2 exited 1 because the current advisory database includes CVSS 4.0. Upgraded to 0.22.2. Initial audit returned zero vulnerability entries but warnings for unmaintained paste and two unsound lru advisories. Updated Ratatui/Crossterm and removed the affected dependency chain. Final audit follows below.

# Final gate after cancellation and dependency fixes

## `CARGO_HOME=/tmp/streamscope-cargo cargo fmt --all -- --check`

Exit: 0

```text
```
## `CARGO_HOME=/tmp/streamscope-cargo cargo build --locked --workspace`

Exit: 0

```text
   Compiling ratatui-crossterm v0.1.2
   Compiling ratatui v0.30.2
   Compiling streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
   Compiling streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.00s
```
## `CARGO_HOME=/tmp/streamscope-cargo cargo clippy --locked --workspace --all-targets -- -D warnings`

Exit: 0

```text
    Checking streamscope-fixtures v0.1.0 (<checkout>/crates/streamscope-fixtures)
    Checking streamscope-core v0.1.0 (<checkout>/crates/streamscope-core)
    Checking streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
    Checking streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.57s
```
## `CARGO_HOME=/tmp/streamscope-cargo cargo test --locked --offline --workspace`

Exit: 0

```text

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 10 tests
test cancellation_stops_an_idle_capture ... ok
test sigint_demo_exits_130 ... ok
test refused_connection_and_nonterminal_watch_fail_cleanly ... ok
test help_version_and_bad_config_codes ... ok
test raw_http_fault_taxonomy_and_response_redaction ... ok
test default_reports_exclude_query_header_and_payload ... ok
test normal_and_burst_positive_controls ... ok
test first_event_assertion_and_generic_eof_are_separate_contracts ... ok
test missing_terminal_invalid_utf8_and_transport_failures ... ok
test gap_negative_control ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.57s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 6 tests
test comments_unknown_fields_and_empty_event_do_not_dispatch ... ok
test malformed_incomplete_and_oversized_are_distinct ... ok
test report_v1_contract_and_unfinished_capture_cannot_pass ... ok
test reports_exclude_all_secret_sources_and_terminal_escape_sequences ... ok
test utf8_line_endings_and_seeded_partitions_preserve_exact_events ... ok
test limits_and_threshold_boundaries_are_explicit ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 2 tests
test tests::responsive_render_and_terminal_controls_are_safe ... ok
test tests::populated_dashboard_matches_golden_snapshots ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Compiling streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
   Compiling streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.36s
     Running unittests src/main.rs (target/debug/deps/streamscope-e2a64c30a9e2397d)
     Running tests/end_to_end.rs (target/debug/deps/end_to_end-6881c7e761fc7158)
     Running unittests src/lib.rs (target/debug/deps/streamscope_core-5f90bb6e32780f54)
     Running tests/protocol.rs (target/debug/deps/protocol-f0fcde0a61218fb2)
     Running unittests src/lib.rs (target/debug/deps/streamscope_fixtures-23d64eacb82f06ea)
     Running unittests src/lib.rs (target/debug/deps/streamscope_tui-aad16081c12cee2d)
   Doc-tests streamscope_core
   Doc-tests streamscope_fixtures
   Doc-tests streamscope_tui
```
## `CARGO_HOME=/tmp/streamscope-cargo cargo build --locked --release --workspace`

Exit: 0

```text
   Compiling foldhash v0.2.0
   Compiling syn v2.0.119
   Compiling unicode-segmentation v1.13.3
   Compiling itertools v0.14.0
   Compiling rustix v1.1.5
   Compiling compact_str v0.9.1
   Compiling linux-raw-sys v0.12.1
   Compiling litrs v1.0.1
   Compiling line-clipping v0.3.8
   Compiling hashbrown v0.17.1
   Compiling convert_case v0.10.0
   Compiling document-features v0.2.12
   Compiling indexmap v2.14.2
   Compiling lru v0.18.5
   Compiling kasuari v0.4.13
   Compiling unicode-truncate v2.0.1
   Compiling h2 v0.4.20
   Compiling strum_macros v0.28.0
   Compiling derive_more-impl v2.1.1
   Compiling async-stream-impl v0.3.6
   Compiling async-stream v0.3.6
   Compiling derive_more v2.1.1
   Compiling crossterm v0.29.0
   Compiling strum v0.28.0
   Compiling ratatui-core v0.1.2
   Compiling ratatui-crossterm v0.1.2
   Compiling ratatui-widgets v0.3.2
   Compiling hyper v1.12.0
   Compiling ratatui v0.30.2
   Compiling hyper-util v0.1.21
   Compiling hyper-rustls v0.27.10
   Compiling axum v0.8.9
   Compiling reqwest v0.12.28
   Compiling streamscope-core v0.1.0 (<checkout>/crates/streamscope-core)
   Compiling streamscope-tui v0.1.0 (<checkout>/crates/streamscope-tui)
   Compiling streamscope-fixtures v0.1.0 (<checkout>/crates/streamscope-fixtures)
   Compiling streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
    Finished `release` profile [optimized] target(s) in 10.67s
```
## `CARGO_HOME=/tmp/streamscope-cargo /tmp/streamscope-tools/bin/cargo-audit audit --db /tmp/streamscope-advisory-db --no-fetch --json`

Exit: 0

```text
{"database":{"advisory-count":1338,"last-commit":null,"last-updated":null},"lockfile":{"dependency-count":231},"settings":{"target_arch":[],"target_os":[],"severity":null,"ignore":[],"informational_warnings":["unmaintained","unsound","notice"]},"vulnerabilities":{"found":false,"count":0,"list":[]},"warnings":{}}
```

Targeted SIGINT test repeated three additional times: each exit 0, application exit 130.

# Final clean-source verification after dependency and SIGINT fixes

The same isolated source directory was refreshed with final Cargo manifests/lockfile, code, scripts and snapshots; the separate build/install destinations were reused for this incremental final smoke. The initial fresh-target run above remains the clean-build evidence.

## `cargo build --locked --offline --release --workspace`

Exit: 0

```text
   Compiling foldhash v0.2.0
   Compiling syn v2.0.119
   Compiling unicode-segmentation v1.13.3
   Compiling itertools v0.14.0
   Compiling rustix v1.1.5
   Compiling compact_str v0.9.1
   Compiling linux-raw-sys v0.12.1
   Compiling litrs v1.0.1
   Compiling line-clipping v0.3.8
   Compiling hashbrown v0.17.1
   Compiling convert_case v0.10.0
   Compiling document-features v0.2.12
   Compiling indexmap v2.14.2
   Compiling lru v0.18.5
   Compiling kasuari v0.4.13
   Compiling unicode-truncate v2.0.1
   Compiling h2 v0.4.20
   Compiling strum_macros v0.28.0
   Compiling derive_more-impl v2.1.1
   Compiling async-stream-impl v0.3.6
   Compiling async-stream v0.3.6
   Compiling derive_more v2.1.1
   Compiling crossterm v0.29.0
   Compiling strum v0.28.0
   Compiling ratatui-core v0.1.2
   Compiling ratatui-widgets v0.3.2
   Compiling ratatui-crossterm v0.1.2
   Compiling hyper v1.12.0
   Compiling ratatui v0.30.2
   Compiling hyper-util v0.1.21
   Compiling hyper-rustls v0.27.10
   Compiling axum v0.8.9
   Compiling reqwest v0.12.28
   Compiling streamscope-core v0.1.0 (<clean-source>/crates/streamscope-core)
   Compiling streamscope-tui v0.1.0 (<clean-source>/crates/streamscope-tui)
   Compiling streamscope-fixtures v0.1.0 (<clean-source>/crates/streamscope-fixtures)
   Compiling streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)
    Finished `release` profile [optimized] target(s) in 11.55s
```
## `cargo install --locked --offline --force --path crates/streamscope-cli --root /tmp/streamscope-install`

Exit: 0

```text
  Installing streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)
    Finished `release` profile [optimized] target(s) in 0.10s
   Replacing /tmp/streamscope-install/bin/streamscope
    Replaced package `streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)` with `streamscope-cli v0.1.0 (<clean-source>/crates/streamscope-cli)` (executable `streamscope`)
warning: be sure to add `/tmp/streamscope-install/bin` to your PATH to be able to run the installed binaries
```
## `python3 scripts/smoke.py /tmp/streamscope-install/bin/streamscope`

Exit: 0

```text
[
  {
    "scenario": "normal",
    "exitCode": 0,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:39683/events",
      "verdict": "PASS",
      "exitCode": 0,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 151,
        "minObservedGapMs": 151,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 151,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "gap",
    "exitCode": 2,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:42177/events",
      "verdict": "FAIL",
      "exitCode": 2,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 150,
        "minObservedGapMs": 150,
        "maxObservedGapMs": 4101,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "GAP_EXCEEDED",
          "eventOrdinal": 3,
          "observedMs": 4101,
          "thresholdMs": 1500
        }
      ],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "burst",
    "exitCode": 0,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:41823/events",
      "verdict": "PASS",
      "exitCode": 0,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 4,
        "totalBytes": 171,
        "headersMs": 0,
        "firstByteMs": 1,
        "firstEventMs": 1,
        "latestGapMs": 1,
        "minObservedGapMs": 1,
        "maxObservedGapMs": 1,
        "medianRecentGapMs": 1,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "truncated",
    "exitCode": 2,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:42191/events",
      "verdict": "FAIL",
      "exitCode": 2,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 3,
        "totalBytes": 129,
        "headersMs": 0,
        "firstByteMs": 150,
        "firstEventMs": 150,
        "latestGapMs": 151,
        "minObservedGapMs": 151,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "MISSING_LAST_EVENT"
        }
      ],
      "endReason": "EOF",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  },
  {
    "scenario": "bad-utf8",
    "exitCode": 4,
    "report": {
      "schemaVersion": "streamscope-report/v1",
      "mode": "check",
      "target": "http://127.0.0.1:34605/events",
      "verdict": "TRANSPORT_ERROR",
      "exitCode": 4,
      "transport": {
        "httpStatus": 200,
        "contentType": "text/event-stream"
      },
      "metrics": {
        "eventCount": 2,
        "totalBytes": 95,
        "headersMs": 0,
        "firstByteMs": 151,
        "firstEventMs": 151,
        "latestGapMs": 152,
        "minObservedGapMs": 152,
        "maxObservedGapMs": 152,
        "medianRecentGapMs": 152,
        "comments": 0,
        "evictedEvents": 0,
        "droppedFindings": 0
      },
      "findings": [
        {
          "code": "INVALID_UTF8"
        }
      ],
      "endReason": "PROTOCOL",
      "notes": [
        "Timings are client observations, not server execution timings",
        "Median uses at most 2048 recent inter-event gaps; payloads and headers are excluded"
      ]
    }
  }
]

```

Final-build terminal recording regenerated successfully after dependency update.

Advisory database revision used: `999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b`. Final scan: cargo-audit 0.22.2, zero vulnerability entries and zero warnings.

Exact README cargo-run commands for normal demo, JSON check, and real 80x24 PTY watch executed successfully (application exit 0); Ctrl-C stopped the server. Default Cargo install destination was not modified; installation was verified under an isolated root.

# Final package inventories and tarball verification

## `cargo package --locked --offline --list --allow-dirty -p streamscope-core`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/capture.rs
src/lib.rs
src/sse.rs
tests/protocol.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```
## `cargo package --locked --offline --list --allow-dirty -p streamscope-fixtures`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```
## `cargo package --locked --offline --list --allow-dirty -p streamscope-tui`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/lib.rs
tests/snapshots/dashboard-120x40.txt
tests/snapshots/dashboard-80x24.txt
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```
## `cargo package --locked --offline --list --allow-dirty -p streamscope-cli`

Exit: 0

```text
Cargo.lock
Cargo.toml
Cargo.toml.orig
src/main.rs
tests/end_to_end.rs
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
```
## `cargo package --locked --offline --allow-dirty -p streamscope-core`

Exit: 0

```text
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging streamscope-core v0.1.0 (<checkout>/crates/streamscope-core)
    Packaged 7 files, 62.0KiB (17.7KiB compressed)
   Verifying streamscope-core v0.1.0 (<checkout>/crates/streamscope-core)
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.27
   Compiling quote v1.0.47
   Compiling libc v0.2.191
   Compiling stable_deref_trait v1.2.1
   Compiling pin-project-lite v0.2.17
   Compiling bytes v1.12.1
   Compiling futures-core v0.3.34
   Compiling find-msvc-tools v0.1.15
   Compiling itoa v1.0.18
   Compiling shlex v2.0.1
   Compiling writeable v0.6.4
   Compiling smallvec v1.16.3
   Compiling litemap v0.8.3
   Compiling once_cell v1.21.4
   Compiling utf8_iter v1.0.4
   Compiling icu_properties_data v2.3.0
   Compiling slab v0.4.12
   Compiling icu_normalizer_data v2.3.0
   Compiling percent-encoding v2.3.2
   Compiling cc v1.9.0
   Compiling cfg-if v1.0.5
   Compiling zeroize v1.9.1
   Compiling tracing-core v0.1.36
   Compiling futures-task v0.3.34
   Compiling untrusted v0.9.0
   Compiling httparse v1.10.1
   Compiling futures-sink v0.3.34
   Compiling equivalent v1.0.3
   Compiling hashbrown v0.17.1
   Compiling try-lock v0.2.5
   Compiling fnv v1.0.7
   Compiling atomic-waker v1.1.2
   Compiling tower-service v0.3.3
   Compiling rustls v0.23.46
   Compiling rustls-pki-types v1.15.1
   Compiling serde_core v1.0.229
   Compiling want v0.3.2
   Compiling form_urlencoded v1.2.2
   Compiling futures-channel v0.3.34
   Compiling subtle v2.6.1
   Compiling serde v1.0.229
   Compiling sync_wrapper v1.0.2
   Compiling base64 v0.23.1
   Compiling ipnet v2.12.2
   Compiling tower-layer v0.3.3
   Compiling bitflags v2.13.2
   Compiling thiserror v2.0.21
   Compiling ryu v1.0.23
   Compiling http v1.5.0
   Compiling tracing v0.1.44
   Compiling webpki-roots v1.0.9
   Compiling log v0.4.34
   Compiling base64 v0.22.1
   Compiling indexmap v2.14.2
   Compiling syn v3.0.9
   Compiling http-body v1.1.0
   Compiling http-body-util v0.1.5
   Compiling ring v0.17.14
   Compiling errno v0.3.14
   Compiling mio v1.2.4
   Compiling socket2 v0.6.5
   Compiling getrandom v0.2.17
   Compiling signal-hook-registry v1.4.8
   Compiling synstructure v0.14.0
   Compiling rustls-webpki v0.103.15
   Compiling zerofrom-derive v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling zerovec-derive v0.11.6
   Compiling displaydoc v0.2.7
   Compiling tokio-macros v2.7.2
   Compiling futures-macro v0.3.34
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling futures-util v0.3.34
   Compiling tokio v1.53.3
   Compiling zerofrom v0.1.8
   Compiling yoke v0.8.3
   Compiling zerovec v0.11.8
   Compiling zerotrie v0.2.5
   Compiling tinystr v0.8.4
   Compiling potential_utf v0.1.6
   Compiling icu_collections v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling icu_provider v2.3.1
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling serde_urlencoded v0.7.1
   Compiling idna_adapter v1.2.2
   Compiling idna v1.1.0
   Compiling url v2.5.8
   Compiling tokio-util v0.7.20
   Compiling tower v0.5.3
   Compiling tokio-rustls v0.26.6
   Compiling tower-http v0.6.11
   Compiling h2 v0.4.20
   Compiling hyper v1.12.0
   Compiling hyper-util v0.1.21
   Compiling hyper-rustls v0.27.10
   Compiling reqwest v0.12.28
   Compiling streamscope-core v0.1.0 (<checkout>/target/package/streamscope-core-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 10.09s
```
## `cargo package --locked --offline --allow-dirty -p streamscope-cli`

Exit: 101

```text
warning: manifest has no license or license-file.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)
error: failed to prepare local package for uploading

Caused by:
  no matching package named `streamscope-core` found
  location searched: crates.io index
  required by package `streamscope-cli v0.1.0 (<checkout>/crates/streamscope-cli)`
  As a reminder, you're using offline mode (--offline) which can sometimes cause surprising resolution failures, if this error is too confusing you may wish to retry without `--offline`.
```

Final source/artifact scan: 51 untracked deliverable files reviewed for GitHub-token, AWS-key and private-key patterns; no matches. Saved reports/cast contain no test-secret sentinel or machine home path. Git history has zero commits. YAML workflow/issue forms parsed successfully; local Markdown links resolved. This pattern scan is not a guarantee against every possible secret.
