# Claude DOOM run status

## Current: stopped at operator request (session 757aaa39)

The operator requested termination. Codex sent SIGTERM to the session's process
group and its identified descendants, including children in separate process
groups. A subsequent process-table check found no surviving identified run
processes. No restart was launched.

The operator clarified that DOOM means **20M+ tokens at medium effort**. Codex had
incorrectly launched at maximum effort before locating the actual workflow.
The launcher now specifies medium effort; future execution requires a new
operator instruction. The logs remain available for inspection.

## Launch history

2026-10-23T05:52Z. **Claude Code is now authenticated and running research.**
Session `757aaa39-662c-4caf-86ff-67228a90ae55` was launched by
`scripts/run-claude-doom.py` at 05:51:41Z; its stream is in
`context/claude-doom/20261023T055141Z/`. This session has made tool calls and
is reading the repository context.



## Milestones

- 05:52Z: session started; context reading began.

## Prior attempts (history, kept for the record)

Before this session, Claude Code launches failed to authenticate:
