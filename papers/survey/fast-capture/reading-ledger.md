# Reading and execution ledger

Read on 2026-10-23. The `readingStatus` column in
[bibliography.tsv](bibliography.tsv) is the per-source record. The originals and
derived text are named by the `id` column under
`quarantine/{archives,sources}/survey-fast-capture/literature/`.

| Reading group | Items | Extent |
| --- | --- | --- |
| Core sections | NanoLog; Log20; Log²; Linux ring design; BPF ring design; Perfetto buffers; Perfetto V2; JEP 328; Dapper; X-Trace; Pivot; Hindsight; rr extended report; io_uring manual; fsync manual | Fifteen items. Design/contract sections and selected evaluation methods; not a claim of cover-to-cover reading. Exact sections are in the TSV. |
| Selected sections | LTTng thesis; LTTng 2.14 manual; Perfetto service; Eidetic Systems; CMU tracing-design report; Disruptor | Six items. The thesis/manual are large; only named relevant sections were read. |
| Front matter/abstract only | R2; Lamport clocks paper; 2025 DBMS io_uring manuscript | Three items; no core evaluation conclusions used. |
| Acquired, not studied | Linux relay; perf ring documentation; perf_event_open; CTF2; Magpie; Canopy; Stardust; first rr preprint; Scribe; ReVirt; W3C Trace Context; OpenTelemetry SDK specification; Michael–Scott original unavailable; DoublePlay unavailable; Axboe io_uring article unavailable | Twelve acquired unread works plus three unavailable originals. Search abstracts or citation metadata are discovery aids, not completed reading. |

The source-code reading focused on the actual writer, buffer handoff, overload,
and failure paths named in [study.md](study.md). Full repositories were acquired
to keep these paths interpretable, not because all 16,940 files were read.
Relevant tests were located but not executed. No mutation testing, benchmarks,
builds, runtime failure reproduction, or model checking took place in this lane.

The acquisition check is deliberately narrower: every original's recorded hash
was checked, every reference extraction was compared with its retained archive,
and the per-file inventories were checked. That establishes which source bytes
were read. It does not establish correctness, speed, completeness of
instrumentation, storage durability, or capture reliability.

Historical drift observed during reading:

- The 2017 rr paper's hardware limitations differ from the acquired current
  README, which names some AArch64 support.
- Perfetto's acquired V2 document describes a 2025 buffer rewrite; the pinned
  code includes both V1 and V2 implementations.
- The Disruptor page retains a May 2011 heading but now includes later hardware
  comparison tables; its captured bytes, not the heading alone, identify it.
- The new DBMS io_uring manuscript's PDF title and authors were read directly;
  a search-result title and placeholder final DOI were not treated as final
  bibliographic metadata.
