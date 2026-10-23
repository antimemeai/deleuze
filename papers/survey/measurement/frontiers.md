# Citation closure and remaining fronts

This is a bounded wave of 18 bibliography records plus two auxiliary retained HTML originals, and three pinned reference snapshots. It does not claim exhaustive coverage. Search date: 2026-10-23.

The completed conceptual chain is independent versus completion-coupled arrivals → intended versus dispatched timestamps → completion/loss/pending populations → cumulative backlog and drain → sustaining failure loops → uncertainty and observer effects. The older roots do not alone settle current benchmark correctness.

## Explicit 2024–2026 checks

Searches included `TailBench benchmark suite evaluation methodology latency critical applications`, `Metastable Failures Distributed Systems`, `TailBench++ authors 2025`, `coordinated omission benchmark 2024 2025 2026 paper`, `Open Versus Closed 2024 benchmark`, `TailBench 2026 load benchmark`, and `Scientific Benchmarking 2025 measurement`. General search results included unrelated open/closed AI material; no conclusions about citation absence follow from that noise. Primary publisher/author pages, retained papers and current source were used to validate relevant findings.

| Root/front | Recent primary continuation located | Evidence and closure limit |
|---|---|---|
| TailBench | TailBench++ author preprint May 2025; published journal version May 20 2026 | Both versions identified. Preprint PDF and journal HTML retained; journal PDF direct retrieval returned HTTP 400. Lifecycle changes inspected; all performance results and source-release equivalence remain open. |
| Metastability model | MSF-Model, SRDS 2024, arXiv v2 Nov 22 2024 | Retained. Abstract/publication metadata checked; full queueing model unread. |
| Metastability characterization | Characterizing Metastable Faults and Failures, arXiv v2 June 2 2026 | Retained. Intro/§2 explicitly discuss older overload characterization and additional oscillation case; not a verified replacement theory. Author record says submitted to SOSP; no acceptance inferred. |
| Coordinated omission in stream generators | [UK Systems Research Challenges Workshop 2024 paper 31](https://uksystems.org/workshop/2024/papers/srcw24-paper31.pdf) | Located primary workshop abstract in final search: explicitly discusses backpressure causing benchmark generators to halt and hide missing ingestion. Not acquired/read in this bounded wave; exact metadata and associated full work remain open. |
| Coordinated omission in database benchmarks | [Coordinated Omission in NoSQL Database Benchmarking, BTW 2017](https://www.btw2017.informatik.uni-stuttgart.de/slidesandpapers/E4-11-107/paper_web.pdf) | Located primary conference original; useful older bridge challenging a YCSB correction. Not acquired/read in this wave. |
| Current correction machinery | Pinned wrk2 and HdrHistogram source, acquired 2026 | Direct code checks cover intended-time corrections and their population limits; source age/current HEAD is not evidence of new peer-reviewed validation. |

No Scholar/Semantic Scholar/OpenAlex forward-citation graph was exhaustively traversed. These are explicit targeted checks and validated edges, not a bibliometric closure claim. Recent citing uses of a root and substantive improvements to it are different categories.

## Access and provenance limits

- Tene's author/company-uploaded Slideshare HTML retains visible text through slide 100 of 254, plus platform material. Ignore the platform's AI-generated description as evidence. The complete original PDF was not recovered from old Azul URLs; the 2013 InfoQ video was located but not downloaded/watched. wrk2 author documentation and source supply the correction evidence used in the study.
- Little's publisher page denied direct access. The original 1961 paper was retained from an MIT-hosted faculty/course mirror, not described as an author upload. The original Mytkowicz et al. PDF was retained from a Waterloo course mirror; its author-group publication page is separately retained. These are primary papers on mirrors, not secondary accounts; neither mirror establishes the publisher's version of record byte identity.
- Network Calculus is the author-hosted online August 23, 2022 revision of the 2001 book. Only the named chapter 1 definitions/statements were studied; 265 pages acquired does not mean 265 pages read.
- GitHub source archives retain original embedded third-party code and licenses. The TailBench++ archive also contains nested Mercurial history and generated dependency metadata: originals are preserved, extracted working copies omit those directories. No source history was rewritten.

## Unsearched or incompletely studied fronts

- Confidence bounds for extreme quantiles under time dependence, censoring, nonstationarity and rare common-mode failures; sequential stopping risks and reliability demonstration tests.
- Detailed modern load generators (Hyperfoil, Gatling, Vegeta, Nighthawk, Locust, YCSB/Nexmark) and correction debates, rather than trusting an `open-loop` label.
- Batch service queues, flush-on-idle and final-drain analysis with variable-sized/reconstructively compressed events; offered bytes versus reconstructed event counts.
- Device endurance/thermal steady state, sustained write behavior past cache limits, filesystem/device flush semantics, replica recovery traffic, and hardware fault injection. See the separate durable and fast-capture lanes for some roots; this wave does not close them.
- Current clock/timer evidence on deployment hardware, cross-core migration, NUMA and device locality, power management, PMU/profiling observer effects and cross-OS equivalence.
- Replayable arrival traces and burst/correlation models for actual instrumented projects; production calibration without letting backpressure erase required source events.

The report identifies experimental questions. It does not establish feasibility for arbitrary offered load, choose a drop/backpressure policy or relax all-arrival capture to a success-only percentile.
