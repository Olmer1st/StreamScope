# Troubleshooting

- Compiler older than 1.90: run `rustup toolchain install 1.90.0 --profile minimal --component rustfmt --component clippy`. The checked-in toolchain file selects it only inside this project; the global default need not change.
- Network required on first build: fetch dependencies once with `cargo fetch --locked`. Afterwards tests can run with `--offline`.
- Port busy: use `demo --port 0` and copy the printed URL, or choose an unused port.
- watch says terminal required: run in a terminal or use `check --format json` in CI.
- Terminal damaged after forced termination: run `reset`. q/Ctrl-C, unwinding panic, and ordinary errors restore raw/alternate-screen state. SIGKILL cannot run cleanup.
- HTTP_STATUS / WRONG_CONTENT_TYPE: verify your endpoint returns a 2xx SSE response. Redirects are intentionally not followed.
- INCOMPLETE_FRAME: a final block lacked its blank line. A clean EOF without a done event fails only when that contract is requested.
- READ_TIMEOUT: increase `--idle-timeout-ms` for intentionally sparse streams; `--timeout-ms` still bounds total time.
- GAP_EXCEEDED: inspect synthetic reproduction, threshold and tolerance. This is not proof of model or server delay.
- CONNECTION_FAILED: check the host, port, network and TLS certificate. No insecure bypass is provided and sensitive transport errors are not echoed.
- Missing old events: the display retains 256; inspect evicted counter. Aggregate counts and max gap cover the full capture.
