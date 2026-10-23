# First performance baseline

2026-10-23. Measure the implemented path before choosing optimizations. Use the
approved std/BLAKE3/optional gate stack only, Node's built-in HTTP/HTTP2 clients
and simple external TSV measurement artifacts. No new production dependency,
persistence subsystem, power-loss testing or deployment-nines claim.

## Workloads and boundaries

Native: release-mode closed-loop saturation, a fixed request count divided over
1 or several producer threads. Measure complete public Log::append calls, with
input allocation/ID stamping inside the operation timer. Compare a private std
file baseline using the same mutex/write_all/optional sync_all boundary and the
same total bytes per record (payload+108 for native Bytes packing/frame overhead).
The baseline uses preformatted bytes with no hash/record semantics: an I/O floor,
not an equivalent log. Keep written and per-append-synced results separate.
Start producer threads together; throughput uses start through final completion,
not sums of worker rates. Label these closed-loop service latencies honestly.

Network: a Rust example starts a real local gate/listener and one Node load
process using HTTP/1 keep-alive or one HTTP/2 gRPC session. Node schedules a fixed
count on absolute monotonic due times at a configured rate, independently of
previous completions. A configurable outstanding cap reports not-sent work rather
than waiting to schedule it. No hidden retry. Report generator lag, actual-start
latency and due-to-completion latency separately; report outcome-specific and
all-completed tails so quick rejections cannot hide retained-event tails.
After the schedule stops, await every issued request (bounded client timeout),
then close/drain the gate and read counters. Server-retained throughput uses
the complete Rust wall window from client-process launch through gate drain;
this conservatively includes client startup/artifact-output overhead. Label
client and offered windows separately. Reject zero/overflow inputs.

Default payloads contain an eight-byte little-endian workload ID followed by
0xa5 bytes, minimum size8. IDs are content only and never Deleuze sequence. Gate
limits and client limits are separate. Expose payload size, count, producer count
or offered rate/outstanding cap, sync choice and gate in-flight bound as CLI
options. Record the runtime worker count and connection model.

## Direct reconciliation and reporting

Native samples record workload ID, actual-start/completion offsets, outcome and
returned sequence. Network samples additionally record due offset and receipt.
Write raw samples and compact summaries as TSV for operator analysis; these are
benchmark artifacts, not production persistence or an internal event model.
Nearest-rank p50/p95/p99/max use individual samples, with counts alongside them.
No block mean labelled a percentile or confidence interval inferred from one run.

After timing, query native records once with configured result bounds sufficient
for this workload. Verify every retained payload's ID, exact size/pattern and
uniqueness; verify each retained receipt against its actual sequence/hash. Use
workloads that fit both gate wire limits (including five gRPC prefix bytes) and
native encoding limits. Still recognize/count explicitly incomplete records
separately after transport errors: validate available ID/pattern prefix, never
call a partial record a completely retained event. For
the file floor, verify total length and fixed records/IDs. Reconciliation time is
outside capture timing and reported separately. Exit nonzero on contradictions.

Always expose offered, issued, not-sent, completed, pending and client outcomes,
plus server arrived/active/outcomes/retained-record counts. Transport errors are
client-unknown; they never prove absence. Report server-only retained results
separately when a reply was lost. Full/failing server arrivals remain failures,
not exclusions. Assert population sums and post-drain active/pending zero.
Do not require issued==server-arrived when transport failed before notification;
show the difference and reconcile retained IDs. HTTP parser pre-service coverage
remains a documented limitation.

Report elapsed workload/drain/reconciliation times, offered and retained rates,
logical bytes and file bytes, producer/runtime/connection settings and sample
counts. Capture hardware/OS/Rust/Node/build/source revision, filesystem facts and
storage-boundary choice, without host names or private-source information.
Same-host clients share CPU with the server; no cloud cost or hardware purchase
price is invented. Keep raw samples/logs in ignored target/measurements, summaries
and reproducible commands in papers/measurements.

## Implementation and validation

After independent design/plan review: implement native/file measurement with
small shared sample/quantile/output helpers, then fixed-rate Node load and a
Rust gate runner using those artifacts, then run the modest matrix. Direct
fixtures check quantiles/accounting and retained identity; an injected client
cap demonstrates not-sent accounting, and low gate capacity demonstrates
server failures. Keep checks proportionate and use the existing conformance
suite for production behavior. Review measurement code before publishing claims.
Commit/push reviewed plan, compiling utilities, fixes and results incrementally.
