# First native and network performance baseline

2026-10-23. The native written path reached **594k retained records/s** with
128-byte payloads and one producer (median of three fresh processes); median
per-run retained p99 was **3.75µs**. Four producers reduced throughput to
**276k/s** and raised that p99 to **96µs**. Per-append file synchronization
changed the regime to roughly **235–238 records/s** on this machine.

Network capacity limits matter to fidelity: raising the gate limit from 64 to 256
eliminated server rejections in the controlled comparison, but increased client
not-sent work and retained-request tails. Useful retained throughput barely
changed. No production default was changed.

## Setup and interpretation

Apple M4 Max, 16 physical/logical CPUs, 128GiB RAM, macOS 26.6, internal SSD/APFS.
The APFS container had about 25GB free before the first matrix; the volume was
near capacity. Rust 1.94.1, aarch64-apple-darwin, default Cargo release profile;
Node 24.7.0. No warmup, CPU pinning, cache flush or exclusive-host control. Payloads
are an eight-byte workload ID followed by 0xa5 bytes, not production traffic.
Three processes per condition, randomized order seed 108.

The first matrix has 72 runs from source `8f2dba1`; the controlled capacity matrix
has 24 from `28cdf5c`, with the same Rust/Node load implementations. Exact source
revisions, lock hashes, build commands and environment facts are retained below.
Counts in tables sum three runs. Rates and p99s are **medians of three per-run
values**, not pooled quantiles or confidence intervals. Ranges are observed
minimum–maximum. Individual measurements remain in the CSV/TSV files.

Native workloads are closed-loop saturation. Each operation times producer
allocation/ID stamping through append completion; wall throughput also includes
inter-call sampling. The frame-sized std-file floor has the same byte count and
write/sync boundary, but no hashing, metadata or log semantics.

Network workloads schedule absolute due times independently of replies, with
128 client requests outstanding at most, 64 gate requests in flight and four
Tokio workers unless otherwise specified. HTTP uses keep-alive pooling; gRPC
uses one HTTP/2 session. Client and server share the host. Server throughput
includes client startup through server drain, without waiting for raw-sample
export. Do not read native/network rate ratios as isolated protocol overhead.

## Native written boundary

Each 128-byte trial has 50,000 operations; each 4096-byte trial has 10,000. All
native and file-floor operations in these conditions were completely verified.

| Path | Payload B | Producers | Retained k/s, median (range) | Retained p99 µs, median (range) |
| --- | ---: | ---: | ---: | ---: |
| Deleuze | 128 | 1 | 593.5 (585.9–606.3) | 3.75 (3.75–4.12) |
| File floor | 128 | 1 | 729.5 (710.3–767.7) | 3.33 (3.29–3.33) |
| Deleuze | 128 | 4 | 275.6 (268.1–282.0) | 96.42 (83.54–107.12) |
| File floor | 128 | 4 | 338.8 (282.9–351.4) | 155.29 (153.00–189.08) |
| Deleuze | 4096 | 1 | 123.7 (116.9–125.3) | 7.08 (6.33–8.12) |
| File floor | 4096 | 1 | 196.4 (137.0–200.3) | 4.08 (4.08–8.88) |
| Deleuze | 4096 | 4 | 94.7 (74.7–98.9) | 132.38 (90.29–409.33) |
| File floor | 4096 | 4 | 126.1 (123.2–153.6) | 63.88 (9.62–252.92) |

The 128-byte single-producer log reached about 81% of the file floor's median
rate; the 4096-byte case reached about 63%. Those gaps include different work,
including encoding/hash/clock costs; they do not identify a specific bottleneck.
Multiple producers reduced both log and file-floor throughput. Profiling and a
controlled writer/batching comparison are justified before changing the path.

## Per-append sync boundary

These 128-byte trials each have 1,000 operations. "Synced" means the ordinary
`File::sync_all` call returned before success. All completed and were verified.

| Path | Producers | Median retained/s | Median per-run retained p99 ms |
| --- | ---: | ---: | ---: |
| Deleuze | 1 | 238.2 | 6.64 |
| File floor | 1 | 238.3 | 7.13 |
| Deleuze | 4 | 235.4 | 394.04 |
| File floor | 4 | 231.3 | 8.62 |

The four-producer synchronized log had large call tails (per-run p99s 385–458ms).
These measurements establish the symptom, not its scheduler/lock/I/O cause.
Adding producers did not increase synchronized throughput. This is a separate
completion boundary from the default written mode, not a reason to impose a
power-loss subsystem on the program.

## Real network, default gate capacity

All rows use 128-byte payloads and three two-second offered schedules except
synced rows, which use three one-second schedules. Retained p99 reply time starts
at actual request creation; due-time p99 also includes generator lateness.
Known gate rejections remain failures. Not-sent work never reached this gate and
stays visible in the offered population. No requests were retried.

| Protocol | Boundary | Offered/s | Offers | Retained | Gate failed | Not sent | Median retained/s | Reply p99 ms | Due p99 ms |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| HTTP | written | 1,000 | 6,000 | 6,000 | 0 | 0 | 985 | 0.405 | 0.415 |
| GRPC | written | 1,000 | 6,000 | 6,000 | 0 | 0 | 985 | 0.396 | 0.443 |
| HTTP | written | 10,000 | 60,000 | 59,979 | 21 | 0 | 9,839 | 1.821 | 2.382 |
| GRPC | written | 10,000 | 60,000 | 59,987 | 13 | 0 | 9,841 | 1.243 | 2.311 |
| HTTP | written | 40,000 | 240,000 | 235,235 | 1,982 | 2,783 | 38,594 | 3.384 | 4.525 |
| GRPC | written | 40,000 | 240,000 | 229,674 | 4,465 | 5,861 | 37,641 | 3.900 | 5.170 |
| HTTP | synced | 1,000 | 3,000 | 907 | 2,093 | 0 | 233 | 268.471 | 268.476 |
| GRPC | synced | 1,000 | 3,000 | 872 | 2,128 | 0 | 233 | 271.369 | 271.374 |

All 12,000 default-capacity written offers at 1k/s were retained. The initial
10k/s cohort had 21 HTTP and 13 gRPC server rejections across 120,000 offers; the
follow-up's 10k/s cohorts had none. This variation and the generator-lag metrics
are reasons not to turn these short runs into a capacity guarantee or nines.
At 40k/s both client and gate limits contributed; same-host generator limits
prevent treating these runs as a maximum server-throughput measurement.

## Controlled gate-capacity comparison

Same 40k/s offer schedule, 128 client outstanding cap, 128-byte payload, written
boundary and four Tokio workers. Each row sums 240,000 offers over three runs.
The separate 10k/s comparison retained all 60,000 offers in every protocol/capacity
combination.

| Protocol | Gate capacity | Retained | Gate failed | Not sent | Median retained/s | Reply p99 ms | Due p99 ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| HTTP | 64 | 235,740 | 1,717 | 2,543 | 38,655 | 3.289 | 4.589 |
| HTTP | 256 | 235,775 | 0 | 4,225 | 38,633 | 4.329 | 5.386 |
| GRPC | 64 | 229,983 | 3,952 | 6,065 | 37,653 | 3.787 | 5.125 |
| GRPC | 256 | 229,416 | 0 | 10,584 | 37,616 | 4.382 | 5.758 |

Capacity 256 exceeds the client's outstanding cap, so normal requests in these
runs no longer exhausted gate admission. That says something specific about this
setup, not unlimited capture. Client not-sent work increased and observed
retained throughput did not materially improve; retained-request p99s rose.
A larger admission bound alone does not solve end-to-end fidelity or speed.

## Storage and resource cost

Byte content uses 108 bytes of encoding/frame overhead per record plus 12 bytes
per file. Thus 128-byte payloads use 236 bytes/record (1.844× logical content),
while 4096-byte payloads use 4204 (1.026×). At 10k 128-byte records/s, that is about
203.9GB/day of plain file bytes before filesystem, replication or derived-index
costs. This arithmetic is not a measured 24-hour retention run.

Whole-process user/system times and reported max-RSS are retained in the compact
CSVs; their scope includes verification/export and is not capture-only CPU or
steady-state server memory. No hardware/cloud price or monetary estimate is
invented. The near-full local volume, shared CPU and unisolated scheduler constrain
interpretation.

## Reconciliation, artifacts and reproduction

Every run checked offered/issued/not-sent/completed populations, exact retained
IDs and bytes, receipt sequence/hash, duplicates, missing acknowledged successes
and post-drain state. All 96 runs reconciled, with zero unknown outcomes, zero
pending work and no partial-content records. Known rejections and not-sent work
above remain failures to capture offered work; they were not removed from tables.
The HTTP parser's pre-service rejection coverage remains unmeasured here.

- [Baseline runs](baseline.csv), [environment](baseline-environment.tsv), [commands](baseline-commands.tsv).
- [Capacity runs](capacity.csv), [environment](capacity-environment.tsv), [commands](capacity-commands.tsv).
- [All per-run summary metrics](metrics.tsv): `NA` means an empty/unavailable metric, not zero latency. CSV line endings are normalized to LF.

Full samples, synthetic logs and resource output remain in ignored
`target/measurements/2026-10-23-first/` and
`target/measurements/2026-10-23-capacity/`. Re-run into new directories:

```sh
python3 scripts/run-measurements.py --out-dir target/measurements/new-baseline --repetitions 3
python3 scripts/run-measurements.py --out-dir target/measurements/new-capacity --repetitions 3 --suite gate-capacity
```

See [the measurement guide](../../../docs/measurement.md) and
[review findings](../../reviews/performance-baseline.md). This baseline supports
profiling allocation/packing and testing serialized writer batching under the
same populations and completion boundaries. It does not select that design or
establish deployment reliability.
