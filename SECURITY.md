# Security

Only the current development branch is evaluated; no released version is supported yet. StreamScope does not intercept traffic, disable TLS validation, or collect telemetry. Treat live terminal previews as sensitive. Default reports exclude event bodies, header values, URL query/fragment, and embedded credentials. URL paths are still visible.

Private reporting route: NOT_CONFIGURED until verified and selected by the maintainer. Do not include suspected vulnerabilities, credentials, or exploit details in public issues. The maintainer must enable and verify GitHub private vulnerability reporting (or designate a private channel) before inviting disclosures. No private email, response SLA, or bounty is promised.

Dependencies are locked. Review Cargo.lock changes, registry advisories, and package licenses before release; run `cargo audit` if installed. CI uses a read-only GitHub token and no user secrets. Reports deliberately omit raw networking error strings because they can contain URLs.
