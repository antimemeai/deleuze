# Adaptive batching for an append-only capture gate

2026-10-24 UTC. Operator direction: a set-and-forget batch size is inadequate;
engineering choices need both literature and reference implementations. This is
selected mechanism reading with source inspection, not an exhaustive literature
claim or a replacement controller design. No production code or dependency
changes accompany this study.

The [bibliography](bibliography.tsv) has twelve citation entries, including
documentation, repeat citations from other lanes and unacquired leads. It does
not claim twelve fully read papers. Reading depths and code provenance are
reported separately.
Unacquired full text and unmatched historical artifacts are listed in the
[acquisition gaps](SHOPPING_LIST.md).

The current collector learns neither a target size nor a waiting interval.
`src/batch.rs::collect` drains ready submissions under configured record/byte
grouping targets, allowing a valid oversized frame alone, and preserves the
first submission's collection deadline. When the queue empties it
may wait until that deadline. Current 256 records / 1 MiB / 1 ms are provisional
baseline settings. Varying occupancy is useful but does not justify these knobs
or satisfy the request for adaptation.
Separate pending admission limits, `max_pending_batches` and
`max_pending_bytes`, bound owned encoded work, not caller inputs or exact RSS.

## The prior-art map

| Mechanism | Primary literature and inspected reference | What actually varies | Transfer and limit |
| --- | --- | --- | --- |
| Completion-driven group commit | Aether §4.1; PostgreSQL WAL and BookKeeper Journal | Group membership follows queued work and prior service completion | Strong cheap baseline; not learned size or wait. Aether and BookKeeper are analogous mechanisms, not paper/artifact lineage. |
| Producer-size-aware write groups | RocksDB WriteThread | Group ceiling depends on leader size and compatible waiting writers | Small leaders avoid always inheriting the maximum group; adaptive yielding is a separate mechanism. |
| Load-switched log batching | PALF paper and current OceanBase PALF | Switch between queue-driven and timed batching | Relevant log-specific adaptation; paper and acquired implementation must be compared separately. |
| Storage congestion feedback | Stout, USENIX ATC 2010 | Batch dispatch interval follows rate/delay feedback | Directly relevant objective; corresponding public source was not located, so this is a literature lead with a reference gap. |
| Throughput hill climbing | SEDA §3.4 and original Sandstorm 3.0 `AggThrottle` | Batch target changes using smoothed output rate | Real target adaptation; lacks direct queue-age/sync/tail signals. |
| Latency-aware batch sizing | Clipper §4.3 and pinned Clipper implementation | Paper AIMD; later code estimates size from deadline and observed runtime | Useful latency-budget precedent; current code is not the paper's AIMD and includes non-transferable expiration/exploration behavior. |
| Adaptive interrupt moderation | Linux Net DIM documentation and v6.12 source | Select among moderation profiles from measured byte/packet/interrupt rates | Useful deadband and parking mechanics; no end-to-end latency objective. |
| ACK-driven accumulation | RFC 896 and Linux TCP Nagle/Minshall predicate | Whether partial output waits on previous completion | Completion-sensitive readiness, not a learned batch cap; networking analogy only. |

Detailed reading and acquisition provenance:

- [Queue, SEDA, BookKeeper, Kafka and Nagle lane](queue-lane.md),
  [originals](../../recon/adaptive-batching-queue.md).
- [Database lane](database-lane.md),
  [originals](../../recon/adaptive-batching-database.md).
- [Feedback-control lane](control-lane.md),
  [originals](../../recon/adaptive-batching-control.md).
- [Self-clocking hypothesis and prototype audit](self-clock-lane.md),
  [originals](../../recon/adaptive-batching-self-clock.md).

Kafka's accumulator is a useful counterexample to imprecise terminology: it
varies occupancy and readiness with full/expired/flush/resource-pressure state,
but does not learn its batch-size allocation or linger in that method. Adaptive
partition routing does not establish adaptive batch-size or delay control.

A recent *Group Commit Self-Clocks* preprint has a companion prototype, but
inspection reproduced policy-model counterexamples despite its test suite
passing. Its closed-loop competitive bound is conjectured, and stored results
also do not support its strongest universal headline. The lane report records
the exact examples. Keep the completion-driven hypothesis in the comparison;
do not use this prototype to certify optimality or transfer closed-loop results
to independent gate arrivals. *Low-Overhead Paxos Replication* supplies another
literal adaptive interval rule, averaging the prior interval and reported flush
time, but its reported comparison does not isolate that rule's benefit. The
replication protocol is outside the batching choice under study.

## What should emerge into a Deleuze design

These are research implications, not adopted algorithms. The first comparison
should be a bounded completion-driven baseline: on writer availability, flush
already-ready work without an additional deliberate wait. Arrivals during the
previous write/sync already had time to accumulate. This has an immediate idle
path and does not need an estimated arrival rate. Compare it against the current
collector and against feedback policies before adding control complexity.

Keep four independent decisions visible:

1. **What is already available?** Queue state and oldest age determine whether
   the writer has work ready now. Avoid requiring a minimum batch to make
   progress. Do not reset the oldest age on each new arrival.
2. **How much ready work should one service turn cover?** A record and byte
   target can adapt; record count alone misrepresents mixed payloads. Hard
   resource bounds still constrain any chosen target, including oversized valid
   records handled under the existing content policy.
3. **Should the writer deliberately wait for more?** Treat waiting separately
   from sizing. Quiet load, bursts and sustained backlog differ. Waiting can
   amortize service overhead near the capacity knee, but under backlog it can
   add latency without obtaining useful extra work. A fixed collection timer
   is not an end-to-end deadline.
4. **What constitutes completion?** Written and Synced have different service
   costs. Adaptation cannot change the requested completion milestone, publish
   before it, reorder the append stream, replay inputs, or discard content.

An adaptive policy needs an objective before it needs a formula. For this
project, useful measurements include retained records/s and bytes/s, gate-to-
completion latency, oldest outstanding age, arrival rate, queue bytes, batch
occupancy, write/sync service time and resource cost. Evaluate these together;
do not reward fast rejections or shorter latency obtained by dropping work.
Not all measurements need to become public counters or run on every append.
Measure the observer/controller overhead as part of capture cost.

**A late group is not automatically evidence that the batch is too large.** A
slow sync can inflate queue delay while per-record marginal service cost stays
small. Shrinking the group can then reduce drain capacity further. Conversely,
a large variable-byte group can itself inflate service time. Separate queue
waiting, encoding, write, sync and response costs before selecting a reaction.
This is a hypothesis to test with workloads and service perturbations, not a
claim that one direction of adjustment always wins.

The sources suggest bounded targets, monotonic timing, smoothing or windows,
deadbands and limited exploration as candidates. Their exact constants are not
portable evidence. Deadline checks also cannot guarantee completion when the
underlying service time exceeds the available budget. A missed target must stay
visible; it does not authorize cancellation of owned content.

## Experiments that distinguish the policies

The prior [batched measurements](../../measurements/2026-10-23-batched/README.md)
establish that ready bulk work amortizes sync; they do not identify an online
adaptive policy. Closed-loop synchronous callers cannot offer independent new
work while blocked. In particular the ready 1,024-record bulk result does not
say how quickly a live stream can form such a batch.

Compare the current static-capped collector, zero-intentional-wait ready drain,
and proposed adaptive size/wait policies under the same completion contract,
resource bounds, payloads and offered trace. Include sparse traffic, isolated
bursts, load steps, sustained backlog, mixed sizes and a change in service time.
Use independent offered-load schedules as well as closed-loop callers. Include
the controller's transition and settling period in results; do not select only
steady state. Vary each policy's resource budget transparently rather than
crediting extra memory or concurrency to the controller.

Report every offered and gate-arrived population, retention, full/failing-gate
outcomes, uncertain replies, remaining backlog, recovery to ordinary load and
tail latency. A faster admitted subset is not better completeness. Finite memory
and finite storage throughput still impose a deployment capacity problem;
dynamic batching does not remove it or silently shrink the gate promise.

## Evidence that remains provisional

Historical implementations and models establish mechanisms, not a Deleuze
optimum. Source was read rather than built unless a lane says otherwise.
Neither a controller nor new defaults have been selected. Exact record/byte
targets, collection-wait policy, update cadence, cold-start behavior and
latency/cost objective need a concrete design plus review, grounded in this
reading and comparative measurement. Follow-up work is tracked in beads rather
than as a second task list here.
