StreamScope is a local-first SSE debugger for inspecting framing and client-observed event timing. This initial version is intended for early feedback; the CLI and report contracts may evolve before 1.0.

### Included

- Synthetic local normal, gap, burst, truncated, and invalid-UTF-8 streams.
- Live terminal timeline with navigation, freeze/resume, and bounded event previews.
- Headless timing/terminal-event assertions and sanitized report/v1 JSON.
- Native Linux/macOS archives with Apache-2.0 and third-party notices.

### Download and run

Choose the archive whose Rust target matches your operating system and architecture. Verify it against `SHA256SUMS`, extract it, and run `./streamscope --help` inside its directory. Run `./streamscope demo --scenario gap`, then use another terminal:

```sh
./streamscope watch --url http://127.0.0.1:8383/events
```

For CI, replace `watch` with `check` and add `--max-gap-ms 1500 --require-last-event done --format json`. The synthetic gap deliberately exits 2.

### Limitations

Timings are client observations, not proof of server/model execution time. No automatic reconnect, provider-specific parsing, NDJSON, or Windows support. Reports omit payloads, headers, URL query and fragment; URL paths remain visible. Live terminal previews can contain sensitive data.

Linux binaries require a compatible glibc/runtime; they are not statically linked portable binaries. macOS binaries are unsigned and not notarized. Only the architectures named in the assets are provided. Checksums detect corruption and are not cryptographic release signatures. Native builds, tests and packaged-binary smoke checks must pass before this release is published.

crates.io installation is not provided by this release. Source builds remain available through the repository instructions.
