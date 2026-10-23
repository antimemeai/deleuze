# Performance measurement review

2026-10-23. Independent design/plan review accepted separate closed-loop native
and fixed-rate network workloads, with explicit failure populations. It required
a server window ending after drain and partial-prefix reconciliation.

Source review corrected raw export delaying the network denominator, native
aggregation overlapping producers, and exact-count reads hiding an unexpected
suffix. The suffix regression failed before its correction. Nearest-rank samples,
absolute due times, not-sent accounting and direct ID/receipt reconciliation were
reviewed. A real HTTP/2 reset completed as one unknown with zero pending work.

Two label corrections identify the native pre-barrier origin and partial fixed
records in the file floor. No remaining blocker to the first clean-success
matrix was found. The file floor has no log semantics; same-host clients and
uncontrolled cache/scheduler state constrain interpretation. No reliability
nines or isolated protocol overhead follows from these measurements.
