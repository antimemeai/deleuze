#!/usr/bin/env python3
"""Run one bounded, read-only command on neuroses and log it.

Usage: claude-doom-ssh.py [--timeout SECONDS] [--tag NAME] -- 'remote command'

The remote command is wrapped in the server's coreutils `timeout`, and the local
subprocess has its own bound, so a stalled link cannot hang the caller. Each call
appends the command, exit status, elapsed time and byte counts to
context/claude-doom/ssh-log.tsv and keeps stdout/stderr under
context/claude-doom/ssh/. A timeout is reported as an incomplete search.
"""
import argparse
import datetime
from pathlib import Path
import shlex
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[1]
LOGDIR = ROOT / "context/claude-doom/ssh"


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--timeout", type=int, default=45)
    parser.add_argument("--tag", default="cmd")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = " ".join(args.command).removeprefix("-- ").strip()
    if command.startswith("--"):
        command = command[2:].strip()
    remote_bound = max(args.timeout - 8, 5)
    remote = f"timeout {remote_bound} bash -c {shlex.quote(command)}"
    argv = ["ssh", "-n", "-o", "BatchMode=yes", "-o", "ConnectTimeout=8", "neuroses", remote]
    LOGDIR.mkdir(parents=True, exist_ok=True)
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    start = time.monotonic()
    try:
        result = subprocess.run(argv, capture_output=True, timeout=args.timeout)
        status, out, err = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as expired:
        status, out, err = "local-timeout", expired.stdout or b"", expired.stderr or b""
    elapsed = time.monotonic() - start
    (LOGDIR / f"{stamp}-{args.tag}.out").write_bytes(out)
    (LOGDIR / f"{stamp}-{args.tag}.err").write_bytes(err)
    with (ROOT / "context/claude-doom/ssh-log.tsv").open("a") as log:
        log.write("\t".join([stamp, args.tag, str(status), f"{elapsed:.1f}",
                             str(len(out)), str(len(err)), command.replace("\t", " ").replace("\n", " ")]) + "\n")
    sys.stdout.write(out.decode("utf-8", "replace"))
    if err:
        sys.stdout.write("\n[stderr]\n" + err.decode("utf-8", "replace")[-4000:])
    sys.stdout.write(f"\n[status={status} elapsed={elapsed:.1f}s bytes={len(out)}]\n")


if __name__ == "__main__":
    main()
