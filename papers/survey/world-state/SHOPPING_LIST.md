# Open acquisition and citation fronts

This lane is deliberately broad and unfinished as a field survey. The catalogue distinguishes full originals, web references, central reading, partial reading and material not yet read. No missing item is treated as a proved result.

## Originals not acquired

| Work | Primary pointer | Result in this run |
|---|---|---|
| Fidge 1988, Timestamps in Message-Passing Systems That Preserve the Partial Ordering | https://zoo.cs.yale.edu/classes/cs426/2012/lab/bib/fidge88timestamps.pdf | HTTP 403; alternate Lund/Yale paths returned 404. Mattern original and Lamport original are retained, but do not stand in for reading Fidge |
| Abadi et al. 2013, Formal Analysis of a Distributed Algorithm for Tracking Progress | https://www.microsoft.com/en-us/research/wp-content/uploads/2013/06/clock-verif2.pdf | Browser extraction available; direct original downloads repeatedly returned 502 or timed out |
| Rodeheffer 2013, The Naiad Clock Protocol: Specification, Model Checking, and Correctness Proof | https://www.microsoft.com/en-us/research/wp-content/uploads/2013/02/paper.pdf | Browser reports 249-page original; direct original download failed. Not read |

No TLS validation was disabled. Several dead primary URLs were repaired using author, publisher or university mirrors; actual successful sources are recorded in bibliography.tsv. The Naiad SOSP original was recovered from a Wisconsin course mirror after Microsoft/other-mirror failures. The acquired rollback survey is a **1999 report**; obtain the **2002 journal version** before making version-specific claims. Delta CRDT and CALM entries identify the acquired preprint separately from later publication metadata.

## Missing implementation material

- CamFlow `libprovenance` requires the `C-Thread-Pool` submodule; the kernel capture implementation, kernel headers and relay-capacity/drop behavior are not in this userspace snapshot. Primary project pointer: https://camflow.org/ . Pin a corresponding kernel revision before drawing whole-system completeness conclusions.
- Flink’s docs theme, Beam’s docs/action submodules, XTDB’s docs/dataset submodules, and GProM’s antutils submodule are omitted by upstream source archives. They are listed in source-reading.tsv. No build completeness is claimed.
- The Isabelle progress-tracking artifact is linked by the ITP 2021 paper: https://www.isa-afp.org/entries/Progress_Tracking.html . Acquire a pinned AFP/Isabelle release and its original artifact before treating a proof as executable local evidence. The paper is acquired and partially read; the artifact was not acquired or run.

## Next citation fronts

These are directions exposed by actual references or stated limitations, not a selected design:



## Recent forward-check follow-up

Seven concrete 2024–2026 fronts are now in bibliography.tsv and forward-checks.tsv, with six new original PDFs. In particular:

- Study the September 2026 *Fixing the Fixpoint* Lean artifact and compare its declared well-formed/regular-circuit assumptions with a pinned runtime before transferring its proof claim. The central counterexample and internal-convergence definitions were read; the artifact was not acquired or run.
- The 2025 *Keyed Watermarks* full text remains blocked by publisher access. DOI 10.1016/j.future.2025.107796; authors confirmed against the publisher’s Crossref deposit: Tawfik Yasser, Tamer Arafa, Mohamed ElHelw, Ahmed Awad. Only the publisher abstract/reference listing was read. The paper’s “data loss” concerns analytical processing; no raw-ingest loss estimate is inferred.
- Eg-walker’s author artifact (https://github.com/josephg/egwalker-paper) supplies a TypeScript reference and optimized Rust implementation pointers. Neither was acquired in this lane; the eight pinned reference snapshots remain the list in references.tsv.
- KAIROS, ProHunter and ProvICS need a deeper read of preprocessing, graph reduction, benchmark ground truth, and retained raw observations. Their recent primary originals were acquired; no detection result is treated as a delivery or reconstruction guarantee.
- These searches do not establish that a relevant 2024–2026 temporal/provenance/causal-time work is absent. Forward-citation collection is incomplete, especially temporal databases and dynamic-membership clocks.
