# CLI and report/v1 contract

`streamscope demo --scenario normal|gap|burst|truncated|bad-utf8 --port 8383` binds IPv4 loopback only. Port zero selects a free port and prints it. Ctrl-C stops the server (exit 130).

`streamscope watch --url URL` requires an interactive stdin/stdout and shows the live TUI. j/k/arrows move selection; space snapshots the view while capture continues; q cancels and exits 0; Ctrl-C cancels and exits 130. Completed or failed captures stay visible until quit. The current bounded display ring is 256 events; previews cap at 512 Unicode characters; event names cap at 64 characters. The scrollbar reflects retained events, not total historical events. Frozen displays retain their snapshot metrics; the eviction counter stays live.

`streamscope check --url URL [assertions] --format text|json` runs without a TTY. `probe` aliases `check` for scaffold compatibility. `--help`, subcommand `--help`, and `--version` return 0.

| Flag | Meaning/default |
|---|---|
| --max-gap-ms N | Fail when an inter-event client interval exceeds N + tolerance |
| --first-event-ms N | Fail when first complete event exceeds N + tolerance, including connect time |
| --require-last-event NAME | Require final dispatched event type; generic streams need no `done` |
| --timeout-ms N | Overall request deadline, default 30000, positive |
| --idle-timeout-ms N | Maximum wait for next body chunk, default 10000, positive |
| --tolerance-ms N | Added timing tolerance, default 0; equality passes |
| --max-frame-bytes N | 1..16777216; default 1048576 |
| --header 'NAME: VALUE' | Repeatable request header; rejected on invalid syntax/controls |
| --format text\|json | Human summary or report/v1, default text |

Header arguments can appear in shell history/process listings. Redirects are disabled. Only HTTP(S) with a host and no embedded user/password is accepted. TLS verification stays enabled. Unknown flags/configuration errors return fixed sanitized diagnostics without echoing arguments.

| Exit | Meaning |
|---|---|
| 0 | PASS, help/version, or deliberate TUI q |
| 2 | Assertion FAIL |
| 3 | INVALID_INPUT (including watch without a TTY) |
| 4 | Transport/protocol/runtime failure, or empty-stream INCONCLUSIVE |
| 130 | User interruption |

JSON includes `schemaVersion: streamscope-report/v1`, `mode`, sanitized `target`, `verdict`, `exitCode`, `transport`, `metrics`, bounded `findings`, `endReason`, and `notes`. Metrics include total bytes, observed header/first-byte/first-event times, event count, min/max/latest gap, upper-middle median of at most 2048 recent gaps, comments and evictions. Optional unavailable metrics are null. Findings use stable codes; up to 64 are retained and excess counts appear in `droppedFindings`. `observedMs` and `thresholdMs` are numeric evidence; event ordinals start at 1. Empty streams cannot pass. Interrupted/error captures take precedence over assertion verdicts.

Codes: GAP_EXCEEDED, FIRST_EVENT_EXCEEDED, MISSING_LAST_EVENT, NO_EVENTS, INVALID_UTF8, FRAME_TOO_LARGE, INCOMPLETE_FRAME, CONNECTION_FAILED, CLIENT_INIT_FAILED, HTTP_STATUS, WRONG_CONTENT_TYPE, ABRUPT_CLOSE, READ_TIMEOUT, TIMEOUT, INTERRUPTED. A clean EOF is not evidence of abrupt disconnection; `--require-last-event` detects a missing application-level terminator. Non-2xx and redirects produce HTTP_STATUS. Invalid content-type values are never echoed.

Reports omit event data/IDs/types, all headers, response error bodies, query and fragment. Only canonical `text/event-stream` and numeric status are recorded, never arbitrary header values. Paths may contain sensitive data; sanitize paths before sharing. No report files are written by default. Before 1.0 public APIs may change; report schema incompatibilities require a version marker and changelog entry.

## Consumer CI example

Build with `cargo build --locked --release --workspace`, start `target/release/streamscope demo --scenario normal` as a managed background process, then run:

```sh
target/release/streamscope check --url http://127.0.0.1:8383/events --max-gap-ms 1500 --require-last-event done --format json
```

Always stop/wait for the background process. For an executable example that manages readiness and cleanup, run `python3 scripts/smoke.py target/release/streamscope`.
