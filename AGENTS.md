# Deleuze

Read [blackbird.md](blackbird.md) for the working doctrine. The symlink points to
our shared workspace instructions; reference copies inside quarantine are historical.

Use `bd` for all issue tracking. Run `bd prime` for workflow context.
Commit coherent changes frequently and push them liberally, as the operator
requested; do not save the whole phase for one final commit.

Deleuze is a standalone Rust crate with one responsibility: capture every event
that reaches its gate, extremely quickly, with deployment-funded reliability in
nines. Assess throughput and tail latency alongside retention reliability and
cost. Full/failing-gate arrivals remain within scope; do not redefine completeness
as only successfully admitted events. Support Rust integration and polyglot
interfaces. World-state acquisition/evals are consumers of the retained stream.
Ship HTTP and gRPC gates plus interfaces and guidance for custom adapters. The
promise begins when a request reaches a gate. All integrations use one owned
capture path; do not expose internals or alternative mutation routes. An event
has an extremely small, strict record schema with associated arbitrary property
key/value pairs outside that schema. Producers normally send existing output;
Deleuze packs it at the gate. Core fields are write-assigned seq, property_hash
and created_at for log write; received_at is a candidate gate timestamp. Only
Deleuze sets record timestamps; producer times remain properties. seq describes
log-write order and emphatically does not order actions in time or imply causality.
Event types, producer IDs and cooperation with producer-masked data are
configurable features with defaults; see design/record-profile.md. Producers mask
data; Deleuze does not redact on ingest. It records the supplied representation
and can point readers to underlying data only when their ABAC is sufficient.
Reference assistance defaults to off until configured. property_hash covers the
properties actually retained. Unknown input still needs a persistable representation.
Large content may use references in properties. The Event { properties } proposal
remains withdrawn. See design/gates-and-events.md.
Persist event contents only. Do not add whole-request archival, duplicate original
body storage, a second capture layer or mandatory request-lifecycle events.
Stored records and associated properties are immutable. Later contradictory
assertions and corrections append new records. Expose append/read/query only:
no update/delete/overwrite/truncate/clear/prune or history-rewriting compaction
methods and no raw storage handles providing a bypass. Current-state views are
derived and never rewrite retained records.
Use maintained, reviewed cryptographic implementations. The official BLAKE3 crate
was discussed and accepted for ordinary property integrity hashing. Good primitives
do not establish protocol security: do not invent signing, authentication, chaining
or anchoring protocols. See design/cryptographic-boundary.md. Any new security
protocol needs a separate established-design/implementation evaluation.
The operator approved proceeding from the record profile. Complete concrete design
and implementation-plan reviews before production code. Power-loss resilience is
outside the current build; do not let it drive storage architecture or tests.
Use ordinary I/O error handling and clean reopen. Preserve public source
snapshots in ignored `quarantine/` and put tracked acquisition manifests in
`papers/recon/`.
The optional `gates` transport stack (Tokio, Axum, http-body and tower-service)
was discussed and accepted; see design/gate-implementation.md. Keep research and
independent lane reports in `papers/`, and session records in `journal/`.
Do not adopt a new dependency without discussing it with the operator.

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
