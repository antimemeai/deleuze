# Access gaps and research fronts

These are limits of this lane, not a claim that the material does not exist.
The primary bibliography has 39 entries and 36 retained originals.

## Originals still to obtain

| Work | Primary URL attempted | Observed result | Next acquisition route |
| --- | --- | --- | --- |
| DoublePlay, ASPLOS 2011 | https://people.cs.vt.edu/dongyoon/papers/ASPLOS-11-DoublePlay.pdf | HTTP 503 on repeated direct acquisition; attempted Michigan author host also had certificate-chain failure | Author/publisher copy, DOI `10.1145/1950365.1950370`. Cached search text is not a preserved original. |
| Michael–Scott queues, PODC 1996 | https://www.cs.rochester.edu/~scott/papers/1996_PODC_queues.pdf | HTTP 404; moved `/u/scott/` author path returned HTTP 403; companion pseudocode path also unavailable | Author's corrected paper or publisher copy, DOI `10.1145/248052.248106`. Check version/corrections rather than copying third-party pseudocode. |
| Efficient IO with io_uring, Jens Axboe 2019 | https://www.kernel.dk/io_uring.pdf | Expired TLS certificate; alternate apex host had hostname mismatch; HTTP returned 404 | Author/maintainer-hosted copy after certificate repair or an identified author-maintained mirror. liburing and its official manuals were acquired independently. |

No TLS verification was disabled to obtain these originals. Access failures are
preserved in the bibliography. If the operator acquires one, retain the original,
hash it, and update access/reading state separately.

## Citation snowballs not yet exhausted

- NanoLog's ETW/WPP lineage; Windows TraceLogging and ETW lost-event behavior;
  OSLog and Apple's unified logging; Fuchsia tracing; DTrace and USDT; QNX and
  embedded interrupt/NMI-safe trace capture.
- FastForward and later SPSC/MPSC/MPMC queues, progress guarantees under producer
  suspension, reusable ring slots, safe reclamation, and C/C++/Rust weak-memory
  models. The current corpus gives concrete queue assumptions, not a complete
  concurrency proof toolkit.
- Hardware-assisted Intel PT/Arm CoreSight/Arm SPE capture, DMA and IOMMU
  boundaries, hardware timestamp synchronization, tracing across reset, and
  uninstrumented data sources. Do not infer whole-machine state from software
  tracepoints.
- Data-race-complete multiprocessor replay, ReSpec, STRIDE/LEAP/CLAP and replay
  of language runtimes; externally shared memory; GPUs/devices; cross-machine
  replay; replay of a failed first occurrence instead of an instrumented rerun.
- Unread acquired roots: Magpie, Stardust, Canopy, Scribe, ReVirt, CTF2 and perf's
  memory ABI. Follow their citations before designing comparable semantics.
- Direct comparisons among current NanoLog, binary_log/Quill/spdlog, Rust tracing,
  tracing-appender, defmt, and binary schema encoders, using equivalent event
  shapes, storage barriers, failure paths, and event-loss accounting. None was
  benchmarked here.
- Recovery of dynamically assigned schemas/dictionaries; schema identifiers
  across builds and polyglot modules; truncated dictionary publication; bounded
  ingestion of untrusted packets; clock-epoch and sequence exhaustion behavior.
- End-to-end cost of checksum, compression, encryption, replication, file
  rotation, directory synchronization, and restart scans. Those costs cannot be
  inferred from a producer-call microbenchmark.
- Contemporary io_uring evaluation and the acquired DBMS manuscript's actual
  artifacts; platform-specific filesystem and power-loss behavior; ARM and
  non-Linux recorder I/O. This lane acquired a Linux source selection, not a
  portable I/O guarantee.

Source references not acquired: closed Google Dapper internals, historical
Microsoft Magpie, Facebook Canopy internals, OpenJDK JFR implementation, and
full kernels beyond the named Linux selection. Their papers/docs do not make
those implementations available locally.
