# Claude DOOM run status

2026-10-23 UTC. The operator requested a Claude Code DOOM frumentarii run.

A full assignment is prepared in [task.md](task.md). Claude Code 2.1.304 is
installed locally and its auth status reports a cached subscription login, but
an actual noninteractive model request returned:

> Failed to authenticate: OAuth token revoked. Please log in again or contact your administrator.

Session `32b1e9d6-71c0-4772-abf2-31b296af602e` exited before research or any tool
call. This is an attempted launch, not a running Claude research lane. The task,
process metadata and request stream are retained; no credentials are in them.
The operator was asked to run `claude auth login` locally. Reuse the task after
successful login. Initial native background launch also required workspace trust;
noninteractive print mode avoided that UI without changing permission settings.

Neuroses SSH became intermittently unreachable during the fallback check, then
responded. `claude` is not on the remote noninteractive PATH. The standard
`~/.claude/CLAUDE.md`, `~/CLAUDE.md`, commands and skills locations did not contain
the workflow definition in the first check. A bounded search of additional
workflow/CLAUDE filenames is in progress. No DOOM definition has been inferred.

Codex's broad acquisition wave has produced the durable-log, world-state,
fast-capture, Verbs/verifiability, integrity and measurement reports. These are
independent Codex results, not Claude work.

A second actual local readiness request also failed with `OAuth session expired and could not be refreshed`; no research ran. Neuroses executable `/home/patrick/.local/bin/claude` reported version 2.1.306, but its authentication-status command stalled for eleven minutes and was terminated (only that task-owned process). The fallback has not established a usable authenticated session. An independent read-only discovery lane is locating the actual workflow definition.

After restoring login, run `python3 scripts/run-claude-doom.py` from the repository root. Each attempt gets a distinct retained session/log directory. A launched PID is not a successful research request; inspect the stream for authentication errors and actual tool work.
