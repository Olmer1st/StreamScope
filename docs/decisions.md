# Decisions

## 002 — Bounded state instead of event queue

Use a bounded ring under a short-lived Mutex plus watch cancellation. A queued per-event channel would apply backpressure or require redundant overflow policies. The single shared bounded model coalesces rendering without losing aggregate counts and keeps freeze independent from capture. See architecture.md. Rust lesson: explicit ownership and no lock across awaits.

## 003 — Strict diagnostics and byte accounting

Invalid UTF-8 and unterminated blocks fail rather than applying browser replacement/discard rules. CRLF is one normalized parser delimiter; totalBytes retains wire-body byte accounting. Record these deviations explicitly in protocol.md. Synchronous byte consumption makes arbitrary read partitioning testable. Default limits cap frame state, previews, gaps, and findings.

## 004 — Privacy and output taxonomy

Serialize allowlisted report fields, never internal event structs or raw errors. Redact all header values rather than relying on a secret-name denylist. Hide query, fragment, and credentials; reject userinfo. Do not follow redirects. Exit 4 groups protocol/runtime failures; finding codes preserve distinctions. Empty streams are inconclusive. Thresholds are strict greater-than plus explicit tolerance. Rust lesson: typed contracts at the CLI boundary.

## 005 — Reproducible tooling

Pin Rust 1.90.0 and Cargo.lock, use synthetic fixed-delay fixtures, and keep four small crates. Registry publication remains disabled pending licensing and release preparation.

## 006 — Honest release hold

No license is selected without the owner's choice. No fake LICENSE, SPDX, release, hosted CI result, or platform validation. Keep source and dependency inventories available locally; clean archive onboarding can be tested before any genuine initial commit exists. Public pushes, tags, and publication require explicit scope authorization. Licensing, private disclosure, and macOS validation remain release gates until verified.

## 007 — Original conduct policy

Use a short original conduct policy to avoid implying adoption of an established enforcement process with no private contact configured. This intentionally deviates from the suggested established-text approach; the behavioral expectations are present and private reporting remains an explicit hold.

## 008 — Advisory-driven terminal dependency update and SIGINT race

The initial Ratatui 0.29 dependency graph included unmaintained paste and unsound lru 0.12.5 (RUSTSEC-2024-0436, RUSTSEC-2026-0002, RUSTSEC-2026-0253). Updated to Ratatui 0.30.2 with only the Crossterm backend enabled and matched Crossterm 0.29 to avoid duplicate backend versions. The updated graph passes the current advisory database scan.

A repeated full gate exposed a real demo cancellation race: graceful server completion could win the select and return 0 instead of 130. Register Unix SIGINT synchronously before readiness and check the cancellation state when the server completes. The existing signal test caught the failure; rerun it after the fix. No arbitrary startup sleeps were added.
