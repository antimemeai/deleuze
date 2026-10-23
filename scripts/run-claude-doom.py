#!/usr/bin/env python3
"""Launch the operator-requested, resumable Claude research assignment.

Requires an already authenticated Claude Code subscription. This does not log in,
change account settings, select a model, or bypass tool permission checks.
"""
import datetime
import json
from pathlib import Path
import subprocess
import uuid

ROOT = Path(__file__).resolve().parents[1]


def main():
    session = str(uuid.uuid4())
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    outdir = Path.home() / ".local/share/deleuze-private-research/claude-runs" / stamp
    outdir.mkdir(parents=True, exist_ok=False)
    prompt = ROOT / "papers/survey/claude-doom/task.md"
    command = [
        "claude", "-p", "--output-format", "stream-json", "--verbose",
        "--session-id", session, "--name", "deleuze-doom-frumentarii",
        "--effort", "medium", "--permission-mode", "acceptEdits",
        "--permission-prompts", "none", "--allowedTools",
        "Bash,Read,Glob,Grep,Write,Edit,WebSearch,WebFetch,Agent,Task",
    ]
    with prompt.open("rb") as source, (outdir / "events.jsonl").open("wb") as output, (outdir / "stderr.log").open("wb") as errors:
        process = subprocess.Popen(
            command, cwd=ROOT, stdin=source, stdout=output, stderr=errors,
            start_new_session=True, close_fds=True,
        )
    metadata = {
        "session_id": session, "pid": process.pid, "prompt": str(prompt),
        "events": str(outdir / "events.jsonl"), "stderr": str(outdir / "stderr.log"),
        "note": "Process launch does not establish authentication or successful work; inspect the event stream.",
    }
    (outdir / "session.json").write_text(json.dumps(metadata, indent=2) + "\n")
    print(json.dumps(metadata, indent=2))


if __name__ == "__main__":
    main()
