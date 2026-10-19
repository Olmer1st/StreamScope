# Proposed 0.1.0 release notes — NOT RELEASED

StreamScope inspects SSE framing and client-observed event timing using a live terminal dashboard or headless assertions. It includes fully local synthetic fixtures and bounded memory policies. No model API keys or account are needed to evaluate it.

Compatibility: Linux validated locally with Rust 1.90.0; macOS is an unverified CI target; Windows untested. Parser is deliberately strict on UTF-8 and unfinished EOF blocks. No reconnect, NDJSON, HTML export, traffic interception, or server-side causal diagnosis. Report/v1 omits bodies, arbitrary headers, URL query and fragment; URL paths remain visible.

Publication hold: maintainer license choice, private disclosure mechanism, hosted CI and platform checks. The CLI package dry run cannot resolve unpublished streamscope-core on crates.io. Before a registry release, publish the internal crates in dependency order (core and fixtures, then TUI, then CLI), or deliberately redesign the distribution as one package; do not claim the CLI tarball is standalone today. Workspace path installation is verified. No tag, release assets, checksums, downloads, adoption, or publication claimed. Before publishing, review package inventory and dependency licenses/advisories, approve legal terms, repeat clean-install demo, and authorize the exact push/tag/release.

If an eventual release is defective, document the issue, publish a corrective version, and consider yanking an affected crate after maintainer review. Never rewrite shared history or promise registry deletion. Signing/binary distribution automation is deferred.
