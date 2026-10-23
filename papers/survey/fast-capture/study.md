# Fast capture and faithful replay: what the acquired systems actually promise

This is a study of primary literature and pinned source, not a proposed
architecture. The operator's target is every arrival at a Deleuze gate, with
deployment resources paying for reliability and very high speed. A tracing
system that elects to keep fewer arrivals solves a different collection problem,
even when its transport, encoding, or analysis techniques are useful here.

The central distinction in this corpus is between an application returning from
instrumentation, a recorder receiving bytes, storage completing a write, storage
acknowledging synchronization, a remote copy being acknowledged, and a reader
reconstructing state. Published numbers measure different points in this chain.
No number below measures Deleuze.

## Literature lineages

| Lineage | Acquired primary roots and successors | The question it answers | Boundary that remains separate |
| --- | --- | --- | --- |
| Deferred logging work | NanoLog; Log20's runtime; JFR | What can be removed from each instrumented call and done once or later? | Visibility and durability of queued bytes; schema/dictionary availability |
| Kernel/user-space tracing | Desnoyers thesis; LTTng-UST manual/code; Linux ftrace, relay, perf, BPF ring docs/code | How can reentrant instrumentation publish records with little interference? | Lost-record behavior, producer progress assumptions, consumer scheduling, persistence |
| Shared-memory trace services | Perfetto buffers/service/V2 design | How can many untrusted producers write efficiently while preserving packet sequences? | Service commit, flush, file write, packet loss, and storage synchronization |
| Causal tracing | Magpie; Stardust; X-Trace; Dapper; Pivot; Canopy; CMU tracing-design report | Which records belong to the same execution, including asynchronous and cross-host work? | Which dependencies are actually instrumented; sampling and omitted payloads |
| Retroactive retention | Hindsight; JFR flight-recorder configuration | How can recent detailed data be kept cheaply until a symptom selects it? | Event horizon and deliberate eviction; this is not retention of every arrival |
| Record/replay | ReVirt; R2; Scribe; DoublePlay; rr; Eidetic Systems | What nondeterminism must be recorded to reproduce a stated execution boundary? | Unrecorded shared memory, races, devices, external systems, and changed execution schedules |
| Producer/consumer machinery | Michael–Scott queues; Disruptor; Linux/BPF rings; liburing | How do memory ownership, ordering, batching, and I/O submissions advance? | A queue operation is not a durable event acknowledgment; progress assumptions differ |

The bibliography includes unread roots deliberately: a citation discovered during
snowballing is not silently promoted into a conclusion. NanoLog's related work
points toward ETW/WPP; Dapper points toward Magpie and X-Trace; Hindsight points
back toward causal tracing and sampling; rr points toward earlier replay systems;
the LTTng dissertation supplies the synchronization lineage. Those are expansion
paths, not evidence of exhaustive coverage.

## Low overhead does not imply selection-free capture

[NanoLog](https://www.usenix.org/system/files/conference/atc18/atc18-yang.pdf)
§§3–4 separates static call-site metadata from dynamic values, records into one
SPSC staging buffer per application thread, batches ownership observations, and
defers formatting and global timestamp sorting. Its format carries the dictionary
needed for later expansion. The interesting reduction is in work performed per
arrival; it does not depend on randomly omitting arrivals. Producer calls may
block when the recorder cannot drain buffers. This makes the overload behavior
part of the performance claim, rather than a detail hidden behind “asynchronous.”

[Log20](https://www.eecg.utoronto.ca/~stumm/Papers/Zhao-SOSP17.pdf) solves placement
under an overhead budget using execution-path information. Its runtime library
separately has thread-owned containers, minimal per-call synchronization, and
flushing near request boundaries. Its placement model treats paths as collections
of executed basic blocks rather than their full ordered sequence. The placement
objective and the runtime mechanism must be evaluated independently for Deleuze:
reducing instrumentation coverage is different from efficiently recording an
already instrumented arrival.

[Log²](https://www.usenix.org/system/files/conference/atc15/atc15-paper-ding.pdf)
§§3–4 makes runtime selection explicit: local utility filters discard requests,
then a global stage ranks retained candidates and emits only those fitting a
volume budget. Its performance-diagnosis objective is to retain useful outliers,
not every monitored occurrence. It is useful literature about the cost of
selection, adaptation, and batched work, but its discard policy cannot establish
the operator's capture requirement.

[JEP 328](https://openjdk.org/jeps/328) describes thread-local event buffers feeding
a global circular system, self-description, retention policy, and configurable
event thresholds. Its stated overhead target is a particular default configuration
on SPECjbb2015. That target neither covers arbitrary custom events nor guarantees
retention of every event. The API's event commit, the recorder's buffer movement,
and a persisted recording are separate operations.

[Hindsight](https://www.usenix.org/system/files/nsdi23-zhang-lei.pdf) §§3–5 separates
cheap local data generation from costly ingestion. Applications write into
shared-memory buffers; agents index metadata, propagate trigger breadcrumbs, and
retrieve selected whole traces. Its event horizon is finite, and missing pool
capacity leads to a discarded null buffer. Coordinated whole-trace eviction
protects coherence of retained traces rather than preserving all traces. Its
techniques illuminate capture cost and control/data separation; its retention
objective differs from Deleuze's.

## Rings make their execution assumptions visible

The [LTTng dissertation](https://lttng.org/pub/thesis/desnoyers-dissertation-2009-12-v25.pdf)
§§5.3–5.4 distinguishes reservation, writing, commit counters, sub-buffer handoff,
and reader wakeup. It moves relatively expensive cross-CPU work toward handoff
rather than paying it for every event. Source-level probes can keep values
available that breakpoint instrumentation would find optimized away. The
dissertation's algorithm arguments have specific execution and memory-model
assumptions; they are not a proof of the current LTTng-UST revision or a future
Rust implementation.

The [current LTTng manual](https://lttng.org/docs/v2.14/), “Event record loss mode,”
states the tradeoff directly: default tracing prioritizes low perturbation and
may lose records. Discard mode counts lost records; overwrite mode records packet
sequence gaps but does not necessarily know how many individual records were
inside a lost packet. Blocking with a timeout does not itself eliminate loss
after that timeout. A gap signal and a known number of missing arrivals are
different information.

Linux's [lockless ring design](https://docs.kernel.org/trace/ring-buffer-design.html)
assumes per-CPU writers that nest like interrupt-stack frames. An inner writer
can finish while the outer writer still prevents a full commit. This is not an
arbitrary collection of independent user threads writing concurrently to one
ring. Its overwrite and producer/consumer modes choose different records to lose
when space is unavailable.

The [BPF ring design](https://docs.kernel.org/bpf/ringbuf.html) preserves a shared
reservation order across CPUs. Reservation is serialized; commits are independent,
but consumers cannot pass an earlier unfinished reservation. NMI reservation can
fail because a lock is unavailable even when memory has space. Thus bounded
allocation, lack of sleeping, lock freedom, reentrancy, publication order, and
reader progress are not interchangeable properties.

[Disruptor's technical document](https://lmax-exchange.github.io/disruptor/disruptor.html)
separates storage from sequences and consumer dependencies, preallocates reusable
events, and exploits batching. Its current page combines historical text with
later benchmark tables. Those producer/consumer pipeline measurements should not
be treated as disk logging results or silently attributed to 2011 hardware.
Michael–Scott's original queue paper remains an acquisition gap; a cached search
result is not a locally retained original.

## Shared-memory efficiency needs a loss and flush vocabulary

[Perfetto's dataflow documentation](https://perfetto.dev/docs/concepts/buffers)
distinguishes per-producer shared memory, service-owned central buffers, and
optional kernel buffers. Direct producer serialization avoids a writer-side copy;
the service still copies data into central storage. Buffer sizing depends on
bursts and consumer descheduling, not just average event rate. A session flush
collects outstanding producer chunks into service buffers; it is not by itself
a filesystem durability contract.

The [TraceBuffer V2 design](https://perfetto.dev/docs/design-docs/trace-buffer)
tracks producer/writer sequences, packet fragments, deferred patches, and reasons
for loss. It publishes complete packets and reports sequence losses, while trying
to preserve later valid packets after malformed data. Its roughly inter-sequence
ordering is not a causal order. This is a concrete model of explaining degraded
capture without presenting incomplete packets as complete observations.

The [io_uring manual](https://man7.org/linux/man-pages/man7/io_uring.7.html) makes
submission, completion, return codes, buffer lifetime, and ordering separate
concerns. Submission order does not generally impose completion order. A write
completion says what the equivalent write returned; synchronization has its own
operation. The [fsync/fdatasync manual](https://man7.org/linux/man-pages/man2/fsync.2.html)
also distinguishes file synchronization from directory-entry synchronization and
documents errors that can appear only at synchronization time.

The acquired [2025 author manuscript on DBMS io_uring use](https://www.informatik.tu-darmstadt.de/media/systems/pdf_publications/iouring_vldb.pdf)
is a recent expansion lead. Its front matter warns that replacing an existing I/O
API alone is insufficient. The evaluation is not read in this lane, so its speedup
figures are not used as evidence here. Its PDF still contains a placeholder PVLDB
DOI/citation; the bibliography records the actual arXiv version rather than
inventing a final publication DOI.

## Causal context and replay are stronger than timestamp sorting

[Dapper](https://research.google.com/archive/papers/dapper-2010-1.pdf) §§2 and 4
instruments shared RPC, threading, and callback libraries so context survives
asynchronous work and crosses C++/Java processes. It samples traces, bounds
application annotations, and omits RPC payloads. It is a successful account of
deploying a specific observation boundary, not a record of all state entering
each service. A small producer cost can coexist with much later central
availability.

[X-Trace](https://www.usenix.org/legacy/events/nsdi07/tech/full_papers/fonseca/fonseca.pdf)
§§2–3 propagates task metadata across protocol layers and along downstream
operations, then reconstructs reports offline. The instrumented boundary must
know which operations continue a task. Causal context must survive queued or
forwarded work; timestamps alone do not recover links that were never emitted.

[Pivot Tracing](https://jonathanmace.github.io/papers/mace2015pivot.pdf) §§3–4
propagates query tuples as baggage and evaluates happened-before joins in the
execution rather than performing all correlation centrally. Its definition
restricts the relevant causal relation to one request; it explicitly notes that
cross-request causality would require extension. Aggregations and most-recent
filters are query results, not replacements for the underlying observations.

The [CMU tracing-design report](https://www.pdl.cmu.edu/PDL-FTP/SelfStar/CMU-PDL-14-102.pdf)
§§3–5 distinguishes the request that triggers background work from the request
that originally submitted it, and examines aggregation/fork/join effects.
Sampling an initiating request can imply retaining a much larger population of
downstream events when batched work combines requests. This helps explain why
thread context or a single parent span is not a complete account of state
dependencies.

[rr's deployability report](https://arxiv.org/pdf/1705.05937) §§2–6 defines a
user/kernel observation boundary and records syscall results plus asynchronous
nondeterminism. It schedules one application thread at a time to avoid recording
arbitrary cross-core races, trading parallel workload speed for a simpler replay
contract. Scratch buffers and syscall interception handle effects that otherwise
race with executing threads. This is exactness relative to a stated boundary,
not a claim to reproduce an independently running distributed world. The paper's
2017 ARM limitation is historical: the acquired current README lists supported
AArch64 microarchitectures.

[Eidetic Systems](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-devecsery.pdf)
§§3–4 groups processes for replay and records dependencies between groups so it
can answer later lineage queries. Its treatment of data races is explicit:
divergence can require additional instrumentation for later runs. Its model-based
compression and selective re-execution are relevant to world-state acquisition,
but a useful retrospective query does not automatically establish faithful replay
of a first occurrence outside the recorded contract. R2, Scribe, DoublePlay, and
ReVirt remain acquired or unavailable expansion roots, not fully studied here.

## Performance claims with their measured boundary attached

| Source and location | Reported experiment | Event shape and hardware | What the result measures; limitation |
| --- | --- | --- | --- |
| NanoLog §5.1, Table 1, Table 4 | About 8 ns median producer invocation for several microbenchmarks | Repeated fixed call sites; Xeon X3470, four 2.93 GHz cores; 24 GB RAM; two Samsung 850 PRO SATA SSDs | Enqueue-side call latency, not synchronization latency for that event. Arguments and recorder pressure change it. |
| NanoLog §5.1, Figure 5 | Up to about 80 million messages/s | Simplest repeated static message compacts to a few bytes; logging threads varied 1–16 and the maximum was selected; same machine | Aggregate throughput includes final flush for NanoLog. The caption excludes flush from Log4j2's comparator. Do not generalize to arbitrary payloads or all gate arrivals. |
| Hindsight Appendix A.3, Figure 9 | Memory-bandwidth-scale client tracepoint throughput | `begin`, 100 tracepoints, `end`; 4–4000-byte payload sweep; 1–64 threads; 32 kB buffers; 1 GB pool. Platform descriptions name Dell R920/M620 machines but do not fully specify CPU microbenchmark details. | Writes into rotating shared memory, compared with STREAM; not durable retained throughput. Its event horizon and selective collection remain in force. |
| rr §4 | Warm-cache workload runtime relative to unrecorded baseline | Dell XPS15, quad-core Intel Skylake, eight SMT threads, 16 GB RAM, 512 GB SSD/Btrfs/Fedora 23; six runs, first discarded | Whole application recording/replay overhead. Highly parallel `make` behaves very differently from the low-parallelism cases; no event-ingest rate follows from this. |
| JEP 328 goals | Default overhead target on SPECjbb2015 | JEP does not supply a controlled hardware/event-shape result for arbitrary custom events | A design target for its default event configuration; not a measured universal bound. |

No throughput figure here includes a stated Deleuze replication factor, recovery
time, gate-loss probability, or cross-language serialization workload. Those
measurements remain to be defined when the operator has selected contracts and
deployment assumptions.

## What the pinned code adds to the papers

Exact revisions, licenses, archives, and inventories are in
[references.tsv](references.tsv). These are static observations; no reference
test suite or benchmark was run.

| Reference | Examined path and behavior | Implication for further study |
| --- | --- | --- |
| NanoLog | `runtime/Config.h:28` uses `O_DSYNC`; `RuntimeLogger.cc:720` waits for background completion; `:752` reserves producer space; `:568` prints AIO failures and then advances completion bookkeeping | Do not call its default disk path buffered-only. Separately investigate error propagation and partial AIO results before treating its “guaranteed” terminology as Deleuze's contract. |
| LTTng-UST | `src/common/ringbuffer/frontend_api.h:158` returns disabled/space/size/I/O errors; `:282` separates commit from reservation; loss counters are populated on the fast path | Error code, lost-record count, packet handoff, and application continuation are distinct contracts. The current source is not automatically covered by the dissertation's old model. |
| Perfetto | `src/tracing/core/trace_writer_impl.cc:183` marks shared-memory loss; `shared_memory_arbiter_impl.cc:116` handles drop/stall/stall-then-drop; `:248` prevents IPC-thread stall deadlock; `src/tracing/service/trace_buffer_v2.cc:106` tracks loss reasons | Producer stalls can block the work needed to drain the producer. Detailed loss signaling is still not loss prevention. |
| Hindsight | `client/src/buffer.c:160` allocates a null sink when no pooled buffer is available; `tracestate.c:124` rotates buffers; `agent/pkg/agent/datamanager.go:91` evicts untriggered traces | Its measured fast path includes a deliberate sacrifice under pool pressure and selective history retention. |
| Pivot/X-Trace framework | `pivottracing/agent/src/main/java/edu/brown/cs/systems/pivottracing/agent/advice/baggage/BaggageAPIImpl.java` archives baggage on splits and merges on joins; sibling `advice/output/EmitAPIImpl.java` publishes query results periodically and catches failures | Context propagation and derived query output need different failure/retention treatment from source-event capture. |
| rr | `src/record_syscall.cc` handles scratch-buffer/syscall-buffer paths; `src/TraceStream.cc:408` writes execution records; `README.md:16` states current hardware assumptions | Reproduction depends on the exact interposition boundary and machine support, not merely retaining log statements. |
| liburing | `src/queue.c:247` publishes submission-queue entries with the required ownership ordering; `src/include/liburing.h:804` prepares an explicit fsync operation | Ring publication, request submission, write result, and synchronization completion require separate observations. |
| Linux v6.12 selection | `kernel/trace/ring_buffer.c` implements nested-writer publication; `kernel/bpf/ringbuf.c` implements shared reservations; `io_uring/sync.c:70` executes fsync in a blocking context and returns its result | Fast submission does not remove storage synchronization work. Concurrency/reentrancy properties must be checked for the actual producer execution model. |

## Questions this lane leaves open

Preserving every gate arrival rules out silently borrowing the selection policy
from a sampled tracing system. It does not settle the gate's byte ownership,
acknowledgement, recovery, ordering, overload, or deployment contracts. The corpus
contains examples of each, but no single cited paper demonstrates them all for
Deleuze's proposed scope.

Work still needed includes signal/interrupt capture, allocation and variable-size
event bounds, dictionary/schema recovery, timestamp calibration, cross-language
ownership, and the relation between distributed causality and physical storage
order. The fastest available no-contention path is only one fault/performance
class; sustained bursts, stalled producers, dead recorders, storage errors, and
recovery demand their own direct evidence. See [gaps.md](gaps.md) for concrete
unsearched families and unavailable originals.

After completing this report, explicit 2024–2026 forward checks established new
citation edges and a separate compression/retention front. Those checks, their
limits, and a proposed overload-evaluation research unit are recorded in
[recent-forward-citations.md](recent-forward-citations.md).
