#!/usr/bin/env python3
"""Validate versions and package/verify native GitHub release candidates (Python 3.9+)."""
import argparse
import gzip
import hashlib
import io
import json
import re
import shutil
import subprocess
import tarfile
import tempfile
from pathlib import Path, PurePosixPath

from smoke import smoke

ROOT = Path(__file__).resolve().parent.parent
TARGETS = {"x86_64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-apple-darwin"}
MAX_LICENSE_BYTES = 4 * 1024 * 1024
MAX_ARCHIVE_BYTES = 256 * 1024 * 1024


def validate(tag):
    """Require an exact Cargo/SemVer tag match; reject unsafe or ambiguous names."""
    number = r"(?:0|[1-9][0-9]*)"
    if not re.fullmatch(rf"v{number}\.{number}\.{number}(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?", tag):
        raise ValueError("Expected vMAJOR.MINOR.PATCH with an optional prerelease suffix")
    version = tag[1:]
    if "-" in version:
        for part in version.split("-", 1)[1].split("."):
            if part.isdigit() and len(part) > 1 and part.startswith("0"):
                raise ValueError("Numeric prerelease identifiers cannot have leading zeroes")
    workspace = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--offline", "--no-deps", "--format-version", "1",
    ], cwd=ROOT, text=True))["packages"]
    if not workspace or any(package["version"] != version for package in workspace):
        raise ValueError("Tag must exactly match workspace.package.version; update manifests and lockfile before tagging")
    if any(package["license"] != "Apache-2.0" for package in workspace) or not (ROOT / "LICENSE").is_file():
        raise ValueError("Root Apache-2.0 license and metadata are required")
    return version


def digest(path):
    """Hash files in bounded chunks rather than loading release archives into memory."""
    result = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            result.update(chunk)
    return result.hexdigest()


def third_party_files(target):
    """Collect upstream notices without local paths or following links outside a crate."""
    metadata = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--format-version", "1", "--filter-platform", target,
    ], cwd=ROOT, text=True))
    files = {}
    inventory = []
    notice_bytes = 0
    for package in sorted(metadata["packages"], key=lambda p: (p["name"], p["version"])):
        if package["source"] is None:
            continue
        folder = Path(package["manifest_path"]).parent.resolve()
        candidates = []
        for path in folder.iterdir():
            if path.name.lower().startswith(("license", "copying", "notice", "copyright")):
                candidates.extend(path.rglob("*") if path.is_dir() else [path])
        if package.get("license_file"):
            candidates.append(folder / package["license_file"])
        notices = []
        for path in sorted(set(candidates)):
            if not path.is_file():
                continue
            resolved = path.resolve()
            if not resolved.is_relative_to(folder) or path.stat().st_size > MAX_LICENSE_BYTES:
                raise ValueError("Upstream notice is outside its package or exceeds the size limit")
            notice_bytes += path.stat().st_size
            if notice_bytes > MAX_ARCHIVE_BYTES // 2:
                raise ValueError("Combined upstream notices exceed the size limit")
            relative = path.relative_to(folder).as_posix()
            name = f"THIRD_PARTY_LICENSES/{package['name']}-{package['version']}/{relative}"
            files[name] = path.read_bytes()
            notices.append(relative)
        if not notices:
            raise ValueError(f"No distributable license/notice file found for {package['name']}; review before release")
        inventory.append({"name": package["name"], "version": package["version"], "license": package["license"], "notices": notices})
    files["THIRD_PARTY_LICENSES/index.json"] = (json.dumps(inventory, indent=2) + "\n").encode()
    return files


def add_file(archive, name, data, mode=0o644):
    """Write reproducible tar headers with no build-user names or local timestamps."""
    info = tarfile.TarInfo(name)
    info.size = data.stat().st_size if isinstance(data, Path) else len(data)
    info.mode = mode
    info.mtime = 0
    with (data.open("rb") if isinstance(data, Path) else io.BytesIO(data)) as source:
        archive.addfile(info, source)


def package(tag, output):
    """Package the tested native binary and smoke-test the binary extracted from it."""
    version = validate(tag)
    verbose = subprocess.check_output(["rustc", "-vV"], cwd=ROOT, text=True)
    target = next(line.removeprefix("host: ") for line in verbose.splitlines() if line.startswith("host: "))
    if target not in TARGETS:
        raise ValueError("Native release target is not supported")
    binary = ROOT / "target/release/streamscope"
    if binary.stat().st_size > MAX_ARCHIVE_BYTES:
        raise ValueError("Release binary exceeds the size limit")
    observed = subprocess.check_output([str(binary), "--version"], text=True, timeout=10).strip()
    if observed != f"streamscope {version}":
        raise ValueError("Built binary version does not match the tag; rebuild it")
    stem = f"streamscope-{version}-{target}"
    output.mkdir(parents=True, exist_ok=True)
    archive_path = output / f"{stem}.tar.gz"
    if archive_path.exists():
        raise ValueError("Release archive already exists; use a fresh output directory")
    files = third_party_files(target)
    files["LICENSE"] = (ROOT / "LICENSE").read_bytes()
    files["release.json"] = (json.dumps({"version": version, "target": target, "license": "Apache-2.0"}, indent=2) + "\n").encode()
    files["README.md"] = f"""# StreamScope {version}

Native target: `{target}`. Licensed under Apache-2.0; see LICENSE and THIRD_PARTY_LICENSES.

Run `./streamscope --help`. Start `./streamscope demo --scenario gap` and use another terminal:
`./streamscope watch --url http://127.0.0.1:8383/events`.
For CI: `./streamscope check --url http://127.0.0.1:8383/events --max-gap-ms 1500 --require-last-event done --format json`.

Timings are client observations. No automatic reconnect, server-side causal diagnosis, or Windows support.
TLS uses system networking and the binary requires a compatible host runtime. macOS binaries are unsigned/unnotarized.
Verify this archive against the release SHA256SUMS before extracting it. Checksums detect corruption, not publisher identity.
Source and complete documentation: https://github.com/Olmer1st/StreamScope
""".encode()
    # Gzip's filename and tar ownership are explicitly neutral, avoiding personal build metadata.
    with archive_path.open("xb") as raw, gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=0) as compressed:
        with tarfile.open(fileobj=compressed, mode="w") as archive:
            add_file(archive, f"{stem}/streamscope", binary, 0o755)
            for name, data in sorted(files.items()):
                add_file(archive, f"{stem}/{name}", data)
    inspect_archive(archive_path, version, target)
    with tempfile.TemporaryDirectory(prefix="streamscope-release-") as temp:
        unpacked = Path(temp) / "streamscope"
        with tarfile.open(archive_path) as archive:
            with archive.extractfile(f"{stem}/streamscope") as source, unpacked.open("wb") as dest:
                shutil.copyfileobj(source, dest)
        unpacked.chmod(0o755)
        subprocess.run([str(unpacked), "--help"], check=True, stdout=subprocess.DEVNULL, timeout=10)
        smoke(str(unpacked))
    archive_path.with_name(archive_path.name + ".sha256").write_text(f"{digest(archive_path)}  {archive_path.name}\n")
    print(f"Packaged and smoke-tested {archive_path.name}")


def inspect_archive(path, version, target):
    """Reject traversal, duplicate entries, links, excess size, and mismatched metadata."""
    stem = f"streamscope-{version}-{target}"
    if path.stat().st_size > MAX_ARCHIVE_BYTES:
        raise ValueError("Compressed release archive exceeds the size limit")
    names = set()
    total = 0
    with tarfile.open(path) as archive:
        for member in archive:
            parts = PurePosixPath(member.name).parts
            total += member.size
            if (not member.isfile() or not parts or parts[0] != stem or ".." in parts
                    or member.name in names or total > MAX_ARCHIVE_BYTES or len(names) >= 4096 or len(member.name) > 512):
                raise ValueError("Unsafe or oversized release archive")
            names.add(member.name)
        required = {f"{stem}/{name}" for name in ["streamscope", "LICENSE", "README.md", "release.json", "THIRD_PARTY_LICENSES/index.json"]}
        if not required.issubset(names):
            raise ValueError("Release archive is missing required files")
        with archive.extractfile(f"{stem}/streamscope") as binary:
            header = binary.read(32)
        if target.endswith("linux-gnu"):
            matches = header[:6] == b"\x7fELF\x02\x01" and int.from_bytes(header[18:20], "little") == 62
        else:
            cpu = 0x0100000C if target.startswith("aarch64") else 0x01000007
            matches = header[:4] == b"\xcf\xfa\xed\xfe" and int.from_bytes(header[4:8], "little") == cpu
        if not matches:
            raise ValueError("Executable format/architecture does not match its release target")
        member = archive.getmember(f"{stem}/release.json")
        if member.size > 4096:
            raise ValueError("Oversized release metadata")
        with archive.extractfile(member) as source:
            metadata = json.load(source)
        if metadata != {"version": version, "target": target, "license": "Apache-2.0"}:
            raise ValueError("Archive metadata does not match this release")


def verify(tag, artifacts, output):
    """Require one Linux and one macOS candidate, validate checksums, and assemble assets."""
    version = validate(tag)
    archives = sorted(artifacts.rglob("*.tar.gz"))
    if len(archives) != 2:
        raise ValueError("Expected exactly two native release candidates")
    candidates = []
    platforms = set()
    for archive in archives:
        prefix = f"streamscope-{version}-"
        target = archive.name.removeprefix(prefix).removesuffix(".tar.gz")
        if not archive.name.startswith(prefix) or target not in TARGETS:
            raise ValueError("Unexpected candidate version or target")
        platform = "linux" if target.endswith("linux-gnu") else "macos"
        if platform in platforms:
            raise ValueError("Duplicate release platform")
        platforms.add(platform)
        expected = f"{digest(archive)}  {archive.name}\n"
        sidecar = archive.with_name(archive.name + ".sha256")
        if sidecar.stat().st_size > 1024 or sidecar.read_text() != expected:
            raise ValueError("Release checksum mismatch")
        inspect_archive(archive, version, target)
        candidates.append((archive, expected))
    if output.exists() and any(output.iterdir()):
        raise ValueError("Use an empty output directory for release assembly")
    output.mkdir(parents=True, exist_ok=True)
    for archive, _ in candidates:
        shutil.copyfile(archive, output / archive.name)
    (output / "SHA256SUMS").write_text("".join(checksum for _, checksum in candidates))
    print("Verified Linux/macOS candidates and assembled SHA256SUMS")


def main():
    """Run a release preparation command without publishing or reading credentials."""
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    for name in ["validate", "package", "verify"]:
        command = commands.add_parser(name)
        command.add_argument("--tag", required=True)
        if name == "validate":
            command.add_argument("--github-output", type=Path)
        else:
            command.add_argument("--output", type=Path, default=Path("dist"))
        if name == "verify":
            command.add_argument("--artifacts", type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "validate":
            version = validate(args.tag)
            if args.github_output:
                with args.github_output.open("a") as output:
                    output.write(f"prerelease={'true' if '-' in version else 'false'}\n")
            print(f"Validated {args.tag}")
        elif args.command == "package":
            package(args.tag, args.output)
        else:
            verify(args.tag, args.artifacts, args.output)
    except (ValueError, OSError, subprocess.SubprocessError, tarfile.TarError) as error:
        parser.exit(1, f"Release preparation failed: {error}\n")


if __name__ == "__main__":
    main()
