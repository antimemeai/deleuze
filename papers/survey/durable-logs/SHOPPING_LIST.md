# Durable-log survey: explicit gaps and next citation fronts

2026-10-23. This is a research backlog, not issue tracking or a request to install dependencies. Root owns `bd` issues and cross-lane prioritization.

## Inaccessible full text

- **Viewstamped Replication Revisited**, Barbara Liskov and James Cowling, MIT-CSAIL-TR-2012-021 (2012). Catalogue entry `viewstamped` records the original author URL. `https://pmg.csail.mit.edu/papers/vr-revisited.pdf` refused the connection; MIT DSpace's `https://dspace.mit.edu/bitstream/handle/1721.1/71763/MIT-CSAIL-TR-2012-021.pdf` returned HTTP 429. The Washington course mirror returned non-PDF bytes. No full-text summary was fabricated and no denied/error response is labelled as an acquired paper. Operator-supplied originals or a later successful institutional fetch can close this gap.

## Acquired originals awaiting substantive reading

The catalogue marks these `acquired-not-read`: Taurus; Poplar; vCorfu; NOVA; Paxos Made Simple; Chain Replication for Supporting High Throughput and Availability; CRAQ; End-To-End Arguments in System Design; BtrLog (2026 preprint). Titles/authorship are verified from the original/author source. Their substantive claims have not been imported into the lane's design reasoning. The two storage-failure empirical papers have abstract-only reading, not dataset/method review.

BtrLog is a particularly recent citation front. Check its full assumptions, current artifact release, and comparisons after reading; do not infer behavior from its title or abstract.

## Missing matching implementation versions

Eight pinned trees are acquired, but those HEADs are not automatically the artifacts used in older papers. No submodule, package, external toolchain, or benchmark-data closure was installed.

- **Historical PMDK `libpmemlog`:** the captured current PMDK tree lacks this old library. Acquire the exact old PMDK version used by the PMem I/O/Arcadia comparisons, retain its license, and inspect append/atomicity/recovery paths before comparing code to those papers. Current `libpmem` primitives are available and were sampled.
- **CORFU/Tango:** current CorfuDB is not the 2012/2013 implementation. Resolve the corresponding artifact commits or author-provided packages if reproducing historical claims. The current tree's selected log-file paths were inspected only to expose the persistence boundary.
- **Scalog, FuzzyLog/Dapple, Aether/Shore-MT, SiloR, Arcadia, PMem I/O primitives:** matching experiment artifacts were not acquired in this lane. Seek the papers' author/project artifact links and exact experiment revisions. For CrashMonkey, the paper points to `https://github.com/utsaslab/crashmonkey`; this repository has not yet been ingested.
- **NOVA:** original is acquired but unread; no matching kernel tree/commit, setup, or hardware data package was collected.
- **Pravega:** pinned repository architecture and selected operation-processor paths are acquired, including the diagrams referenced by that document inside the source archive. Its live documentation site was inaccessible through the browser tool; the pinned upstream source document supplies the primary text. Deployment configuration and BookKeeper/backend settings remain unreviewed.
- **Kafka/BookKeeper/Redpanda/LogDevice:** selected source paths and documentation reveal settings-dependent boundaries. Complete configuration-to-ack call graphs, deployment overrides, rollover/truncation/recovery tests, corruption repair, and multi-domain failure behavior are not yet reviewed. Some repository prose is historical; do not assume it overrides current code or an operator's deployment.
- **etcd Raft:** the core is acquired; an actual durable storage/transport integration and its error handling are separate material. Do not count example `MemoryStorage` as persistence.

## Citation fronts located in the read originals

The subsequent [recent/hardware expansion](forward-frontiers.md) adds nine acquired originals, including LazyLog/PALF 2024, SpecLog 2025, LogDrive/XLL 2026, SSD power-fault experiments and newer SSD field/monitoring work. LazyLog, LogDrive and selected power-fault sections were read; PALF remains acquired but unread, SpecLog introduction-only, and XLL/newer device papers abstract-only. Matching new implementation artifacts remain unacquired.

These are discovery leads, **not read summaries**. Resolve original bibliographic details and acquire the primary work before drawing conclusions. A paper's reference list may itself contain an incorrect year/venue; verify rather than copy blindly.

| Starting original | Next works or artifacts to locate | Why the question remains open |
|---|---|---|
| Aether references and §3 | DeWitt et al., *Implementation Techniques for Main Memory Database Systems*; Rafii/DuBois, *Performance Tradeoffs of Group Commit Logging*; Soisalon-Soininen/Ylonen, *Partial Strictness in Two-Phase Locking*; Chen, *FlashLogging* | Historical provenance and conditions behind safe early release, flush batching, and flash behavior |
| ARIES references | Earlier recovery/commit implementations, stable-storage mechanisms, media recovery and restart work | The full recovery lineage is much broader than the selected ARIES sections |
| CORFU references | Camelot, Zebra, chain replication; corresponding exact implementations | Shared-log lineage and its fault/reconfiguration contracts |
| FuzzyLog references | Li et al., *Just Say NO to Paxos Overhead: Replacing Consensus with Network Ordering*; Lloyd et al., *Stronger Semantics for Low-Latency Geo-Replicated Storage* | Ordering assumptions and regional failure envelopes |
| Scalog references | Zlog (`https://github.com/cruzdb/zlog`); network sequencers; *Liquid: unifying nearline and offline big data integration* | Alternative retained-log implementations and the producer/analytics boundary |
| Arcadia references | *NVRAM-aware Logging in Transaction Systems*; FLEX; Mojim; Query Fresh; remote persistence/flush hardware primitives | Exact integrity, replication, commit, and benchmark assumptions |
| PMem I/O | Prior logging protocols, old PMDK algorithms, precise device/platform contracts | CPU cache persistence, atomicity, alignment and performance are platform/version dependent |
| Pillai and CrashMonkey | Subsequent application crash-consistency, sync-error behavior, resource exhaustion and verified storage work | Current bounded crash tests do not cover every recorder/application/filesystem fault class |

The root's formal-contract/ownership lane owns verification roots such as FSCQ, Perennial, IronFleet, RustBelt, RefinedRust, and Verus. Link their actual acquired revisions and reading outcomes. In particular, root discovered that Perennial's newest `lean` branch explicitly excludes crash/recovery reasoning; it must not be substituted for the historical crash-reasoning artifact.

## Required operating evidence before numerical claims

No paper supplies this project's event-size distribution, offered-load envelope, burst duration, available CPU/NUMA budget, sync latency distribution, storage/failure domains, repair rate, archival horizon, or evaluation-reader access pattern. Acquire those from representative instrumented projects and the eventual operating environment when the operator chooses the design stage. Historical disk/corruption statistics are not deployment-specific nines.

No acquired source was compiled, installed, run, or adopted as a Deleuze dependency. No missing item above authorizes such adoption.
