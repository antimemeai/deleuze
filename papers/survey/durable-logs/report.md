# Durable append logs and retained distributed event streams

2026-10-23. Expanded frumentarii lane for Deleuze. The operator's target is to capture **all events reaching the gate**, fund the required reliability, and make recording extremely fast. This report gathers mechanisms, assumptions, source disagreements, and unfinished research. It selects no dependency, crate architecture, storage layout, acknowledgment policy, or deployment topology.

## What was acquired and actually read

The [bibliography](bibliography.tsv) contains 48 primary-source entries: 47 acquired originals, including four linked first-pass items, and one inaccessible report. Nine additions cover explicit 2024–2026 and hardware-failure fronts; see [forward-frontiers.md](forward-frontiers.md). Eight corresponding implementation repositories are pinned in the [source manifest](references-manifest.md): Kafka, BookKeeper, Pravega, Redpanda, CorfuDB, LogDevice, PMDK, and etcd Raft. Their original archives and stripped reading trees are ignored material under `quarantine/{archives,sources}/survey-durable/`.

The initial fifteen papers received focused section reading: ARIES, LFS, Aether, SiloR, CORFU, Tango, FuzzyLog, Scalog, Delos, Raft, Zab, CrashMonkey, RIFL, Persistent Memory I/O Primitives, and Arcadia. Kafka 2011 also received targeted historical/durability/benchmark reading. Seven implementation/API documents received relevant-section reading. The later expansion added section reading of LazyLog, LogDrive, and the SSD power-fault experiment, plus introduction-only SpecLog reading; XLL and three newer device-monitoring/field papers remain abstract-only. Two older empirical storage papers also received abstract-only reading. Four previously acquired first-pass sources are linked with their existing reading scope. Other acquired works remain unread beyond bibliographic identification. These distinctions are recorded per row; source-file inspection is bounded in [source-reading.tsv](source-reading.tsv).

This is a substantial first survey, not a claim that the relevant literature is exhausted. A retained tree is not a reviewed implementation or a reproducible build closure. No benchmark, reference test, acquired program, or service was executed.

## The lineages and what each one promises

| Lineage | Sources in this corpus | Concrete thing retained or recovered | Question for Deleuze |
|---|---|---|---|
| Database recovery | [ARIES](https://people.eecs.berkeley.edu/~brewer/cs262/Aries.pdf), [Aether](https://www.vldb.org/pvldb/vol3/R61.pdf), [SiloR](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-zheng_wenting.pdf); Taurus and Poplar acquired but unread | Enough logged information to recover permitted database states and committed transactions | Which instrumented observations must survive even when the observed operation aborts or a projection no longer needs them? |
| Log-structured storage | [LFS preprint](https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf), first-pass SQLite and LevelDB; NOVA acquired but unread | Current file/database contents, plus recovery metadata; space reclaimed by explicit rules | Which bytes remain part of the acquisition record after checkpoints, compaction, and indexing? |
| Shared logs and derived state | [CORFU](https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final30.pdf), [Tango](https://www.microsoft.com/en-us/research/wp-content/uploads/2013/11/Tango.pdf), [Scalog](https://www.usenix.org/system/files/nsdi20-paper-ding.pdf), [Delos](https://www.usenix.org/system/files/osdi20-balakrishnan.pdf); vCorfu acquired but unread | Replicated ordered records, with views or objects derived by replay | Which operations need one order, and what establishes the durable replay boundary? |
| Partially ordered logs | [FuzzyLog](https://www.usenix.org/system/files/osdi18-lockerman.pdf) | A durable DAG with application-visible ordering constraints | Can independent producers remain independent while preserving the relationships needed by readers and evaluations? |
| Retained stream services | [BookKeeper protocol](https://bookkeeper.apache.org/docs/development/protocol/), [Pravega Segment Store](https://raw.githubusercontent.com/pravega/pravega/3142a01075bfae78d66588e46ff87e6e10225744/documentation/src/docs/segment-store-service.md), [Kafka 4.1](https://kafka.apache.org/41/operations/hardware-and-os/), [Redpanda](https://docs.redpanda.com/streaming/current/develop/manage-topics/config-topics/), [LogDevice](https://logdevice.io/docs/Concepts.html) | Append streams with replication, configuration, tiering, and retention behavior | What does acceptance mean through failover, storage migration, and loss of a failure domain? |
| Consensus and failover | [Raft](https://raft.github.io/raft.pdf), [Zab](https://www.cs.cornell.edu/courses/cs6452/2012sp/papers/zab-ieee.pdf); Paxos, chain replication, CRAQ acquired but unread; VR unavailable | Chosen commands or ordered state changes under stated failures | What storage actions must precede a vote, response, or acknowledgment? |
| Persistence and crash behavior | [CrashMonkey](https://www.usenix.org/system/files/osdi18-mohan.pdf), [PMem I/O](https://arxiv.org/pdf/1904.01614), [Arcadia](https://arxiv.org/pdf/2206.12495), first-pass [Pillai et al.](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-pillai.pdf) | Recoverable bytes under a modeled device/filesystem/persistence domain | Which fault classes does the recorder actually cover, and what directly checks that claim? |

The table describes the sources' problems, not an inheritance claim between every listed implementation. A recovery log and an observation archive can share machinery while keeping different histories.

## Central reading notes

### ARIES: stable storage and recovery are protocol obligations

Read §1.1 and §3 of the [1992 TODS original](https://people.eecs.berkeley.edu/~brewer/cs262/Aries.pdf), DOI 10.1145/128765.128770. The initially acquired image-only scan remains intact; the searchable original used here is separately hashed.

**Source:** WAL requires relevant recovery records on stable storage before an updated page replaces its prior durable contents. Commit completion requires forcing the transaction's log through its commit record. Restart repeats logged history, then undoes losing transactions; compensation records describe rollback progress so repeated failures need not undo the same work indefinitely.

**Interpretation:** examine both forward recording and interrupted recovery. A recorder's restart path can itself fail. Database rollback semantics do not decide whether an observation of an unsuccessful action should remain in Deleuze.

### LFS: sequential writes do not imply the target failure guarantee

Read buffering assumptions, segment cleaning, and checkpoint/recovery passages in the [July 24, 1991 author preprint](https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf). Its first page announces later SOSP/TOCS publication; the journal DOI in the catalogue identifies that related publication, not identical bytes.

**Source:** segment cleaning copies live data and reclaims space; checkpoints bound recovery work. This preprint explicitly accepts losing seconds or minutes of buffered work on a crash. It also says roll-forward is described but not yet implemented in the reported prototype.

**Interpretation:** its locality and cleaning ideas deserve study without borrowing its loss allowance or representing a described recovery path as a tested one. Historical sequential-write throughput is not automatically a comparison for durable gate capture.

### Aether: remove serialization without returning success early

Read §§1–5 of [Aether](https://www.vldb.org/pvldb/vol3/R61.pdf), Johnson et al., PVLDB 2010.

**Source:** the paper separates device I/O volume, lock retention during flushing, OS scheduling, and log-buffer contention. Early lock release retains dependency constraints and postpones client results until durability. Flush pipelining detaches worker execution from the flush wait, then finishes client completion after the daemon reports hardened transactions. Consolidation addresses contended buffer reservations.

**Interpretation:** “fast” has several independent causes. Concurrency and overlapping useful work can improve throughput while preserving the acknowledgment condition. Its database lock rules are source-specific; they are not a drop-in recorder design.

### SiloR: parallel logs need a published durable frontier

Read §§2–4 of [SiloR](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-zheng_wenting.pdf), Zheng et al., OSDI 2014.

**Source:** workers construct value-log records in per-worker buffers; loggers persist batches. A persistent epoch is computed across loggers, stored and synchronized, then published before the corresponding client results are released. Bounded buffer pools provide backpressure. Its recovery exploits database transaction IDs and value semantics; the largest version per value can win during replay.

**Interpretation:** multiple physical streams need an explicit account of which records are durably included together. Value-reconstruction shortcuts that intentionally discard older states do not preserve every observation, even when they correctly recover the database.

### CORFU: ordering before placement creates holes to resolve

Read §§3.2–3.5 of [CORFU](https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final30.pdf), Balakrishnan et al., NSDI 2012.

**Source:** clients map log positions to storage using epoch-tagged projections. A sequencer reserves positions; clients then write them through replica chains. A reserved position can remain unwritten after delay or failure. Hole filling must finish an existing partial value or install junk without creating conflicting successful values. Reconfiguration seals the old projection before replacing it.

**Interpretation:** reservation, completed append, and readable contiguous history are distinct states. Gaps created by coordination must not be mistaken for missing observations whose contents were once accepted. Trimming is an explicit operation with consequences for later history.

### Tango: a history can drive views, while forgetting remains explicit

Read §3.1 and selected transaction/history passages of [Tango](https://www.microsoft.com/en-us/research/wp-content/uploads/2013/11/Tango.pdf), SOSP 2013, DOI 10.1145/2517349.2522732.

**Source:** update helpers append opaque updates; query helpers play the log into local views. Reading through a chosen boundary supplies consistent state and historical reconstruction. Checkpoints can shorten replay; `forget` permits prior history to be trimmed.

**Interpretation:** an index or current view need not own the original observation record. Reconstruction claims must state which bytes, operation semantics, checkpoints, and retained prefix they depend on. Keeping current state and keeping its entire observation history are different policies.

### FuzzyLog: durable observation does not force one global total order

Read §§2–3 of [FuzzyLog](https://www.usenix.org/system/files/osdi18-lockerman.pdf), Lockerman et al., OSDI 2018.

**Source:** the abstraction stores a DAG partitioned into colors and regional chains. Cross-links encode dependencies; append can involve several colors. Sync supplies a snapshot and a traversal respecting the relevant DAG edges. Unordered nodes may appear in different orders; the paper carefully scopes real-time ordering guarantees.

**Interpretation:** complete retained records, causal relationships, and a single presentation sequence are separate properties to discuss. Removing unnecessary global coordination is a concrete research direction; weakening a relationship that an evaluation requires would change its semantics.

### Scalog: place and replicate first, then order durable prefixes

Read §§3–4 and §6.3 of [Scalog](https://www.usenix.org/system/files/nsdi20-paper-ding.pdf), Ding et al., NSDI 2020.

**Source:** storage servers first replicate local FIFO segments. A Paxos ordering layer aggregates reports of durable segment prefixes into global cuts; clients receive positions after records are durably replicated and ordered. Unlike ordering-before-placement designs, its sequencing acts on records that already exist. Its paper distinguishes physical runs from storage-server emulation and larger ordering estimates.

**Interpretation:** record placement and ordering can be separated in more than one direction. Compare both availability during recovery and work on the data path. A large order-generation result is not a measurement of an equally large durable storage deployment.

### Delos: failover includes metadata and an interrupted transition

Read §§3.1–3.2 of [Virtual Consensus in Delos](https://www.usenix.org/system/files/osdi20-balakrishnan.pdf), OSDI 2020.

**Source:** a VirtualLog maps ranges to Loglets. Moving the appendable range requires sealing the current Loglet, finding its final tail, conditionally installing a new mapping in the MetaStore, and retrying through the new mapping. Another client can finish a reconfiguration interrupted after sealing.

**Interpretation:** recorder lifecycle transitions can carry correctness work just as ordinary writes do. Replacing an underlying stream requires a preserved boundary; configuration metadata participates in that claim. The paper does not imply that every service needs this abstraction.

### Raft and Zab: ordering relies on concrete persistence and synchronization

Read Raft Figure 2, §5.3, and the current-term commitment restriction in §5.4.2 of the [extended author version](https://raft.github.io/raft.pdf). Read Zab's primary-order problem and discovery/synchronization/broadcast in the [DSN 2011 original](https://www.cs.cornell.edu/courses/cs6452/2012sp/papers/zab-ieee.pdf), DOI 10.1109/DSN.2011.5958223.

**Sources:** Raft identifies state that must reach stable storage before RPC responses and restricts directly counting replicas to commit entries from the leader's current term. Zab requires primary-order and a synchronization phase before a newly selected primary begins broadcasting state changes.

**Interpretation:** a consensus name is not a storage implementation. The actual persistence-to-response ordering and takeover prefix need inspection in the integration. A majority of volatile copies does not establish the stable-storage assumption merely by being called replication.

### CrashMonkey: direct persisted-data oracles, with honest bounds

Read the workload bounds and §§4.3–4.4 of [CrashMonkey/B3](https://www.usenix.org/system/files/osdi18-mohan.pdf), Mohan et al., OSDI 2018.

**Source:** bounded workloads and crashes after persistence points make a tractable search. The oracle checks persisted file contents and metadata after recovery; filesystem structural repair alone is insufficient to reveal data loss. The paper explicitly excludes some mid-operation and large/resource-exhausting cases from its bounded coverage.

**Interpretation:** test the actual claim about accepted bytes and recovered events, including names and metadata needed to find them. A passing crash campaign should say what crashes and workloads it covered. First-pass [Pillai et al.](../../recon/literature.md) supplies the complementary application/filesystem-contract lineage.

### PMem I/O and Arcadia: persistence barriers, integrity, and ordered completion

Read §3.1/§3.3 of [Persistent Memory I/O Primitives](https://arxiv.org/pdf/1904.01614), acquired v3 dated 2019-06-06. Read §3 and §§4.3–4.4 of [Arcadia](https://arxiv.org/pdf/2206.12495), acquired v1 dated 2022-06-24.

**Sources:** PMem I/O examines failure-atomic append protocols and CPU-cache writeback/fence ordering; its results are hardware-specific. Arcadia separates reservation, data completion, and force; synchronous force supplies durable, gap-free completion. Its optional frequency-based force policy explicitly permits bounded loss of completed-but-unforced records. Remote RDMA completion alone is not its persistence acknowledgment.

**Interpretation:** reduced persistence work can come from a better protocol or from a weaker completion condition; compare them separately. Physical memory persistence, record atomicity, corruption detection, and failover each require their own mechanism. Neither source justifies silently importing a loss window into Deleuze's gate contract.

### RIFL: retry evidence belongs with the completed operation

Read assumptions and architecture in §§2–3 of [RIFL](https://web.stanford.edu/~ouster/cgi-bin/papers/rifl.pdf), SOSP 2015, DOI 10.1145/2815400.2815416.

**Source:** exactly-once RPC behavior requires unique request identity, a durable completion result created atomically with the operation's mutations, retry rendezvous after migration, and safe garbage collection. Its stated assumptions include reliable metadata storage and a bound on the client-crash claim.

**Interpretation:** a timed-out append might already exist. An interface needs enough retained identity/result information to explain retry outcomes. This does not by itself provide atomicity with arbitrary external side effects or acquire uninstrumented events.

## Implementation paths and source disagreements

These observations concern the pinned files in [source-reading.tsv](source-reading.tsv). They do not establish an entire running service's behavior.

- **BookKeeper:** the protocol specifies disk-persisted bookie acknowledgments, ordered client completion, and fencing before takeover. The separately versioned [4.11.1 Ledger API](https://bookkeeper.apache.org/docs/4.11.1/api/ledger-api/) documents `DEFERRED_SYNC`, `force`, and the requirement to force before close when using deferred synchronization. `Journal.java` contains explicit callback-before-sync paths when configured or requested. There is no contradiction once write flags and configuration are kept attached to the claim. A fencing error can leave an ambiguous append outcome; recovery determines it.
- **Pravega:** the pinned architecture describes durable Tier 1 acknowledgment followed by background aggregation into Tier 2. `OperationProcessor.java` packs operations into frames and completes through frame-commit processing. Tier 1 truncation and stream-segment truncation serve different purposes. Source documentation's broad small-append performance language is not an independent measured guarantee.
- **Kafka:** the [2011 paper](https://www.microsoft.com/en-us/research/wp-content/uploads/2017/09/Kafka.pdf) permits some log-event loss, benchmarks asynchronous flushing, and places built-in replication in future work. It is unsuitable as a current Kafka durability specification. The acquired [4.1 operations page](https://kafka.apache.org/41/operations/hardware-and-os/) recommends replication-oriented defaults with application fsync disabled and separately discusses multiple-failure hazards. Current pinned `LocalLog.flush` flushes selected segments and, when needed, the parent directory; tracing every acknowledgment/configuration caller remains unfinished.
- **Redpanda:** the acquired write-caching documentation distinguishes replicated receipt from disk synchronization. The inspected `replicate_batcher.cc` requests flushing for explicit forced requests or quorum acknowledgments with write caching disabled. Transactions and other paths have additional semantics; this is a selected branch, not full call-graph verification.
- **CorfuDB:** selected `StreamLogFiles` append and `sync(force)` paths expose a distinct flush operation. The 2012 CORFU paper is not a contract for every current implementation setting. Mapping a token to a position is not enough to conclude that its record has completed replication and synchronization.
- **LogDevice:** pinned `docs/LogsDB.md` describes normal appends without WAL. Pinned `Settings.cpp` defaults `append-store-durability` to `async_write`, documenting memory, kernel/WAL, and synchronized-WAL modes. Preserve that prose/default mismatch and investigate deployment overrides before claiming one universal behavior. The captured archive is a historical reference, not a maintenance recommendation.
- **PMDK:** the inspected current `pmem_persist` calls flush then drain. The acquired versioned manual explicitly says persistence is neither atomic nor transactional and stores may persist before that call. Current PMDK source does not contain the historical `libpmemlog` implementation referenced by older papers; that historical version remains an acquisition gap.
- **etcd Raft:** the pinned README delegates networking/storage to the application and spells out Ready-batch persistence/message ordering. `node.go` exposes `MustSync` and asynchronous-storage response dependencies. `MemoryStorage` in an example is an integration aid, not durable storage. All external I/O and its errors remain integration responsibilities.

## Where speed claims change meaning

The papers offer serious ways to increase throughput while preserving their stated contract: batching synchronized writes, overlapping useful work, reducing shared-buffer contention, separating ordering from placement, exploiting locality, and reducing unnecessary persistence barriers. The following numbers must retain their setup and acknowledgment conditions.

| Source/result | Conditions and limit on the comparison |
|---|---|
| Aether reports up to roughly 35× benefit for one early-lock-release experiment | A deliberately varied latency/skew experiment on Shore-MT/TPC-B; the 10 ms case includes imposed latency. It is not a prediction for modern recorder append latency. |
| SiloR recovers a 43.2 GB key/value database in 106 s | Its reported 32-core system, three SSDs and RAID, parallel logging/checkpoint/recovery design. This is recovered database size and recovery duration, not per-event durable latency. |
| Scalog's real 17-shard example reaches 255K 4 KiB records/s; 52M records/s is a larger ordering estimate | The paper uses 40 physical machines and emulates storage/ordering workload at larger scales. Its approximately 1.6 ms value at 52M is an estimate, not a measured 3,500-shard storage latency distribution. |
| RIFL adds 530 ns to a 13.5 µs durable-write baseline | Incremental RPC/idempotence overhead in the paper's RAMCloud setup, not total local crate capture cost or a claim about fsync on arbitrary disks. |
| Arcadia's frequency policy bounds unforced completed records by `F × T` in its stated model | This quantifies an optional loss allowance. Synchronous force has a different completion claim; do not compare throughput as if both provided the same boundary. |
| Kafka 2011 producer comparison uses asynchronous flushing | Historical loss tolerance and planned replication matter as much as the throughput bars. Current Kafka configuration must be examined independently. |

These are source-reported experiments, not reproduced measurements. Payload size/distribution, offered load, producer concurrency, batch size/delay, CPU/NUMA placement, replication/failure domains, actual persistence calls, compression, retention/cleaning work, recovery load, and tail-latency reporting all change comparisons. No source here establishes a Deleuze performance number.

## Reliability observations for cost-funded nines

The abstract of [Bairavasundaram et al. 2008](https://www.usenix.org/legacy/event/fast08/tech/full_papers/bairavasundaram/bairavasundaram.pdf) reports spatial/temporal locality and non-independence of corruption within and across disks. The abstract of [Schroeder/Gibson 2007](https://static.usenix.org/events/fast07/tech/schroeder/schroeder.pdf) distinguishes field replacement behavior from datasheet MTTF. These are historical datasets; only abstracts were read. They are reasons to investigate correlated failures and repair behavior, not numerical estimates for modern hardware or Deleuze.

Interpretation: a reliability budget needs an event boundary, failure domains, time horizon, retention horizon, recovery/repair behavior, and observable outcomes. More replicas can address particular failures; their count alone does not supply a probability model. Storage errors, corruption, resource exhaustion, operator deletion, and interrupted repair are different failure classes. Funding the target makes it useful to study their concrete mechanisms rather than weaken the capture objective implicitly.

## Unfinished citation fronts and acquisition gaps

The detailed [shopping list](SHOPPING_LIST.md) distinguishes inaccessible originals, missing implementation versions, and additional citation fronts. Acquired-but-unread originals include Taurus, Poplar, vCorfu, NOVA, Paxos Made Simple, chain replication, CRAQ, End-to-End Arguments, and the 2026 BtrLog preprint. Their titles are catalogue entries, not substitutes for reading.

Useful next citation paths already visible in the read papers include Aether's early-lock-release/group-commit predecessors; CORFU's Camelot and chain-replication references; FuzzyLog's network-ordering and geo-replication references; Scalog's shared-log implementations including Zlog; Arcadia's NVRAM-aware logging, FLEX, Mojim and Query Fresh comparisons; and the crash-consistency literature following Pillai/B3. Each needs original-source acquisition and reading before its claims enter a design argument. The root's verification/ownership/formal-contract lane should be linked for verified storage and protocol work rather than duplicated here.

No literature survey can establish the target's nines or latency before its gate boundary and operating envelope are made concrete. This corpus gives the discussion actual mechanisms and actual counterexamples to compare, while preserving the operator's full-capture and speed objectives.
