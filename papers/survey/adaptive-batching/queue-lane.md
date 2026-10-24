# Queue-driven and throughput-feedback batching

Selected primary reading, 2026-10-24 UTC. These are mechanisms to compare, not
dependencies or a controller implementation plan. Acquisition and restoration:
[root manifest](../../recon/adaptive-batching-queue.md).

## SEDA: adjust the target, not merely occupancy

Welsh, Culler and Brewer, *SEDA: An Architecture for Well-Conditioned, Scalable
Internet Services*, SOSP 2001, [author PDF](https://mdw.la/papers/seda-sosp01.pdf),
§3.4 and Figure 9. Selected section read. The controller searches for a smaller
event batch that sustains throughput: smooth output rate, reduce the batch,
increase it when throughput deteriorates, reset to maximum after a large change.
The paper describes load shedding separately; shedding is not a transferable
capture policy for Deleuze.

Read the actual Sandstorm 3.0 release of 2002-07-12, not just the paper's prose:
`seda/src/seda/sandStorm/internal/AggThrottle.java:54–74,130–244`;
`ThreadPool.java:65–73,261–267`; `TPSThreadManager.java:175–195` under the
extracted tree. `ThreadPool.getAggregationTarget()` calls the controller and the
thread manager uses that target to choose queue dequeue behavior. This closes
the paper-to-code connection.

With defaults, the code starts at maximum 1,000 and permits a minimum 8; both
bounds and the 0.7 smoothing weight are configurable. It samples no more often
than once per 1,000 ms when queried, uses `0.7*previous + 0.3*observed` throughput,
and changes the target every fifth eligible sample. Direction changes use
0.90/0.98 watermarks and 1.2 factors; 0.2/2.0 changes relative to the best
throughput reset the intermediate target to maximum and the direction to
decreasing, then fall through the ordinary adjustment in the same call. The
returned target therefore need not equal the reset maximum. These are historical implementation choices, not
validated settings for this crate. It reads a wall clock, not a monotonic clock.
No direct queue-age, bytes, sync latency or end-to-end percentile signal enters
`getAggTarget`. Therefore a change in demand can resemble a change in capacity.
Its response to a throughput drop is not a general tail-latency controller.

Transfer: explicit target adaptation, smoothing and separated update cadence.
Do not import its constants, thread-per-stage architecture, load shedding, or
throughput-only objective. Paper and source were read; the historical Java
runtime was not built or benchmarked.

## Aether plus BookKeeper: completion-driven grouping

Johnson et al., *Aether: A Scalable Approach to Logging*, PVLDB 2010,
[paper](https://www.vldb.org/pvldb/vol3/R61.pdf), §4.1 reread. A daemon groups
flushes and resumes clients only after the covering I/O, separating producer
execution from log completion. The paper gives count/bytes/time as possible
trigger policies; it does not establish an adaptive optimum for those knobs.
Avoid importing its database lock-release or recovery machinery into an
immutable capture log.

Apache BookKeeper `7b66b5d7631cecc9198c5d555d8b2d28d3b6525d`,
[`Journal.java`](https://github.com/apache/bookkeeper/blob/7b66b5d7631cecc9198c5d555d8b2d28d3b6525d/bookkeeper-server/src/main/java/org/apache/bookkeeper/bookie/Journal.java),
lines 497–554 and 1054–1130, supplies an analogous production implementation,
not Aether's own artifact. The journal drains available entries; waits according
to the oldest pending enqueue age; flushes on configured count/byte/age
conditions or optionally empty queue. Its timeout handling explicitly groups
already-aged work after a previous slow write. Separately the force-write thread
`takeAll`s waiting requests, syncs through the last covered position, then sends
responses for that group. IOException stops that thread and notifies its parent.

This changes realized group size through queue state and prior I/O completion,
without fitting a target from observations. It is strong baseline prior art for
Deleuze's current collector, including draining ready aged work. It is not
evidence that 256 records, 1 MiB or 1 ms are optimal. A separate sync thread can
overlap journal writes with sync; copying this structure would need its own
ownership/error analysis, not merely a change to batch size.

## Kafka: dynamic fill does not imply adaptive target

Apache Kafka `da2224096692906b5e4398b2531fc4e810550c01`,
[`RecordAccumulator.java:714–735`](https://github.com/apache/kafka/blob/da2224096692906b5e4398b2531fc4e810550c01/clients/src/main/java/org/apache/kafka/clients/producer/internals/RecordAccumulator.java#L714),
source read. A batch becomes sendable when full, linger expires, buffers are
exhausted, the accumulator closes, an explicit flush occurs, or transaction
completion is in progress; retry backoff can prevent sending. The count of messages actually
sent varies; this method does not learn the batch allocation size or linger.
`BuiltInPartitioner.java:222–250` explains an additional interaction: switching
partitions mid-batch can fragment batches, so switching may be deferred with a
bounded overshoot. Kafka's adaptive partition routing is not automatic batch
size or deadline tuning.

Transfer: flush reasons and resource pressure matter, and stacked batching
layers interact. Kafka is a transport batching reference here, not evidence for
Deleuze's local sync completion semantics or an excuse to import retries.

## Nagle: let completion govern accumulation

John Nagle, [RFC 896](https://www.rfc-editor.org/rfc/rfc896.html), 1984,
sections “The small-packet problem” and “The solution to the small-packet
problem” read from retained original. The text rejects a universal fixed delay
and defers more small sends while earlier data remains unacknowledged. An idle
connection sends immediately; pending work accumulates while completion is
outstanding. Modern implementation details differ from this early prose; the
[control manifest](../../recon/adaptive-batching-control.md) includes pinned
Linux TCP source. Linux v6.12 `adc218676eef25575469234709c2d87185ca223a`,
`net/ipv4/tcp_output.c:1962–1997,2115–2135` read: `tcp_nagle_check` suppresses
partial segments under CORK, or under ordinary Nagle when packets are outstanding
and the Minshall small-segment condition holds. `tcp_nagle_test` allows push,
urgent and FIN cases. This is an actual completion-sensitive readiness predicate,
not an adaptive timer or a literal copy of every rule in the 1984 text.

Transfer is an analogy: when one sync is in flight, arrivals already accumulate
for the next group without an extra collection timer. Network acknowledgments,
full-segment exceptions and delayed-ACK interactions do not map directly to file
sync. This is not a proposal to change TCP options or to gate storage on a
network response.

## What these sources leave open

Readiness, target, intentional wait, byte/resource bounds, concurrency and
completion milestone are distinct choices. The existing native Written path,
explicit ready bulk writes, and Synced automatic collection have different
costs. None of this reading establishes one adaptive controller for all three.
In particular a collection deadline is not a bound on total latency: queued
service, write, sync and response scheduling still consume time.
