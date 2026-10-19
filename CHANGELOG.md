# Changelog

## Unreleased — proposed 0.1.0

- Tag-triggered GitHub releases reuse Linux/macOS checks, package native binaries and license notices, verify checksums, and publish after both platforms pass.
- Rust workspace with core decoder, CLI, loopback fixtures, and terminal dashboard.
- Incremental SSE handling independent of transport read boundaries; explicit strict UTF-8, frame limits, and EOF diagnostics.
- Synthetic normal, gap, burst, truncated, and invalid-UTF-8 scenarios.
- Client-observed first-byte/first-event/gap metrics and bounded display state.
- Timing and terminal-event checks, report/v1, safe output defaults, cancellation, and explicit exit codes.
- Contributor documentation, CI configuration, authentic demo evidence, and round execution records.

No release, tag, or crates.io publication is claimed. Existing hosted Linux/macOS checks pass; the release workflow awaits its first tag. The project is licensed under Apache-2.0; release preparation remains pending.
