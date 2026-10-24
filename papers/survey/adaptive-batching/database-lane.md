# Database and journal batching: mechanisms that actually adapt

2026-10-23. Public-source reading lane. No controller, dependency, persistence protocol or implementation is selected. [Acquisitions, exact revisions, hashes, licenses and restoration commands](../../recon/adaptive-batching-database.md) distinguish current code from historical paper artifacts.

The strongest references describe three different mechanisms: naturally varying groups when the writer becomes available, a measured-load switch between dispatch policies, and a feedback controller that changes a batching interval. Calling all three “adaptive batching” hides the engineering choice. A fixed resource ceiling can constrain any of them; the ceiling is not the adaptive policy.

| Reference | Observed variable | Action | Classification |
| --- | --- | --- | --- |
| PostgreSQL `XLogFlush` | Completed insertions and shared flushed position when the write lock becomes available | One writer covers additional completed WAL; satisfied waiters skip redundant flushes | Opportunistic, self-clocked grouping; optional configured delay |
| LevelDB / RocksDB write group | Queued compatible writes and leader's byte size | Gather available writes up to an input-sensitive cap | Natural fill plus static cap rule; no measured performance target |
| PALF paper §6 | Group factor used as a proxy for client concurrency | Switch feedback-triggered freeze versus periodic freeze | Load-sensitive policy switch |
| Current OceanBase PALF code | Append count in approximately one-second checking windows | Switch feedback freeze versus nominal 1 ms periodic freeze | Feedback mode selection, fixed threshold and fixed periodic interval |
| Stout §3.2 | Completed-operation latency, bytes, current interval and recent performance | Change interval using performance comparisons and multiplicative updates | Explicit feedback interval controller |
| Aether §4.1 | Example transaction, byte and elapsed-time thresholds | Flush daemon dispatches groups while workers do other work | Scheduling/pipelining mechanism; no adaptive target rule specified there |

## PostgreSQL: the writer supplies a clock

At pinned revision `126976a8ffcfa47f40d3df09243ebf4606001912`, [`src/backend/access/transam/xlog.c`](https://github.com/postgres/postgres/blob/126976a8ffcfa47f40d3df09243ebf4606001912/src/backend/access/transam/xlog.c#L2867), `XLogFlush` first tests whether the caller's position is already flushed. It samples the shared write request, waits for relevant insertions to finish, and uses `LWLockAcquireOrWait(WALWriteLock, LW_EXCLUSIVE)`. A waiter loops back after another writer releases the lock and can discover that its own flush is already satisfied. The holder rechecks progress, then writes/flushed completed insertions through the sampled `insertpos`, extending beyond the original caller's position where possible. Actual group size follows concurrency and service time; there is no batch-size setpoint or latency-feedback gain in this path.

Optional `CommitDelay` sleeps while the write lock is held only when fsync is enabled and enough sibling transactions are active. That sleep gives additional insertions time to join; it is configured rather than learned. [PostgreSQL 18's WAL documentation](https://www.postgresql.org/docs/18/wal-configuration.html) explains that natural group commit still occurs at zero configured delay as commits accumulate during the previous flush. Documentation version 18 and the pinned development source are separately identified, rather than silently equated.

**Interpretation for Deleuze:** dispatching available work when the sole writer finishes can produce dynamic groups without any fixed fill target. It also avoids adding a deliberate wait to an isolated event. This is an empirical baseline for evaluating a controller, not evidence that adaptation is universally unnecessary. PostgreSQL's sync amortization is directly relevant to Deleuze's Synced mode, which uses ordinary `File::sync_all`; those costs do not all transfer to Written mode. Its transaction workload also differs, so batching benefit must be measured at each actual completion boundary. Excluding a power-loss recovery protocol does not exclude ordinary sync.

## RocksDB: an input-sensitive limit is not target adaptation

At revision `ea313605367281013c2c81158069e88411516357`, [`db/write_thread.cc`](https://github.com/facebook/rocksdb/blob/ea313605367281013c2c81158069e88411516357/db/write_thread.cc#L443), `EnterAsBatchGroupLeader` starts with the leader's bytes. For configured maximum `M`, a leader of size at most `M/8` gets a group-growth limit of `leader_bytes + M/8`; otherwise the limit is `M`. The options default `M` to 1 MiB. An individual leader can already exceed the configured group limit: this is a bound on additional grouping, not a general input-size validator.

It samples the newest queued writer and scans that available population. Compatibility checks include sync versus nonsync leadership, WAL mode, `no_slowdown`, integrity-protection level, rate-limiter priority, callback permission and special ingest operations. This revision moves excluded writers into a remainder list and can include later compatible writers. **It does not simply stop at the first incompatible writer.** Copying the grouping mechanics would therefore require a separate ordering argument if arrival order were promised.

[`include/rocksdb/options.h`](https://github.com/facebook/rocksdb/blob/ea313605367281013c2c81158069e88411516357/include/rocksdb/options.h#L1469) calls another feature `enable_write_thread_adaptive_yield`. Its implementation in `AwaitState` adapts spin/yield/block waiting credit from sampled waiting outcomes. It does not update the group byte target. Labeling that option an adaptive batching controller would be incorrect.

The earlier pinned [LevelDB `BuildBatchGroup`](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/db_impl.cc#L1281) similarly uses a 1 MiB maximum and, for a small leader, `leader_bytes + 128 KiB`. Unlike this RocksDB revision, its scan stops at a sync incompatibility or size boundary. Both are valuable queue/grouping references; neither supplies a throughput/latency-feedback target estimator.

## PALF: distinguish the paper from today's implementation

The [2024 PALF paper](https://www.vldb.org/pvldb/vol17/p3745-xu.pdf), §6, proposes selecting between I/O-feedback grouping at low concurrency and periodic grouping at high concurrency. Its detailed threshold paragraph and §8.2 evaluation agree on that direction: the evaluation used group-factor threshold 10 because ten workers handled clients; periodic freeze defaults to 1 ms. One earlier sentence in §6 reverses low and high. That sentence conflicts with the subsequent explanation and evaluation; it is not a second proven policy. The experiments use closed-loop clients, waiting for the previous log to commit before appending the next.

Current source revision `5e8a63d29dae9fde5e962b8fcc7b167d438fc343` implements a related but different switch. [`log_sliding_window.cpp`](https://github.com/oceanbase/oceanbase/blob/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_sliding_window.cpp#L1350) sums and resets 32 append-counter shards. A count at least 140000 selects periodic mode; a smaller count selects feedback mode. [`log_loop_thread.cpp`](https://github.com/oceanbase/oceanbase/blob/5e8a63d29dae9fde5e962b8fcc7b167d438fc343/src/logservice/palf/log_loop_thread.cpp#L104) schedules that check nominally once per second and uses the 1 ms loop interval while any handle is in periodic mode. The feedback path freezes pending work when the flushed position reaches the pending task's beginning. Switching back to feedback explicitly attempts an immediate freeze.

This is actual runtime adaptation, but its changed state is the dispatch mode, not a continuously learned batch size or latency goal. The inspected switch has one count threshold, no separate enter/exit hysteresis, and no measured latency or service-rate estimator. A busy loop's delayed check changes the count window; it is not an exactly normalized rate calculation. Source scheduling and counter operations do not establish a formal one-second response bound. Current code is not claimed to be the artifact evaluated by the 2024 paper.

**Interpretation:** PALF supports investigating a simple workload-sensitive switch before a complicated optimizer. Its fixed threshold and 1 ms period are deployment choices, not justification for copying those numbers. Replication, lock-free sequencing and durability machinery are outside this lane's recommendation.

## Stout: an explicit interval feedback loop

[Stout](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/12/usenix-2010-stout.pdf) (McCullough, Dunagan, Wolman, Snoeren; USENIX ATC 2010), §3.2, updates after both enough completions and a fraction of measured mean latency have elapsed. It compares `bytes/(latency + interval)` against recent performance with slack, then changes the batching interval multiplicatively. Its history differs after acceleration versus backoff to limit overshoot. The paper gives initial/maximum intervals and gain constants. However, its printed `MAX` gain equation conflicts with prose describing an upper bound; that detail remains unresolved without original code. No corresponding public source was identified on the consulted author/publisher pages.

Stout supplies a concrete feedback vocabulary, but targets a shared remote key-value store and includes write collapsing. Collapsing intermediate writes is incompatible with retaining every event. Its millisecond operating range and mean-latency optimization do not establish Deleuze's tail-latency behavior. This is a controller reference, not an implementation recipe.

## Aether: scheduling is a separate source of batching gains

[Aether](https://www.vldb.org/pvldb/vol3/R61.pdf) (Johnson et al.; PVLDB 2010), §4.1, detaches workers from transaction flush waiting and lets a daemon notify completions after I/O. Its examples trigger on transaction count, bytes or elapsed time, whichever comes first. Those examples specify dispatch thresholds, not an algorithm for changing them. Its useful contribution here is separating worker scheduling from group formation: improved throughput need not demonstrate that a batch-size controller improved anything.

## What these references establish for the next experiment

The reference implementations justify three separately measurable candidates: immediate available-work grouping, a measured-load mode switch, and an explicit feedback wait/size target. They do not establish which wins for Deleuze. The adaptive state, observation window, update rule and output must be named before a design can claim target adaptation. Byte ceilings and queue capacity remain resource limits; describing those alone leaves the operator's dynamic-policy request unanswered.

Experiments should change offered load, event sizes and service time while measuring throughput and the entire arrival-to-write latency, including deliberate batching wait. Closed-loop and externally driven arrivals need separate runs: self-throttling clients can conceal queue growth. Queue saturation and failed writes must stay in the arrival population, with counts and outcomes visible; a controller cannot improve apparent latency by dropping difficult events from its denominator. None of the examined grouping mechanisms establishes completeness under overload.

Use the actual normal-I/O boundary and retain every event separately inside the chosen physical write group. Do not import transaction collapsing, extra log layers or a power-failure protocol to reproduce a paper's benchmark. Source inspection here establishes mechanisms and limits; no performance numbers were reproduced and no production dependency was adopted.
