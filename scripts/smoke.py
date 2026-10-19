#!/usr/bin/env python3
"""Run real CLI scenarios with bounded readiness waits and guaranteed cleanup."""
import argparse
import json
import select
import signal
import subprocess
import time
from pathlib import Path


def start_demo(binary, scenario):
    """Start the loopback demo and wait for its actual readiness line."""
    server = subprocess.Popen([binary, "demo", "--scenario", scenario, "--port", "0"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if not select.select([server.stdout], [], [], 10)[0]:
        server.kill()
        server.wait()
        raise RuntimeError("demo readiness deadline exceeded")
    banner = server.stdout.readline()
    if "http://" not in banner:
        server.kill()
        server.wait()
        raise RuntimeError("demo did not print its loopback URL")
    return server, "http://" + banner.split("http://", 1)[1].split()[0], banner


def stop_demo(server):
    """Stop and reap a child without leaking processes on failures."""
    if server.poll() is None:
        server.send_signal(signal.SIGINT)
    try:
        server.wait(timeout=3)
    except subprocess.TimeoutExpired:
        server.kill()
        server.wait()


def smoke(binary):
    """Verify real binaries, safe report structure, and expected negative verdicts."""
    results = []
    for scenario, expected, finding in [("normal", 0, None), ("gap", 2, "GAP_EXCEEDED"), ("burst", 0, None), ("truncated", 2, "MISSING_LAST_EVENT"), ("bad-utf8", 4, "INVALID_UTF8")]:
        server, url, _ = start_demo(binary, scenario)
        try:
            command = [binary, "check", "--url", url + "?api_key=test-secret-456", "--header", "Authorization: Bearer test-secret-123", "--max-gap-ms", "1500", "--first-event-ms", "5000", "--require-last-event", "done", "--format", "json"]
            out = subprocess.run(command, capture_output=True, text=True, timeout=15)
            assert out.returncode == expected, (scenario, out.returncode, out.stderr)
            assert "test-secret" not in out.stdout + out.stderr
            report = json.loads(out.stdout)
            assert report["schemaVersion"] == "streamscope-report/v1"
            if finding:
                assert finding in [f["code"] for f in report["findings"]]
            results.append({"scenario": scenario, "exitCode": out.returncode, "report": report})
        finally:
            stop_demo(server)
    return results


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary")
    parser.add_argument("--save", type=Path)
    args = parser.parse_args()
    results = smoke(str(Path(args.binary).resolve()))
    text = json.dumps(results, indent=2) + "\n"
    if args.save:
        args.save.write_text(text)
    print(text)
