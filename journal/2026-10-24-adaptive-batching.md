# Adaptive batching research

The operator challenged fixed batch sizing and required literature plus
reference implementations for project details. Claimed `deleuze-iag`. Read the
current collector: occupancy is opportunistic, targets and deliberate wait are
static. Marked the existing defaults provisional and added the operator's
evidence requirement to AGENTS.md.

Research lanes study database/log grouping, feedback control and self-clocking
models. Root acquired the SEDA author PDF and original Sandstorm 3.0 source,
paired §3.4 with `AggThrottle` and its thread-manager use, reread Aether §4.1 and
BookKeeper Journal, inspected Kafka accumulator readiness, and paired RFC 896
with Linux v6.12 Nagle/Minshall source. Originals remain ignored; public hashes
and restoration instructions are tracked. Historical source was read, not built.

The report separates resource bounds, target size, deliberate wait and completion
semantics. No adaptive production policy, dependency or new default is adopted.
Current behavior remains the baseline for comparative measurement.

Independent lane reading found material distinctions: PostgreSQL natural group
commit, input-sensitive RocksDB group limits versus adaptive-yield waiting,
PALF's historical group-factor policy versus today's append-count mode switch,
and Stout interval feedback with an unresolved printed equation/source gap.
Control reading paired Net DIM's profile hill climber with its driver callback,
and distinguished Clipper paper AIMD from later estimator code. A recent
self-clocking preprint's companion prototype produced sparse-arrival
counterexamples despite all14 upstream tests passing; no optimality conclusion
was adopted. Its stored per-load results also conflict with its strongest
universal headline. No AWS/PostgreSQL performance experiment was reproduced.

Reviews corrected SEDA's configurable defaults and intermediate-reset
fallthrough description, Kafka's completion-in-progress trigger, and the
synthesis's distinction between soft grouping targets and pending admission
bounds. Database wording now explicitly distinguishes Deleuze Written and
Synced rather than conflating ordinary sync with excluded power-loss work.
The study records12 citation entries with reading depth, four lane reports and
four restoration manifests. Source gaps are explicit. Follow-up design and
comparative policy evaluation: `deleuze-crp`; acquisition gaps: `deleuze-wv6`.

Closed `deleuze-iag` after research review. Catalog generation and local research
link checks passed; privacy scan and Git whitespace checks passed. Production
code is unchanged, so no crate test rerun was needed. The reference prototype's
14 passing tests and direct counterexamples belong to its lane, not Deleuze's
test results. The installed bd again rejected `bd sync` as an unknown command;
exported `.beads/issues.jsonl` for Git synchronization as in prior sessions.
