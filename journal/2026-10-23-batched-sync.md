# Batched synchronization

The operator explicitly required batching after reviewing the per-record sync
cost. The design preserves the direct Written path, adds explicit native bulk
content, and groups synchronized calls with one bounded std writer. The existing
record format and BLAKE3 property hashing remain unchanged. No dependency,
request archive or power-loss subsystem was added.

Independent design/plan review required Unknown for a lost reply after accepted
submission. Rejected handoffs and explicitly untouched work stay NotWritten.
Source review found no ownership/publication/lifecycle blocker.

Two direct regressions failed before fixes: old ready submissions must still
group after their collection age expires, and zero delay must allow ready-only
grouping. The deadline limits waiting when the queue is empty. The native bulk
benchmarks retain per-member preparation-to-return latency rather than dividing
one duration by the record count. The initial implementation checkpoint and
compatible benchmark API migration were committed and pushed. Broader direct
I/O, worker and lifecycle tests precede measurement.

The full core suite passed 55 runtime tests and 3 compile-fail doctests. Direct
seams check shared synchronization, no pre-barrier acknowledgment, short vectored
writes, write/sync failure publication, unknown versus untouched outcomes,
permits, deadlines, observer loss and channel drain. Independent closure review
found no remaining blocker. Live Python/Node HTTP/gRPC checks also passed in
both Written and Synced modes. The service example now accepts an optional
written/synced completion argument.

The new measurement suite uses explicit input groups up to 1024 records, at least
256 bulk calls per explicit condition, automatic concurrent native appends and
real network load. Per-member latency covers the whole bulk API call rather than
an amortized mean. File-floor groups compare the same explicit submission size;
its concurrent callers do not imitate the production sync collector.

The 105-run release suite completed and reconciled. Explicit 1024-record native
groups reached 138k–200k synchronized records/s across the three runs. Automatic
network grouping showed a severe small-gate gRPC limit at 10k/s, so a narrow
64/256 admission-cap comparison will classify actual protocol statuses before
reporting throughput as an end-to-end improvement. The load generator now counts
response status codes alongside existing outcome populations.

Two client lifecycle fixtures failed against the old driver and passed after
correction: future IDs continue on a new session after GOAWAY, with no replay;
a synchronous INVALID_SESSION before stream creation is NotSent rather than
Unknown. The client reports session/GOAWAY/error codes and awaits all session
cleanup. Source/fixture review found no blocker. Historical probe rows remain
unchanged and labelled; corrected network runs follow from committed source.

Final native results: explicit 1,024-record synchronized groups reached a median
189,975.470 records/s versus 204.598/s for the current single-record reference,
with whole-call p99 about 8.87ms. Default automatic grouping and explicit bulk
input are reported separately; no per-record latency is invented by division.

The corrected 30 network runs reconciled all receipts/identities/populations.
All 12,000 Synced offers at 1k/s were retained. Overloaded gRPC produced 12 actual
GOAWAY (code 11) observations; 769 client-unknown outcomes included 768 complete
synchronized records in the native log and one issued request without observed
service notification. No IDs were replayed. Server failures and client not-sent
work stay visible at higher rates. Full raw probes remain versioned separately.

Independent numeric review corrected one sentence that had conflated Written
and Synced bulk-call sample counts, then confirmed all displayed medians/ranges,
counts, units and the 929× within-matrix ratio. Source/protocol data and publication
remained unchanged. Remaining cost profiling is a separate issue; no deployment
nines or maximum network capacity is claimed.
