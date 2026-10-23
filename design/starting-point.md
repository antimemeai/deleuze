# Starting Deleuze's design from the acquired corpus

2026-10-23. Working synthesis for operator discussion under `deleuze-5n0`, not an
approved specification or implementation plan. The operator asked to begin with
the research already assembled. Claude remains stopped. No dependency chosen.

## Public evidence for the starting point

The [research map](../papers/survey/README.md) indexes 189 lane citation entries
with explicit reading depths. NanoLog studies compact producer capture and
background expansion; Aether studies pipelining and buffer contention; SiloR
studies parallel logging and durable epochs. The world-state literature
separates retained observations from derived state. These are mechanisms to
compare, not reproduced performance results for Deleuze.

Private-source identities, provenance and source-specific findings are excluded.
Generic ideas may inform the discussion; published claims cite public sources.

## The core we should investigate

The working hypothesis is a small capture engine that takes responsibility for
event data, records it, and exposes the recorded stream. Native Rust and foreign
entry paths should share that engine's ownership and failure semantics. Consumers
can build state, indexes, queries and evaluation views over the retained stream.
The first candidate for analysis is producer-local capture buffers with explicit
ownership, feeding batched recording. Compare it with direct serialized append:
extra handoffs and buffering must earn their cost. Writer sharding, collector
placement and the completion promise remain separate choices. This is a research
candidate, not an approved architecture.

The mechanism study has these controlling requirements:

- Every arrival at the chosen public gate remains in the capture population,
  including full/failing-gate arrivals and retries. Grouping attempts into a
  logical operation is a consumer operation and does not erase arrivals.
- Preserve producer-supplied content under its declared mapping, including
  producer-applied masks. Views must not silently replace retained content.
- Give caller data an explicit owner through handoff, recording, failure,
  cancellation and shutdown. A producer cannot return and mutate/free storage
  that an asynchronous recorder still needs without a valid ownership protocol.
- Keep internal values native and persisted records binary, per the working
  doctrine. No encoding/dependency choice follows from this requirement alone.
- State what an operation's return means. Buffer ownership, completed writes,
  synchronized storage and replica completion are distinct possible boundaries.
- Preserve uncertainty after interrupted or failed I/O. Recovery must not invent
  a definite success or definite absence merely because an append timed out.
- Keep retained originals for the promised retention horizon. Derived indexes
  and snapshots can be rebuilt; they cannot silently replace required originals.
- Measure capture, latency, backlog, recovery and resource cost together. Counting
  only successes, or ending before pending work drains, changes the result.

These are design criteria derived from the operator's goals and the research.
They are not a claim that one implementation can guarantee retention under every
possible fault or an unbounded offered load with finite resources.

## Independent choices that need comparison

| Choice | Candidates worth investigating | What separates them |
| --- | --- | --- |
| Producer work | Direct append; producer-local buffers with recorder handoff; owned payload transfer | Caller cycles/allocations, data lifetime, contention, time before retention and overload behavior |
| Physical writers | One journal owner; partitioned streams with several owners | Sustainable bandwidth, contention, recoverable boundaries, reader merge cost and required ordering |
| Completion | Return after owned handoff with separately observable progress; wait for the configured persistence boundary | Caller latency and concrete failure exposure. In both cases every gate arrival remains in accounting. |
| Placement | Embedded engine; collector process; remote retention | Process isolation, IPC/network cost, failure domains, and where each public gate actually is. Moving collection downstream cannot silently move an already declared gate. |
| Payload and metadata | Typed capture with binary encoding; opaque data with representation/version metadata; owned artifacts where promised | Fidelity, encoding work, schema evolution, payload size and the scope of referenced bytes |
| Write sequencing | Required monotonic write-assigned seq; physical coordination mechanism open | Scope across writers/logs, restart and failed-write behavior; no action-time or causal inference |
| Action relationships | Producer-supplied context and times in properties | Consumer interpretations remain separate from the log's write order |

Placement and writer topology are independent choices: a collector can own one
or several streams, and an embedded engine can also shard. Compare them as such.
Likewise, a failed arrival and a successful retry are two arrivals even if a
logical-operation view deduplicates the result.

## Where speed can come from

[Aether and SiloR](../papers/survey/durable-logs/report.md) motivate amortized
persistence work, pipelining and reduced shared-buffer contention. SiloR's
per-worker logging is within one database instance; its value-recovery shortcuts
are not a recipe for preserving all event history.

[NanoLog and the tracing references](../papers/survey/fast-capture/study.md)
motivate compact producer records and moving expansion off the producer path.
The producer timing and eventual persistence timing must remain separately
measured. A speedup achieved by sampling, eviction or changing the completion
promise must be identified as such when comparing capture reliability and cost.

[CORFU, Scalog and FuzzyLog](../papers/survey/durable-logs/report.md) show that
placement and ordering have different possible relationships. The operator now
requires a monotonically increasing write-assigned seq for the log. Physical
coordination and sequence scope still need design; that sequence emphatically
does not establish action-time or causal order. Producer times remain properties.

[Verbs-derived objectives](../papers/survey/verifiability/verbs-objectives.md)
motivate explicit ownership and legal state transitions, narrow checked foreign
boundaries, and obligations discharged without repeated event-time work where
possible. Concurrency and persistence ordering require evidence beyond what
the unfinished Verbs tool currently covers.

## Gate and event requirements

The operator clarified that the promise begins with a request reaching a gate.
Ship HTTP and gRPC gates, native Rust integration and custom-adapter interfaces
and guidance. All integrations use one capture path with no access to engine
internals. An event has an extremely small, strict record schema and associated arbitrary
properties outside that schema; unfamiliar or malformed input must still be
representable and persistable somehow. Large content can use property references. See
[gates and events](gates-and-events.md) for the requirements and unresolved
representation, ownership and artifact questions.

[The capture-contract candidate](capture-contract.md) proposes a concrete
record/property distinction and content ownership rules. Its
[bounded review](../papers/reviews/capture-contract.md) records the counterexamples
addressed so far. Event contents are retained once; whole-request archival and
duplicate original-body storage are excluded by the operator.

The [record profile](record-profile.md) defines gate packing, Deleuze-owned
seq/hash/write-time fields, the candidate received_at field, and configurable
event-type, producer-identity and authorized masked-data reference features with
proposed defaults.

Caller-visible return and persistence milestones still need defined semantics,
but they cannot narrow the earlier gate-arrival population. The desired nines
concern all arrivals, including losses before a completion boundary.

We can then compare concrete candidates against the same workload and failure
model: event sizes and representations, producer concurrency, sustained/burst
rates, required ordering, retention horizon, and available CPU/RAM/storage/network
resources. These quantities define an operating envelope, not a reason to omit
arrivals once it is exceeded. Failure/control capacity helps report exhausted
resources; recording a failure marker alone does not retain a missing payload.

The [measurement study](../papers/survey/measurement/study.md) gives the starting
method: independent offered-arrival records, separate observations of actual
gate arrivals and retained events, explicit pending/failed/unknown outcomes,
backlog and drain, and recovered payload checks for the fault being tested.
No benchmark or acceptance threshold is implemented by this note. Design and
plan reviews still precede production code.
