# Measuring capture

The [first local baseline](../papers/measurements/2026-10-23/README.md) includes
96 release runs, per-run data, environmental details and interpretation.

Use fresh directories; the runners refuse overwrite. No benchmark dependency is
required beyond the optional gate stack and Node's built-in network clients.

```sh
cargo build --release --example measure_native
target/release/examples/measure_native --mode log --out-dir target/measurements/native-new --count 50000 --payload-bytes 128 --producers 4 --sync written
target/release/examples/measure_native --mode file --out-dir target/measurements/floor-new --count 50000 --payload-bytes 128 --producers 4 --sync written

cargo build --release --features gates --example measure_gates
target/release/examples/measure_gates --protocol grpc --out-dir target/measurements/grpc-new --count 20000 --payload-bytes 128 --rate 10000 --outstanding 128 --gate-in-flight 64 --sync written
```

Native mode is closed-loop saturation. Each producer allocates/stamps its
payload and completes append before its next request. Latencies include that
producer work; throughput includes inter-call sampling work. The file floor
writes payload+108 bytes per operation under a mutex, with no hash or record
semantics. Both complete writes. In the current implementation `--sync synced` uses
batched synchronization before completion. `--batch-size` sets native producer
bulk size; batch count/byte/delay settings also control automatic grouping.
These boundaries remain distinct. See [batching](batching.md).

Network mode uses absolute due times, independently of replies. It records
generator lag, start-to-reply and due-to-reply latency, separately for retained,
failed and unknown results. When the client outstanding cap is full, an offer is
not sent and remains in the reported workload. Gate rejections remain server
failures. There are no retries. HTTP uses a keep-alive pool; gRPC replaces retired HTTP/2 sessions for future offers; it never replays an
existing ID. Session/GOAWAY/error counts are reported. The default gate runner uses four Tokio workers.

The server throughput window starts before Node process launch and ends after
gate shutdown/drain. Client startup is included; raw sample export does not
delay the end of that window. Client-cohort timing is also reported. These
network rates and native saturation rates describe different workloads; a
ratio between them is not an isolated measurement of protocol overhead.

Both runners save raw samples and summaries, then check native retained IDs,
exact byte patterns, sequence/hash receipts, duplicates, missing successes and
request populations. Reconciliation is outside capture timing. Explicit partial
content stays separate from completely retained events. The file floor reports
partial fixed-record bytes if an uncertain write leaves them.

Run the initial sequential matrix from a clean committed checkout:

```sh
python3 scripts/run-measurements.py --out-dir target/measurements/baseline-new --repetitions 3
```

Add `--suite gate-capacity` to compare gate limits 64 and 256 at the same 128
client outstanding limit and 10k/40k offered rates. Server rejection can move to
client not-sent work; compare both populations.

This builds release binaries, randomizes a reproducible order, and records the
source revision, lock hash, hardware/OS, storage facts, commands and compact CSV.
It also captures whole-process resource output where available; that CPU/resource
scope includes export and reconciliation. All full samples/logs stay under the
ignored output directory. It does not restart Claude or run mutation tests.

Check the examples with `cargo test --all-features --examples`; run formatting
and Clippy as usual. Quantiles use nearest rank over individual samples and
carry their sample counts. Three fresh processes show run variation, not a
confidence interval or a deployment-reliability estimate. The generated payload
is an eight-byte workload ID plus 0xa5 padding; it is not production traffic.
No CPU affinity, cache flushing or exclusive-host control is implied. HTTP
parser failures before the service callback remain an observation limit.

The `batching` suite compares native explicit sizes 1/16/64/256/1024, concurrent
single appends, matching single-producer file groups, written-path regressions
and real synchronized HTTP/gRPC load. Explicit native groups use zero collection
delay for the matching I/O comparison; automatic grouping uses 1ms.

```sh
python3 scripts/run-measurements.py --out-dir target/measurements/batched-new --repetitions 3 --suite batching
```

The earlier report's per-record-sync numbers remain historical. Reproduce those
original semantics at its recorded source revisions; current Synced behavior
groups calls. The old commands are not a claim that the current default still
uses one synchronization per record.

`--suite batch-network` reruns the network portion with current client session
lifecycle handling. Proven constructor rejection before a stream exists is
not-sent; missing results after stream creation remain unknown. Older single-
session probes are preserved as diagnostics and do not establish steady server
capacity after that session closes.
