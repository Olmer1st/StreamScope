#!/usr/bin/env python3
"""Record actual normal/gap terminal sessions as asciicast v2; verify restoration."""
import argparse
import fcntl
import json
import os
import pty
import select
import signal
import struct
import subprocess
import termios
import time
from pathlib import Path
from smoke import start_demo, stop_demo


def record(binary, output):
    """Write real PTY output and input; require normal terminal attributes on exit."""
    started = time.monotonic()
    with output.open("w") as cast:
        cast.write(json.dumps({"version": 2, "width": 100, "height": 28, "timestamp": int(time.time()), "title": "StreamScope: real normal and four-second gap sessions", "env": {"TERM": "xterm-256color"}}) + "\n")
        for scenario in ["normal", "gap"]:
            server, url, banner = start_demo(binary, scenario)
            master, slave = pty.openpty()
            fcntl.ioctl(slave, termios.TIOCSWINSZ, struct.pack("HHHH", 28, 100, 0, 0))
            before = termios.tcgetattr(slave)
            process = None
            captured = bytearray()
            try:
                env = os.environ.copy()
                env["TERM"] = "xterm-256color"
                cast.write(json.dumps([time.monotonic() - started, "o", banner]) + "\n")
                process = subprocess.Popen([binary, "watch", "--url", url, "--max-gap-ms", "1500"], stdin=slave, stdout=slave, stderr=slave, env=env)
                session_start = time.monotonic()
                inputs = [(0.8, b"j"), (1.0, b" "), (1.4, b" "), (5.3 if scenario == "gap" else 2.0, b"q")]
                while process.poll() is None:
                    elapsed = time.monotonic() - session_start
                    if inputs and elapsed >= inputs[0][0]:
                        _, data = inputs.pop(0)
                        os.write(master, data)
                        cast.write(json.dumps([time.monotonic() - started, "i", data.decode()]) + "\n")
                    if select.select([master], [], [], 0.05)[0]:
                        data = os.read(master, 65536)
                        captured.extend(data)
                        cast.write(json.dumps([time.monotonic() - started, "o", data.decode("utf-8", errors="replace")]) + "\n")
                    if elapsed > 10:
                        raise RuntimeError("TUI failed to quit")
                while select.select([master], [], [], 0)[0]:
                    data = os.read(master, 65536)
                    captured.extend(data)
                    cast.write(json.dumps([time.monotonic() - started, "o", data.decode("utf-8", errors="replace")]) + "\n")
                assert process.returncode == 0
                assert termios.tcgetattr(slave) == before, "TUI failed to restore terminal attributes"
                assert b"\x1b[?1049l" in captured, "TUI did not leave alternate screen"
                assert b"FROZEN" in captured, "freeze control was not rendered"
                if scenario == "gap":
                    assert b"GAP_EXCEEDED" in captured, "actual TUI did not show the gap finding"
                print(f"{scenario}: real TUI, navigation/freeze/quit and terminal restoration PASS")
            finally:
                if process and process.poll() is None:
                    process.kill()
                    process.wait()
                os.close(master)
                os.close(slave)
                stop_demo(server)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("binary")
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    record(str(Path(args.binary).resolve()), args.output)
