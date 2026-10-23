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
