# First capture measurements

The operator said send it to the performance phase. The design separates native
closed-loop saturation from fixed-rate real-network load, and written from
per-append sync boundaries. No dependency or production path changed. The
reviewed plan and compiling measurement tools were committed and pushed early.

Independent review required server rates through drain and explicit treatment
of partial content. Source review then corrected main-thread sample aggregation
interfering with timed native workers and network rates waiting for raw export.
One completion marker on the existing child stdout pipe now starts server drain
before waiting for artifact output. Startup remains included in that window.

A direct extra-record fixture failed on the old exact-count query and passed
when a count+1 query detected the suffix. Native receipts/IDs and file patterns
are reconciled after timing. Small fixtures also check duplicates, missing and
wrong receipts, quantiles and overwrite refusal. Incomplete file-floor bytes
and the pre-barrier clock origin are reported explicitly.

Live smokes checked native/file and HTTP/gRPC exact retention. A bounded-client
fixture reported all 1,000 offers (36 issued, 964 not sent), and a constrained-gate
fixture reported all 3,000 arrivals (2,581 retained, 419 failed). These are accounting
checks, not published capacity measurements. Example tests, formatting and
Clippy pass before the main matrix. Full artifacts remain ignored; compact
results and reproducible commands will be retained with the report.

The first 72-run release matrix completed from committed source 8f2dba1. All
post-timing identities/receipts/populations reconciled; no unknown outcomes
occurred. Higher offered rates exposed server rejections and client not-sent
work. A second randomized 24-run suite compares gate limits 64 and 256 with the
same client 128 window before recommending any production change.

All 96 runs completed and reconciled. The controlled gate 256 versus 64 comparison
removed observed server rejections but increased not-sent work at the client
window, with similar retained throughput and higher retained-request p99. The
report separates those populations and makes no production configuration change.

Compact datasets/environments/commands and all per-run summary metrics were
committed and pushed. CSV CRLF lines and empty trailing TSV values initially
triggered whitespace checks; repository copies now use LF and explicit NA for
empty metrics, with numeric values unchanged. The matrix writer now emits LF.
Full raw timings, synthetic logs and process-resource output remain ignored under
target/measurements. Storage arithmetic and whole-process resource scope are
documented without inventing deployment costs or reliability nines.

Independent report review recomputed every displayed group and checked counts,
medians/ranges, units, environment facts, timing interpretation and storage
arithmetic. No report error remained. Native contention/profile and batching
experiments are tracked separately; pre-service parser coverage remains open.
