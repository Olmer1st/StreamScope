# Validation record

Implementation was validated on Linux x86_64 with Rust 1.90.0. These are recorded implementation results, not a claim that hosted CI or other platforms have passed.

| Check | Recorded result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo build --locked --workspace` | PASS |
| `cargo clippy --locked --workspace --all-targets -- -D warnings` | PASS |
| `cargo test --locked --offline --workspace` | PASS: 18 tests |
| `cargo build --locked --release --workspace` | PASS |
| cargo-audit 0.22.2 | Zero vulnerability entries or warnings after dependency updates |
| Separate source snapshot, release build and path installation | PASS |
| Release binary scenario matrix | Expected exits: normal/burst 0, gap/truncated 2, invalid UTF-8 4 |
| Real terminal recording | Navigation, freeze/resume, quit and terminal restoration passed |
| Real 80x24 terminal Ctrl-C | Application exit 130; terminal attributes restored |
| Golden terminal rendering | 80x24 and 120x40 snapshots passed |
| Package contents | Inspected for all four crates |
| Core crate packaging and verification | PASS |
| CLI crate packaging | BLOCKED: internal dependency is not published to crates.io |
| Workflow and issue-form YAML | Parsed successfully |
| Local Markdown links | Resolved |
| Source and saved artifacts | No matches for scanned token/private-key patterns; synthetic secret sentinels excluded from saved reports/recording |

Tests cover SSE framing across byte partitions, UTF-8, malformed and oversized frames, bounded retention, timing thresholds, report privacy, HTTP failures, cancellation, and CLI exit codes. Dependency installation requires registry access; offline tests assume dependencies have already been fetched.

Evidence: [captured reports](demo-results.json), [terminal recording](demo.cast), [protocol tests](../crates/streamscope-core/tests/protocol.rs), [CLI integration tests](../crates/streamscope-cli/tests/end_to_end.rs), and [terminal snapshots](../crates/streamscope-tui/tests/snapshots/).

## Findings resolved during validation

- A SIGINT race could return 0 instead of 130 when graceful server completion won cancellation. Registering Unix SIGINT before readiness and checking cancellation on server completion fixed it. The regression test and three additional targeted repetitions passed.
- Ratatui 0.29 brought unmaintained paste and unsound lru versions. Updating Ratatui/Crossterm removed those warnings. The final advisory database revision was `999ec89d5d3f10f20be14750ec8c2d8b44e0ac9b`.

## Remaining release checks

Hosted Linux/macOS Rust checks passed at commit 22ee4bf; the new release workflow has not yet run on GitHub. Windows is untested. Clean-source onboarding used a source snapshot rather than a committed clone. A private security-reporting route and internal crate publication planning remain pending. A secret-pattern scan is limited evidence, not a guarantee against every possible secret.

## Release workflow validation

- actionlint 1.7.12: workflow syntax and expressions passed.
- Native Linux archive creation and extracted-binary help plus all five synthetic scenarios passed. Root license, upstream notice inventory, target/version metadata and SHA-256 sidecar were included.
- Fifteen release controls passed: exact version matching; invalid, mismatched and injected tag rejection; stable/prerelease outputs; two-platform assembly; and rejection of missing/duplicate platforms, corrupted checksums, wrong executable architecture, unsafe archive paths, and mismatched metadata. The second-platform assembly fixture was synthetic and never executed/distributed; it is not a macOS build claim.
- Version validation passed with an empty Cargo cache. Existing 18 Rust tests and formatting checks passed after workflow changes.
- Security risk: pushing a reviewed version tag authorizes automated publication. Read-only builds are separated from the contents-write publisher, actions are SHA-pinned, archives are checked without executing downloaded candidates in the publisher, and partial uploads remain drafts. Checksums are not release signatures.
- No version tag, remote release, commit, or push was created by this validation. End-to-end GitHub publication and native macOS packaging remain to be verified on the first release run.
