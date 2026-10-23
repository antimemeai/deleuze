# Forward checks: 2024–2026 and emerging work

Searches performed 2026-10-23, after the main conceptual report. This supplement
checks whether the older roots lead toward new capture mechanisms, toward
selection of fewer events, or toward representations that answer queries without
retaining original records. It is not an exhaustive citation-index export.

[recent-bibliography.tsv](recent-bibliography.tsv) records seven additional
records: five acquired primary PDFs, one captured author announcement, and one
inaccessible PDF. These are separate from the 39-entry root bibliography so the
main report's reading/acquisition counts remain interpretable.

## Searches and closure state

| Root/front | Queries actually searched | Result and what remains open |
| --- | --- | --- |
| NanoLog successors | `"NanoLog" "2024" logging paper`; `"NanoLog" "2025" logging tracing paper`; `"NanoLog" "2026" paper logging -Zscaler -NSS` | Found a 2025 primary overhead study citing NanoLog and a 2024 compression branch. Many results were repeats of the 2018 paper or unrelated Zscaler product terminology; those were excluded. No claim of complete forward-citation closure. |
| Hindsight successors | `"Hindsight" "tracing" "2024" paper`; `"Hindsight" "NSDI" "2025" "tracing" ...`; exact Astraea, TraceMesh, and Tracezip title searches | Acquired the 2024/2025 papers and checked backward references to establish actual citation edges. Later tracing retention/compression work remains a growing front. |
| Sampling versus compression | `"TraceMesh" 2024 paper`; `"Tracezip" "Efficient Distributed Tracing" paper` | Confirmed different objectives: selecting representative traces versus reconstructing transmitted span data from shared compression state. No benchmark equivalence inferred. |
| New retention representations | `"PrvTel" NSDI 2026 tracing`; `"T-PACK" "tracing" "paper"` | PrvTel PDF acquired; T-PACK's author announcement dated July 2026 and publisher title/authors page checked. The latter is an upcoming NSDI 2027 paper, not a completed 2026 publication or read full text. |
| Recent log storage | `uslope yscope OSDI 2024 paper pdf`; exact title searches | Publisher abstract checked, PDF requests returned HTTP 403, including official `.net` alias. Its cited-reference list was not inspected. Follow the primary author/publisher access route rather than substituting a blog summary. |
| Recent I/O | `"High-Performance DBMSs with io_uring"` and direct author manuscript | The 2025 version is retained in the root bibliography. Front matter read; core evaluation remains open. |

## Citation edges verified in acquired primary text

| Recent work | Older work actually present in its text/reference list | Reading and classification |
| --- | --- | --- |
| [Astraea, 2024 author preprint](https://arxiv.org/pdf/2405.15645) | Log², Log20, X-Trace, Dapper, Hindsight preprint | Related-work/instrumentation-control sections and references read. It learns which spans to enable for performance diagnosis. This is selection of instrumentation, not preservation of every gate arrival. |
| [TraceMesh, 2024](https://arxiv.org/pdf/2406.06975) | Hindsight, X-Trace, Dapper, Pivot Tracing | Abstract, related work, and references read. It samples via evolving trace clustering and similarity representations. The clustering implementation/evaluation was not studied. |
| [Tracezip, ISSTA 2025](https://zbchern.github.io/papers/issta25.pdf) | Hindsight, X-Trace, Dapper | Overview, reconstruction-state synchronization, workload description, and references read. It reduces transmitted representation using a span retrieval tree and dictionary. These must reach the receiver before dependent data is released. This is a materially different route from omitting less interesting traces; restart/corruption/retransmission contracts still need study. |
| [Nõu et al., ICPE Companion 2025](https://atlarge-research.com/pdfs/2025-tracing-overhead-anou.pdf) | NanoLog, Dapper, Canopy, Hindsight | Methods/configuration and related work read. It separates tracing pipeline overhead in modern application experiments. Its borrowed numbers about other systems are not independent remeasurements and are not imported into the main performance table. |

These edges were checked in the locally extracted PDFs, not inferred from
search-engine “related papers” suggestions. A work can be topically related
without citing a root; the rows below are recorded separately for that reason.

## Related recent fronts with different fidelity contracts

[PrvTel, NSDI 2026](https://www.bu.edu/peaclab/files/2026/03/nsdi26fall-prvtel.pdf)
is explicitly a retained generative model rather than retained raw telemetry.
Its abstract/intro framing concerns private, accurate query answers over
reconstructed distributions. That can be valuable analytics literature, while
remaining different from exact reconstruction of individual instrumented
arrivals. Only front matter and abstract were read; this lane does not claim to
have evaluated its privacy proof, utility, or performance.

[μSlope's publisher abstract](https://www.usenix.org/conference/osdi24/presentation/wang-rui)
states a lossless semi-structured log representation with search without full
decompression. Its PDF was not obtained, so no verified NanoLog citation edge or
implementation result is claimed. It is a concrete recent storage/search front
to follow, not a primary capture-call benchmark.

The [T-PACK author announcement](https://lesleychou.github.io/#tpack-nsdi27)
announces a structure-aware generative compression paper for NSDI 2027; the
[publisher page](https://www.usenix.org/conference/nsdi27/presentation/chin) confirms
title and authors. A July 2026 news entry establishes that this was announced by
the acquisition date. Full text was not available from the captured publisher
page, and the direct HTML fetch returned HTTP 403; the retained original is the
author's announcement page. “Generative” alone is insufficient to decide whether
its representation preserves original records. Exactness, coding method, and
recovery assumptions remain unexamined.

## A concrete evaluation front to expand next

The root corpus compares producer microbenchmarks, closed-loop application
throughput, flight recorders, and storage-oriented tests. It does not yet supply
a coherent evaluation literature for Deleuze's overload and tail-latency claims.

Tracezip §5.1 calls its Locust workload “open-loop asynchronous.” That description
is not enough to establish whether request arrivals are independent of earlier
response completions: the actual task scheduling, user count, waits, and offered
load need inspection. This lane has not inspected the workload artifacts, so it
does not assert either open-loop correctness or a coordinated-omission defect.

The next conceptual unit should study primary work on open versus closed systems,
coordinated omission, offered-load timing, queue residence, sustained backlog,
burst envelopes, and tail estimates under loss. It should include concrete
load-generator artifacts and separate healthy no-contention latency from latency
when drain capacity is insufficient. That expansion is still open; no new
performance result is inferred from the current corpus.
