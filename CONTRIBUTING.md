# Contributing

This is a prototype licensed under the [Apache License, Version 2.0](LICENSE). Contributions are submitted under the same license. No CLA or DCO is required or adopted.

Use Rust 1.90.0 and Cargo. Run the README demo, format check, Clippy with warnings denied, workspace tests, and release build before proposing a change. Tests are entirely synthetic and loopback-only. Include meaningful positive and negative controls when behavior changes, and explain privacy, compatibility, and dependency impacts. Keep core parsing independent of presentation and fixtures.

Discuss substantial changes in an issue first. Bug reports should include version, OS/terminal, a synthetic reproduction, expected/observed behavior, and sanitized diagnostics. Never post credentials or private stream content. Suspected vulnerabilities follow SECURITY.md, not public issues. Submit a focused PR against main, complete the template, and expect maintainer review without a response-time guarantee. Do not change legal ownership or rewrite history.

Update CHANGELOG.md, user documentation, and docs/decisions.md when a contract changes. Before 1.0, incompatible changes are possible but must be documented.
