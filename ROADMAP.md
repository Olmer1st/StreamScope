# Roadmap

## Release preparation
- Validate the first tag-triggered release end to end; Linux/macOS Rust checks already pass in hosted CI.
- Configure/verify a private security-reporting route.
- Review authentic terminal recording and package inventory before authorizing publication.
- Measure real-world usefulness from voluntary, sanitized reports; no adoption claimed.

## After the SSE MVP
- SS-011: compare saved metrics without pretending payload semantics match.
- SS-012: explicit NDJSON decoder.
- SS-013: local static HTML timeline.
- SS-014: controlled local disconnect/reconnect experiments.
- SS-015: explicit external timing markers, never inferred server execution time.

No MCP inspector, proxy interception, hosted SaaS, model SDK, or production traffic replay is planned for 0.1.
