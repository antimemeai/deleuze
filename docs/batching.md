# Batched writes and sync

`Completion::Written` returns after complete file writes. It is the default and
keeps single appends on the calling thread. `Completion::Synced` sends owned
encoded records to a bounded writer that groups them, writes existing record
frames, calls `File::sync_all` once, then completes callers covered by that sync.
Enqueueing alone never produces a successful acknowledgment.

```rust
use deleuze::{BatchOptions, Completion, Options};
use std::time::Duration;

let options = Options {
    completion: Completion::Synced,
    batch: BatchOptions {
        max_records: 256,
        max_bytes: 1024 * 1024,
        max_delay: Duration::from_millis(1),
        ..BatchOptions::default()
    },
    ..Options::default()
};
```

This replaces the unpublished `sync_each_append` option. Synchronized native,
HTTP and gRPC calls share the same grouping path. Use `append_batch` for bulk
output from a sequential producer; concurrent `append` calls coalesce
automatically in Synced mode.

```rust
use deleuze::{Content, Log};

fn emit(log: &Log) {
    let results = log.append_batch(vec![
        Content::Text("first observation".into()),
        Content::Bytes(vec![0xff, 0x00]),
    ]);
    for result in results {
        match result {
            Ok(record) => println!("write sequence {}", record.seq()),
            Err(error) => eprintln!("{:?}: {}", error.outcome, error),
        }
    }
}
```

The result vector matches input order, with one result per content item. Invalid
items return their owned content through `AppendError`; valid items still proceed.
Bulk input is processed in bounded chunks. Empty input returns an empty vector.
Written bulk calls also write frames together, with no synchronization wait.

A batch is a write optimization, not an event or transaction. Every item retains
its own record, properties, BLAKE3 property hash and write-assigned sequence.
Record timestamps come from Deleuze at batch write; several records can share
that observation. Sequence orders writes, not producer actions. The existing
file format and immutable history remain unchanged.

The defaults are 256 records, 1MiB encoded frame bytes and 1ms collection delay.
Count/byte bounds trigger a write; the original oldest queue age bounds waiting
for additional work. Already-ready work is drained under the size bounds even
when it aged during a previous sync. Delay zero means no intentional wait, while
ready work can still group. These are collection settings, not an I/O deadline.
A valid record larger than the batch byte target travels alone.

Admission defaults to 1,024 pending submissions and 128MiB of encoded frames,
counting both queued and active work. This budget excludes caller-owned originals
and exact object/thread overhead. Full admission returns `QueueFull` and
`NotWritten` with the input; it remains a capture failure. A rejected handoff is
NotWritten. A lost reply after accepted handoff is Unknown. A write/sync error
stops the writer: the attempted group is Unknown, untouched subsequent groups
are NotWritten/Stopped. Nothing retries automatically.

Dropping an observer does not cancel owned work. Log drop closes the worker,
drains owned submissions and joins it before releasing storage. Clean reopen,
strict damaged-file rejection and read snapshots of the successful prefix remain.
No extra request archive, event stream, repair API or power-loss subsystem is
introduced. Producer masks and references remain supplied property values.

For measurement, use `--batch-size` in the native runner and
`--batch-records`, `--batch-bytes`, `--batch-delay-us` in both runners. Individual
latency in an explicit bulk benchmark spans preparing the bulk call through its
return for each member; it is not the batch duration divided by record count.
The file floor has explicit producer batches only, without automatic grouping
across producers. Compare matching single-producer groups for its I/O baseline.
See [measurement](measurement.md) and the [design](../design/batched-sync.md).

Run the bundled service with synchronized completion:

```sh
cargo run --features gates --example gates -- ./events.dlz 127.0.0.1:3000 synced
```
