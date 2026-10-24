# Deleuze: broad study before design

2026-10-23. The controlling objectives are to capture every event that reaches
the gate, at deployment-funded reliability measured in nines, and to do so
extremely quickly. Arrivals at a full or failing gate stay in that population.
Native Rust and polyglot interfaces expose the same responsibility. World-state
acquisition and evaluation fidelity motivate consumers of the retained stream.
The study informed the [first native-core design](../../design/native-core.md).
BLAKE3 is approved; network dependencies remain proposals. The literature below
continues to inform subsequent comparisons rather than imposing every mechanism
on the first build.

## What is here, and how deeply it has been read

The public study spans 189 lane bibliography entries, including duplicate
citations, specifications and primary documentation. This is not 189 distinct
papers read. Each bibliography states acquisition and reading depth. Source
observations are static; reference systems have not been built or benchmarked.
Private sources and identifying notes remain outside this repository and are
used only for inspiration. The repository records generic reasoning and public
citations.

| Study | Entries | Reading, code and provenance |
| --- | ---: | --- |
| Durable logs, WAL, replicated/shared logs, retry and persistence | 48 | [Report](durable-logs/report.md), [recent/device-failure fronts](durable-logs/forward-frontiers.md), [8 pinned implementation snapshots](durable-logs/references-manifest.md) |
| Fast capture, tracing, record/replay, queues and kernel I/O | 39 + 7 recent | [Study](fast-capture/study.md), [recent citation follow-up](fast-capture/recent-forward-citations.md), [8 pinned snapshots](fast-capture/references.md) |
| World-state, causality, temporal data, incremental computation and provenance | 47 | [Report and mechanism map](world-state/README.md), [8 pinned snapshots](world-state/references.tsv), [source reading](world-state/source-reading.tsv) |
| Invariants, ownership, concurrency and crash verification | 15 | [Report](verifiability/report.md), [7 pinned snapshots](verifiability/references-manifest.md), [Verbs objectives](verifiability/verbs-objectives.md) |
| Tamper evidence, forward integrity and verifiable history | 7 | [Supplement](integrity/README.md), [originals and reading depths](integrity/bibliography.tsv) |
| Offered load, tail latency, overload and measurement | 18 | [Study](measurement/study.md), [reading ledger](measurement/reading-ledger.md), [3 pinned snapshots](measurement/references.tsv), [bibliography](measurement/bibliography.tsv) |
| Gate content conversion, protocols and ownership boundaries | 8 | [Selected primary reading](gate-protocols/README.md), [originals and reading depths](gate-protocols/bibliography.tsv) |
| Adaptive batching, group commit and feedback control | 12 | [Synthesis and four reading lanes](adaptive-batching/README.md), [reading depths](adaptive-batching/bibliography.tsv); exact source manifests linked from the synthesis |

The generated [navigation catalog](catalog.tsv) preserves lane IDs, original
paths, URLs, hashes and reading status. Rebuild with
`python3 scripts/catalog-research.py`; lane bibliographies remain authoritative
for richer provenance. The original archives and documents remain ignored, with
tracked restoration metadata. A URL or Git HEAD cannot reconstruct historical
dirty bytes; retained originals and their hashes matter.

## Optimization objectives beneath capture and speed

[The Verbs reading](verifiability/verbs-objectives.md) derives objectives from its
current spec and operator decisions as well as its founding brief. Useful aims
include representations that exclude invalid states, explicit byte ownership and
transfer, narrow checked boundaries, invariant preservation across publication
and recovery, visible unsafe escapes, erased/static obligations where possible,
independent oracles, and concrete mechanical feedback with little ceremony.
Verbs is not ready to serve as a tool or dependency. Its C++ choices, tool limits
and disposable derived facts do not become Deleuze policy.

The study should make candidate costs visible: producer cycles and allocation,
tail latency, sustainable bytes/events per second, backlog and catch-up, space
amplification, recovery time, failure exposure, integration effort and deployment
cost. Each performance result needs its arrival population, payload distribution,
offered-load model, hardware and completion milestone. Counts of successful
admissions alone cannot evaluate the requested capture target.

## Remaining fronts and discussion

This is a broad initial map, not a claim to have exhausted the literature.
Backward/forward citation checks and recent 2024–2026 acquisitions are recorded
in the lane reports; many originals still need full reading. Open fronts include
broader scientific provenance/digital-twin/state-estimation literature, language
and FFI ownership at the gate, comparative schema/encoding evolution, executable
reference closures, deeper cryptographic threat models, and deployment-specific
device/failure evidence. Some lie in the acquisition gaps; others require more
reading of material already retained. See [the acquisition index](../../SHOPPING_LIST.md)
and lane-specific gap files. Beads tracks the work.

Useful questions for the next design discussion are the concrete arrival and
failure populations, the meaning of any receipt, what must survive each fault,
and which measured tradeoffs fit a deployment budget. The existing evidence
supports comparing candidate designs against those questions. It does not yet
support announcing one design as the answer.
