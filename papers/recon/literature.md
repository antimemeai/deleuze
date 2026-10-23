# Primary literature and external references

2026-10-23. Independent frumentarii lane. This is source acquisition and reading, before design. Deleuze records everything explicitly instrumented to it, serves native Rust and polyglot callers, and supplies world-state observations and evaluation material. No format, storage engine, crate, transport, or dependency is selected here.

Four complete upstream source archives, four original PDFs, and nine documentation originals are acquired. Exact revisions, original-byte SHA-256 values, rights notes, and restoration commands are in [external-manifest.md](external-manifest.md). Working trees and document snapshots are under ignored `quarantine/sources/external/`; original archives are under ignored `quarantine/archives/external/`; original PDFs are under ignored `papers/pdfs/`. HTML-to-text and PDF-to-text files are reading aids, not replacement originals.

## Reading scope

The notes below come from focused reading of the named specification sections, the four papers' relevant causal/recovery/simulation/replay sections, and selected source files. They do not claim cover-to-cover review of the entire corpus. LevelDB's log format, reader paths, selected corruption tests, and write options were inspected; redb's design document and license were inspected, but its engine implementation remains unread. OpenTelemetry log/common protobuf schemas were inspected. The acquired rr implementation remains unread beyond attribution/license checks; replay findings come from the paper. Unmentioned files in all four extracted trees remain acquired, not reviewed. No reference implementation was built or executed.

## Findings to carry into operator discussion

**An observation log and a replay system make different claims.** rr captures all nondeterministic inputs across a chosen execution boundary; FoundationDB controls network, disk, clock, randomness, and scheduling inside simulation. Interpretation for Deleuze: preserving emitted observations can support faithful analysis, but execution replay requires the instrumentation boundary and captured external inputs to be specified. Recording successful outputs alone cannot establish that claim. Sources: [rr §2](https://www.usenix.org/system/files/conference/atc17/atc17-o_callahan.pdf), [FoundationDB §4](https://www.foundationdb.org/files/fdb-paper.pdf).

**A log-shaped storage component may deliberately forget history or suppress evidence of an interrupted write.** RocksDB reclaims obsolete WALs; LevelDB ignores incomplete final logical records. Interpretation: database recovery behavior is valuable source material, but the event-retention and interruption-reporting behavior needs separate examination for world-state acquisition. Sources: [RocksDB WAL format](https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-File-Format), [pinned LevelDB reader](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/log_reader.cc).

**Standard telemetry delivery can intentionally discard records.** OpenTelemetry's batch processor drops logs at its queue bound; its exporter contract drops a failed batch. Interpretation: an OpenTelemetry adapter could provide valuable context and interoperability, while ordinary SDK delivery does not itself establish complete acquisition. Sources: [Logs SDK, batching and ExportResult](https://opentelemetry.io/docs/specs/otel/logs/sdk/).

**Correlated records, timestamp order, and causality are distinct.** Lamport's happened-before relation is partial; a total order can extend it without discovering missing causal relationships. W3C context propagates request identity. Interpretation: trace IDs and sorting by wall time do not establish every event's predecessor relationships. Sources: [Lamport 1978](https://lamport.azurewebsites.net/pubs/time-clocks.pdf), [W3C Recommendation 2021](https://www.w3.org/TR/2021/REC-trace-context-1-20211123/).

## Source reading notes

### 1. SQLite: atomic commit and WAL

[Atomic Commit](https://www.sqlite.org/atomiccommit.html) and [Write-Ahead Logging](https://www.sqlite.org/wal.html), SQLite authors; living pages acquired 2026-10-23. Read especially atomic-commit hardware assumptions, flushing, and crash testing; WAL §2.3 performance/durability.

Source claim: atomic commit is implemented over explicit storage assumptions, including caches and incomplete writes. WAL `synchronous=NORMAL` can omit per-transaction synchronization; commits can then roll back after power failure or hard reset while consistency remains intact.

Interpretation: an append returning successfully, an append surviving a process crash, an append surviving power loss, and a readable consistent file are separate facts. Establish the desired claim before comparing throughput. These pages describe SQLite mechanisms, not a Deleuze implementation proposal.

### 2. LevelDB: physical records, recovery, and corruption tests

[Format](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/doc/log_format.md), [reader](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/log_reader.cc), [tests](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/db/log_test.cc), [write options](https://github.com/google/leveldb/blob/bb74ef739973a70ca9f0d90788a1da89b788c0ad/include/leveldb/options.h). The LevelDB Authors; revision `bb74ef739973a70ca9f0d90788a1da89b788c0ad`.

Source claim: 32 KiB blocks contain checksummed full or fragmented records. Block boundaries support recovery without heuristic byte scanning. The reader returns EOF for truncated final headers/payloads and discards an unfinished fragmented record. `WriteOptions::sync` defaults to false. Tests inspect dropped bytes and corruption reports, not merely successful reads.

Interpretation: physical framing and partial-write recovery have concrete adversarial examples worth studying. EOF does not necessarily mean the producer completed every intended event, and resynchronization does not itself mean the intervening loss was acceptable.

### 3. RocksDB: recycled WAL identity and retention

[Write Ahead Log File Format](https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-File-Format), RocksDB authors; page displays last edit 2020-10-19, exact acquired bytes in manifest.

Source claim: WALs reconstruct memtables after failure; obsolete WALs are archived and eventually purged. The recyclable format includes a log number so readers can distinguish records from a previous writer in reused storage.

Interpretation: storage reuse introduces stale-byte ambiguity beyond torn writes. Database WAL retention differs from retaining instrumented observations over their useful lifetime. Check the format's identity and lifecycle assumptions when examining local audit implementations.

### 4. Pillai et al.: filesystem persistence is part of the protocol

[All File Systems Are Not Created Equal](https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-pillai.pdf), OSDI 2014, pp. 433–448. Authors and PDF hash in manifest. Read §§2–4 on persistence properties and application protocols.

Source claim: the study measured different persistence behaviors across six Linux filesystems and found 60 crash vulnerabilities across eleven applications. Correct application behavior depended on filesystem properties that were easy to assume incorrectly.

Interpretation: a crash argument must state its actual filesystem contract, including creation, rename, file-size updates, and synchronization. A deterministic storage model can exercise that stated contract; it cannot demonstrate that every deployed filesystem implements it. The paper's measured filesystem versions are historical, not claims about current macOS or Linux behavior.

### 5. Lamport: causal observation and clock order

[Time, Clocks, and the Ordering of Events in a Distributed System](https://lamport.azurewebsites.net/pubs/time-clocks.pdf), Leslie Lamport, CACM 21(7), July 1978, pp. 558–565. Read partial ordering and logical clocks.

Source claim: same-process order, sending before receipt, and transitivity generate happened-before. Logical clocks preserve that relation; an additional deterministic tie-break can produce a total order.

Interpretation: local append order is an observation order at that recorder. A total presentation order does not prove causality or simultaneous observation across hosts. Preserve the distinction when discussing cross-project and tailnet acquisition.

### 6. W3C Trace Context: interoperability and sampling flags

[Trace Context, Recommendation 23 November 2021](https://www.w3.org/TR/2021/REC-trace-context-1-20211123/), W3C Distributed Tracing Working Group. Read `traceparent`, `tracestate`, flag processing, and privacy sections.

Source claim: `traceparent` carries version, trace ID, parent ID, and flags; `tracestate` carries vendor state. Sampling flags are advisory information passed between tracing participants.

Interpretation: these are useful external context conventions. Correlation identifiers do not document every instrumented state transition or guarantee delivery, and an acquisition contract must address what happens when surrounding tracing is unsampled.

### 7. OpenTelemetry: event time, observation time, and typed values

[Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/), living specification acquired 2026-10-23; [LogRecord schema](https://github.com/open-telemetry/opentelemetry-proto/blob/6c9dd787c3c5a5dae7d18ece0927581d5687f9c0/opentelemetry/proto/logs/v1/logs.proto) and [AnyValue](https://github.com/open-telemetry/opentelemetry-proto/blob/6c9dd787c3c5a5dae7d18ece0927581d5687f9c0/opentelemetry/proto/common/v1/common.proto), revision `6c9dd787c3c5a5dae7d18ece0927581d5687f9c0`.

Source claim: source time and collector observation time have different semantics; log records also identify resource, instrumentation scope, event class, and optional trace context. The schema records dropped-attribute counts. `AnyValue` has signed 64-bit integer, floating-point, bytes, arrays, and key/value-list variants; there is no unsigned-integer variant.

Interpretation: absence, numeric range, raw bytes, and producer-versus-observer identity need explicit treatment at a polyglot boundary. A generic nested value is not proof that arbitrary native types round-trip unchanged.

### 8. OpenTelemetry SDK: bounded queues, failure, and flushing

[Logs SDK](https://opentelemetry.io/docs/specs/otel/logs/sdk/), OpenTelemetry Authors, living specification acquired 2026-10-23. Read batching, `ExportResult`, `ForceFlush`, and record limits.

Source claim: the batch processor's queue is bounded (specified default 2048 records); excess logs are dropped. `ExportResult::Failure` drops the batch. Force-flush honors a timeout and can skip or abort export work.

Interpretation: interoperability should be evaluated separately from completeness. An attempted export, a flush request, and durable acceptance by a receiver are different observations. A telemetry bridge inherits only the delivery behavior actually implemented around it.

### 9. rr: replay requires capturing the boundary

[Engineering Record And Replay For Deployability](https://www.usenix.org/system/files/conference/atc17/atc17-o_callahan.pdf), Robert O’Callahan, Chris Jones, Nathan Froyd, Kyle Huey, Albert Noll, Nimrod Partush; ATC 2017, pp. 377–389. Read §2, especially §§2.1–2.4. [Current acquired source](https://github.com/rr-debugger/rr/tree/c145f2f80e63e9627b79deed6e0d3eae0d658e38), revision `c145f2f80e63e9627b79deed6e0d3eae0d658e38`.

Source claim: rr records inputs and nondeterminism crossing the user/kernel boundary and constrains execution to one application thread at a time; recorded scheduling and signal positions matter. Its paper describes a particular historical hardware/OS envelope.

Interpretation: evaluation fidelity is a boundary claim: which inputs, environment state, tool responses, randomness, and timing influence the evaluated computation? Capturing a random seed cannot reconstruct uncaptured network responses. Current rr source and 2017 paper are distinct revisions; historical platform statements must not be projected onto today's implementation.

### 10. FoundationDB: deterministic simulation and explicit oracles

[FoundationDB: A Distributed Unbundled Transactional Key Value Store](https://www.foundationdb.org/files/fdb-paper.pdf), Zhou et al., SIGMOD 2021, DOI 10.1145/3448016.3457559. Read §4, including its limitations.

Source claim: the simulator runs real database logic with controlled network, disk, time, and pseudorandomness. Fault injection includes unsynchronized disk-write corruption. Workloads and assertions check contracts; simulation does not test everything outside its controlled implementation, including arbitrary third-party libraries or OS contracts.

Interpretation: reproducible failure schedules and contract-derived oracles are directly relevant to a durable recorder. Simulation fidelity is bounded by the modeled environment. An event stream can preserve observations from a run without proving those observations fully determine another run.

### 11. redb: consistency, durability, and the storage assumptions

[Design document](https://github.com/cberner/redb/blob/b04b3d557dbbe479d898f0e4a5375ce3667a3f7b/docs/design.md), Christopher Berner and contributors; revision `b04b3d557dbbe479d898f0e4a5375ce3667a3f7b`. Read commit strategies and underlying-media assumptions.

Source claim: the Rust engine uses copy-on-write B+trees and checksums, with distinct non-durable, single-sync checksum, and two-phase commit strategies. Its document states byte-atomicity, durability after synchronization, and powersafe-overwrite assumptions; it also discusses non-cryptographic checksum limitations.

Interpretation: this is a useful contrasting Rust persistence design, not an append-log recommendation. A corruption checksum and an adversarial tamper-resistance claim have different failure models. Nothing here selects redb for Deleuze.

### 12. Cap’n Proto: framing, schema evolution, and decoder limits

[Encoding](https://capnproto.org/encoding.html), Kenton Varda and contributors; living page acquired 2026-10-23. Read defaults, serialization framing, canonicalization, and security considerations.

Source claim: the format specifies little-endian words, segments, pointers, and default values. Canonical representation exists but is not ordinary encoder output. Readers must validate pointers and bound traversal/depth; small encodings can otherwise amplify work.

Interpretation: a cross-language binary format needs exact malformed-input and resource-limit semantics as well as an encoder. Semantic equivalence and identical bytes differ; that distinction matters if evaluation fixtures or integrity mechanisms compare serialized data.

### 13. Arrow: language-independent arrays and stream framing

[Columnar Format](https://arrow.apache.org/docs/format/Columnar.html), Apache Arrow contributors; living page acquired 2026-10-23. Read serialization/IPC, stream/file formats, buffers, and types.

Source claim: Arrow describes typed columnar buffers and language-independent IPC metadata, emphasizing sequential analytics and relocatable data. Streams use schema and record-batch messages with explicit framing; it leaves mutation coordination to implementations.

Interpretation: analytical export and record ingestion have different access patterns. Arrow is useful comparison material for bulk evaluation readers; it does not supply append durability or recorder coordination by virtue of being a binary interchange format.

### 14. CBOR: explicit value semantics and deterministic encoding

[RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.txt), Carsten Bormann and Paul Hoffman, December 2020, STD 94. Read §2, §4.2, and §5.

Source claim: the RFC distinguishes well-formed, valid, and application-expected values; it supports byte strings and typed numerical values. Deterministic encoding adds requirements beyond generic CBOR validity. Duplicate keys and numeric equivalence require application-level decisions.

Interpretation: compact self-description does not eliminate schema or value semantics. Polyglot conformance must cover distinctions such as missing versus null, byte versus text, signed versus unsigned range, and floating-point edge values. This is comparison material, not a codec decision.

## Research questions exposed by the corpus

1. What does an emitter learn when a write returns: accepted into memory, written to a file, synchronized, or durably acknowledged elsewhere? What can it observe after recorder failure?
2. Which losses are representable: queue overflow, rejected values, partial writes, lost connection, shutdown timeout, and an unobserved producer crash? A failed sink cannot necessarily record its own failure through that same sink.
3. What state must be recorded for the intended evaluations, and what claim is made about replay, inspection, or reconstruction? An uninstrumented fact is not acquired merely because other events mention it.
4. How should origin clock, observer clock, producer sequence, recorder sequence, and explicit causal links be interpreted? Which are optional or unknowable?
5. What does cross-language value fidelity mean for unknown fields, invalid text, raw bytes, large integers, special floating-point values, repeated keys, and schema evolution?
6. How much history must remain readable independently of indexes, database compaction, or a functioning producer? Can a reader distinguish a clean close from an interrupted stream?

These questions prepare discussion; they do not fix APIs, failure policies, data models, storage layouts, or dependencies. Local and remote project archaeology should be compared against them before design begins.
