# Deleuze

Read [blackbird.md](blackbird.md) for the working doctrine. The symlink points to
our shared workspace instructions; reference copies inside quarantine are historical.

Use `bd` for all issue tracking. Run `bd prime` for workflow context.

Deleuze is a standalone Rust crate with one responsibility: capture every event
that reaches its gate, extremely quickly, with deployment-funded reliability in
nines. Assess throughput and tail latency alongside retention reliability and
cost. Full/failing-gate arrivals remain within scope; do not redefine completeness
as only successfully admitted events. Support Rust integration and polyglot
interfaces. World-state acquisition/evals are consumers of the retained stream.
Ship HTTP and gRPC gates plus interfaces and guidance for custom adapters. The
promise begins when a request reaches a gate. All integrations use one owned
capture path; do not expose internals or alternative mutation routes. Event
content uses arbitrary properties on an extremely small, strict schema. Unknown
or malformed input must still have a persistable representation; large content
may use references in properties. See design/gates-and-events.md.
Persist event contents only. Do not add whole-request archival, duplicate original
body storage, a second capture layer or mandatory request-lifecycle events.
The current phase is
source acquisition and research, before design and implementation. Preserve source
snapshots in ignored `quarantine/` and put tracked acquisition manifests in
`papers/recon/`.
Keep research and independent lane reports in `papers/`, and session records in
`journal/`. Do not adopt a dependency without discussing it with the operator.

## Private-source boundary

Private Forgejo repositories may inspire generic reasoning, but do not record
anything about them here: no names, descriptions, paths, inventories, revision or
file hashes, quotations, source-specific findings or identifying attribution.
This applies to tracked files, ignored files, issue records and Git history.
Keep all such acquisitions and notes outside this checkout. This operator rule
supersedes the shared doctrine's default placement and manifest requirements for
private sources. Public literature/reference provenance remains in this repo.

<!-- BEGIN BEADS INTEGRATION -->
## Issue Tracking with bd (beads)

**IMPORTANT**: This project uses **bd (beads)** for ALL issue tracking. Do NOT use markdown TODOs, task lists, or other tracking methods.

### Why bd?

- Dependency-aware: Track blockers and relationships between issues
- Git-friendly: Dolt-powered version control with native sync
- Agent-optimized: JSON output, ready work detection, discovered-from links
- Prevents duplicate tracking systems and confusion

### Quick Start

**Check for ready work:**

```bash
bd ready --json
```

**Create new issues:**

```bash
bd create "Issue title" --description="Detailed context" -t bug|feature|task -p 0-4 --json
bd create "Issue title" --description="What this issue is about" -p 1 --deps discovered-from:bd-123 --json
```

**Claim and update:**

```bash
bd update <id> --claim --json
bd update bd-42 --priority 1 --json
```

**Complete work:**

```bash
bd close bd-42 --reason "Completed" --json
```

### Issue Types

- `bug` - Something broken
- `feature` - New functionality
- `task` - Work item (tests, docs, refactoring)
- `epic` - Large feature with subtasks
- `chore` - Maintenance (dependencies, tooling)

### Priorities

- `0` - Critical (security, data loss, broken builds)
- `1` - High (major features, important bugs)
- `2` - Medium (default, nice-to-have)
- `3` - Low (polish, optimization)
- `4` - Backlog (future ideas)

### Workflow for AI Agents

1. **Check ready work**: `bd ready` shows unblocked issues
2. **Claim your task atomically**: `bd update <id> --claim`
3. **Work on it**: Implement, test, document
4. **Discover new work?** Create linked issue:
   - `bd create "Found bug" --description="Details about what was found" -p 1 --deps discovered-from:<parent-id>`
5. **Complete**: `bd close <id> --reason "Done"`

### Auto-Sync

bd automatically syncs via Dolt:

- Each write auto-commits to Dolt history
- Use `bd dolt push`/`bd dolt pull` for remote sync
- No manual export/import needed!

### Important Rules

- ✅ Use bd for ALL task tracking
- ✅ Always use `--json` flag for programmatic use
- ✅ Link discovered work with `discovered-from` dependencies
- ✅ Check `bd ready` before asking "what should I work on?"
- ❌ Do NOT create markdown TODO lists
- ❌ Do NOT use external issue trackers
- ❌ Do NOT duplicate tracking systems

For more details, see README.md and docs/QUICKSTART.md.

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds

<!-- END BEADS INTEGRATION -->
