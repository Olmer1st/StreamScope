# Releasing with GitHub Actions

The release workflow runs only when a tag matching `v*` is pushed. Ordinary branch pushes and pull requests run CI without publishing. The tagged commit must belong to `main`; tags must exactly match all workspace package versions. Accepted names are `vMAJOR.MINOR.PATCH` with an optional prerelease suffix; build-metadata suffixes are not supported. The workflow reuses the Rust checks on Linux and macOS, smoke-tests packaged binaries, validates both candidates, then publishes a complete GitHub release. A failed gate prevents publication.

## Prepare a version

1. Finish the changelog and edit `docs/github-release-notes.md` for the release. Verify the security reporting route and review redistributable assets/notices.
2. Set `workspace.package.version` in Cargo.toml and the internal path dependency version requirements to the intended version. Refresh Cargo.lock and run the normal CI checks. For `v0.1.0-alpha.1`, these versions must be `0.1.0-alpha.1`; the manifests must match before the tag is pushed.
3. Commit and push the reviewed source to main. Create and push the matching tag only when ready to publish. For example, after version preparation:

```sh
git tag -a v0.1.0-alpha.1 -m "StreamScope v0.1.0-alpha.1"
git push origin v0.1.0-alpha.1
```

These are maintainer commands, not actions performed by adding the workflow. Tagging starts publication automatically. Hyphenated SemVer tags are prereleases and do not become Latest; plain versions are published as regular releases and marked Latest. This workflow never publishes crates.io packages or creates version commits/tags for you.

## Assets and permissions

Each build packages its actual `rustc -vV` host target into `streamscope-VERSION-TARGET.tar.gz`. Linux x86_64 and native macOS x86_64/Apple Silicon targets are allowed; the hosted macOS runner determines which native architecture is produced. The target in the filename is authoritative. These are native builds, not a promise of every architecture or old runtime compatibility.

Archives contain the binary, standalone usage notes, Apache-2.0 LICENSE, release version/target metadata, and upstream license/notice files with an inventory. Tar/gzip metadata excludes build-user names and local timestamps. Package creation refuses missing upstream notices, unsupported targets, and stale binary versions.

Only the publisher job has `contents: write`. It uses the built-in GITHUB_TOKEN, no personal tokens or account switching. Actions are pinned to reviewed commit SHAs. Pull-request checks receive no release credentials. Publication waits for both matrix builds; their SHA-256 sidecars and archive contents are checked again before a combined SHA256SUMS is produced. macOS binaries are unsigned/unnotarized; Linux binaries depend on compatible glibc. Signing and Windows distribution are deferred.

The workflow uploads assets to a draft first, then publishes it. Existing releases are not overwritten. If upload or publication fails, inspect the draft and logs before retrying; resolve/remove an incomplete draft deliberately. Do not move an existing public tag or replace published assets silently. Prefer a corrected version for a defective public release.

## Local preparation without publication

Python 3.9+ and the pinned Rust toolchain are required. From a built checkout:

```sh
python3 scripts/release.py validate --tag v0.1.0-alpha.1
cargo build --locked --release --workspace
python3 scripts/release.py package --tag v0.1.0-alpha.1 --output dist
```

Use the actual manifest version. Package creation may fetch locked registry metadata/notices and runs the existing synthetic scenario smoke checks against the binary extracted from the archive. It creates a tar.gz and matching .sha256 sidecar; use a fresh output directory for each attempt. Assembly (`verify`) requires two distinct platform candidates; a single local host cannot validate the other host's executable. Local checks do not publish, tag, or contact release APIs.
