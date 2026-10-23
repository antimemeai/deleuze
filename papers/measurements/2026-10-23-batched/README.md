# Batched synchronization results

2026-10-23. Synchronized native writes now share a barrier. With 1,024 ready
128-byte events per explicit call, the median was **189,975 records/s**, versus
**205/s** for the current single-record reference: about **929×** higher
throughput. The batch's preparation-to-return p99 was **8.87ms**, not 8.87ms divided
by 1,024. Every member waits for the covering sync before successful completion.

With 256 records per call, synchronized throughput was 55,994/s. Independent
blocking callers also group automatically; 64 producers reached 10,409/s without
constructing bulk input. HTTP and gRPC use that same native completion path.

## What changed

`Completion::Written` keeps single appends on the caller thread.
`Completion::Synced` uses one bounded writer, collecting by record/byte targets
and oldest waiting deadline. `append_batch` accepts ordinary owned contents
and returns per-item results. Records keep their own seq, property hash and
Deleuze write time; no batch event, new format, hash protocol or request archive.
No dependency or power-loss subsystem was added.

Default targets are 256 records, 1MiB encoded frame bytes and 1ms intentional
waiting, with 1,024 pending submissions and 128MiB encoded work including active
batches. Larger explicit groups below raise max_records accordingly. Zero delay
still drains already-ready work. A delayed/failed reply after handoff does not
establish absence; rejected handoffs and explicitly untouched work are NotWritten.

## Setup and measurement meaning

Same M4 Max / 128GiB / macOS 26.6 / internal APFS SSD host as the
[first baseline](../2026-10-23/README.md), Rust 1.94.1 and Node 24.7.0. Three fresh
processes per condition, release builds, shuffled order seed 108, no warmup,
affinity or background-load/cache isolation. Results vary substantially.
Counts in tables sum three runs; rates/p99s are medians of per-run values,
not pooled percentiles or confidence intervals.

The 75 native runs used source `d1368e4`. Native explicit input is ready bulk data;
it does not include waiting to accumulate a live producer stream. Each member's
latency spans preparing the whole call through its return. Synchronized sizes 16/64/256/1024 use 256 bulk calls per run; their one-record
reference uses 1,000 calls. Written single-producer trials use 65,536 calls at
size 1, 4,096 at 16, 1,024 at 64 and 256 at 256/1024. Call counts
are retained because repeated per-member timestamps are correlated, not thousands
of independent storage completions.

The 30 published network runs use corrected client source `ac6f5a3`. Server
throughput includes Node startup through server drain, excluding waiting for
raw export. Client p99 is actual request-start to completion; due-time and
generator-lag distributions remain in the raw summary metrics. HTTP uses
keep-alive pooling; gRPC retires sessions gracefully and creates replacements
only for future offers. No offered ID is retried or replayed.

## Native explicit synchronized groups

One producer, 128-byte payloads, zero collection wait, matching explicit producer
groups in the std-file floor. The file floor has no record/hash semantics and
does not coalesce concurrent producers. Both complete sync before return.

| Records/call | Deleuze records/s | File-floor records/s | Deleuze p99 ms/call | File-floor p99 ms/call |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 205 | 233 | 9.979 | 8.236 |
| 16 | 3,591 | 3,552 | 7.147 | 8.116 |
| 64 | 14,135 | 14,055 | 8.414 | 8.362 |
| 256 | 55,994 | 57,786 | 8.049 | 6.179 |
| 1,024 | 189,975 | 213,028 | 8.873 | 8.199 |

For 1,024 records, native rates ranged 138,147–200,103/s and per-run p99s
8.75–89.83ms (see exact rows). The current one-record reference is deliberately
configured max_records=1/delay=0; it is not the default batching policy. The
historical ~238/s baseline used the earlier implementation and a different
time window. Do not treat the ratio between periods as an isolated code effect.

## Automatic native grouping

Each producer makes blocking single appends; the collector combines independent
calls. Default 256-record / 1MiB / 1ms targets, 4,096 events per run.

| Producers | Median synchronized records/s | Median retained p99 ms |
| ---: | ---: | ---: |
| 4 | 682 | 7.875 |
| 32 | 5,246 | 8.061 |
| 64 | 10,409 | 10.054 |

A sequential producer uses append_batch to supply ready groups. A single
blocking append has no future call available to combine until it returns.
Batching amortizes storage work; it does not make one synchronization complete
in nanoseconds or remove waiting for event production.

## Written path

| Records/call | Deleuze records/s | File-floor records/s | Deleuze member p99 ms |
| ---: | ---: | ---: | ---: |
| 1 | 414,747 | 552,085 | 0.005 |
| 16 | 1,089,026 | 1,988,503 | 0.013 |
| 64 | 1,483,575 | 3,685,283 | 0.045 |
| 256 | 1,043,239 | 3,875,090 | 0.335 |
| 1,024 | 1,410,292 | 3,114,127 | 7.736 |

Single-call written performance was lower than in the first baseline for both
Deleuze and the unchanged file floor (415k versus 552k/s in this period; formerly
594k versus 729k/s). These unisolated periods do not establish a code-regression
effect. Written single appends still have no worker hop. Bulk throughput/tails
are not monotonic with batch size in these short trials; no optimum is claimed.

## Real network with synchronized completion

Payload 128B, four Tokio workers, 256-record / 1MiB / 1ms sync targets. Offered
schedules are two seconds each. Gate 64 uses client cap 128; gate 256 uses cap 512.
Retained counts here are server outcomes; client uncertainty is separate.

| Protocol | Offered/s | Gate cap | Offers | Arrived | Server retained | Server failed | Not sent | Client unknown | Median retained/s | Retained reply p99 ms |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| HTTP | 1,000 | 64 | 6,000 | 6,000 | 6,000 | 0 | 0 | 0 | 981 | 10.508 |
| GRPC | 1,000 | 64 | 6,000 | 6,000 | 6,000 | 0 | 0 | 0 | 981 | 11.805 |
| HTTP | 10,000 | 64 | 60,000 | 60,000 | 37,248 | 22,752 | 0 | 0 | 5,332 | 146.900 |
| GRPC | 10,000 | 64 | 60,000 | 59,999 | 46,885 | 13,114 | 0 | 769 | 7,680 | 11.166 |
| HTTP | 40,000 | 256 | 240,000 | 234,806 | 181,747 | 53,059 | 5,194 | 0 | 30,014 | 17.752 |
| GRPC | 40,000 | 256 | 240,000 | 134,368 | 134,368 | 0 | 105,632 | 0 | 21,692 | 28.706 |

All 12,000 synchronized offers at 1k/s were retained, versus substantial
per-record-sync rejection in the first baseline. Higher load still exceeds
configured limits; batching alone does not establish lossless capture under
those conditions. Client/gate/thread/transport limits matter alongside storage.

Overloaded gRPC at 10k/s produced 12 observed GOAWAY (code 11) events and resource-
exhausted statuses. It had 769 client-unknown results: 768 corresponding complete
records were found in the synchronized native stream, and one issued request
had no service notification observed. The server storage outcome was known;
the client lacked replies. Never erase those records or retry their IDs silently.
A session error does not imply that its owned records were absent.

The higher-capacity 40k/s settings deliberately change two concurrency bounds.
They are deployment-specific observations, not a controlled protocol comparison
or maximum server capacity. All not-sent work and server failures remain visible.

## Client correction and retained evidence

The initial 105-run matrix included 30 single-session network probes; a 12-run
follow-up classified rejection statuses. A closed session was repeatedly reused
for future offers, and pre-stream constructor failures were conservatively
misclassified as issued/unknown. Those 42 diagnostic rows are preserved in
[single-session-probe.csv](single-session-probe.csv), not presented as sustained
network throughput. Their native records/receipts still reconciled.

The corrected client creates replacements for future offers only. Existing
streams finish on retired sessions; missing post-handoff replies remain unknown.
Proven pre-stream failures are not sent, with reason counters. Four direct
fixtures cover graceful/nonzero GOAWAY, pre-stream failure and post-handoff
reset; the two original regressions failed before the fix.

Core checks passed 55 runtime tests and 3 compile-fail doctests, plus direct
benchmark helpers and live Python/Node HTTP/gRPC checks in both completions.
Fake I/O asserts one sync covers multiple frames, no early acknowledgment,
short/interrupted writes, failure publication, permits, deadlines and shutdown.
Independent source/test review found no blocker. All retained native/workload
identities and receipts reconciled; network uncertainty/rejection above remains.

## Artifacts and reproduction

- [75 native runs](native.csv), [environment](initial-environment.tsv), [commands](initial-commands.tsv).
- [30 corrected network runs](network-results.csv), [environment](network-environment.tsv), [commands](network-commands.tsv), [all summary metrics](network-metrics.tsv).
- [Initial/probe metrics](initial-metrics.tsv) and [single-session environment](single-session-capacity-environment.tsv) retain the earlier diagnostic context. `NA` is an empty metric, not zero latency.

Full synthetic logs/samples/resources remain in ignored target/measurements
under 2026-10-23-batched, 2026-10-23-batch-capacity and 2026-10-23-batch-network.
Byte-content framing overhead remains 108 bytes/record plus 12 bytes/file.
Whole-process resource fields include verification/export, not capture-only cost.
The volume was near capacity and no deployment pricing/nines is inferred.

```sh
python3 scripts/run-measurements.py --out-dir target/measurements/new-batched --repetitions 3 --suite batching
python3 scripts/run-measurements.py --out-dir target/measurements/new-network --repetitions 3 --suite batch-network
```

Current network runs use session rollover. Historical probes reproduce their
old connection model only at their recorded source revisions. See
[the API guide](../../../docs/batching.md), [measurement method](../../../docs/measurement.md)
and [design/review](../../reviews/batched-sync.md).
