# Deleuze

A standalone Rust audit log crate with one job: capture every event that reaches
its gate, extremely quickly. Reliability is a deployment-funded target measured
in nines. Throughput, tail latency, retention reliability, hardware and cost must
be assessed together. Events presented to a full or failing gate remain part of
that target; counting only successful queue admissions would change the claim.

Deleuze will support native Rust integration and polyglot interfaces. World-state
acquisition and evaluation fidelity are uses of the retained stream. Its capture
responsibility begins at the gate.

HTTP and gRPC gates, native Rust integration and custom adapters will share one
capture path. Events have a very small, strict record schema and associated
arbitrary properties outside that schema. Opaque data or references can carry
otherwise unrepresentable or large content. See [gates and events](design/gates-and-events.md).
Producers need not construct events: Deleuze packs content at the gate, assigning
write sequence, property hash and its own record timestamps. Sequence orders log
writes, not actions. [Record profiles](design/record-profile.md) describe core
fields and configurable features with proposed defaults.
Persist event contents only; there is no additional request archive.
The public log is append-only: append, read and query, with no update or delete
operations. Later assertions never rewrite earlier records.

The first native core implements the reviewed
[small-core design](design/native-core.md). It uses one append-only binary file
and the official BLAKE3 crate. Power-loss resilience is outside this build.
Optional HTTP and raw-content unary gRPC gates share that core under the `gates`
feature. See [running the gates and polyglot clients](docs/gates.md). Optional
record profiles remain subsequent work.

```rust
use deleuze::{Log, Options};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let log = Log::open("observations.dlz", Options::default())?;
    let record = log.append("ordinary producer output".to_owned())?;
    let entry = log.read(record.seq())?.expect("published record");
    println!("{entry:?}");
    Ok(())
}
```

The default returns after a complete file write. Set `sync_each_append: true`
in `Options` to also call `File::sync_all` before success. An append error returns
the supplied content and says whether a write was attempted with an unknown
outcome. A write/sync error stops later appends in that instance. Clean files
can be reopened; damaged files are rejected without changing them. Reads and
queries currently scan linearly. This is a starting implementation, with no
throughput or deployment-reliability claim yet.

See [the adapter guide](docs/adapters.md) for custom integrations and
[the cryptographic boundary](design/cryptographic-boundary.md) for what the
property hash establishes.

Start with [the public research map](papers/survey/README.md). Public reference
originals live in ignored `quarantine/`; tracked manifests describe restoration.
Private source material and identifying notes stay outside this repository.
Only generic lessons and public-source citations belong here. Research reports
live in `papers/`, session records in `journal/`, and issue tracking in `bd`.

[The design starting point](design/starting-point.md) turns the acquired material
into concrete comparisons for discussion; it is not an approved specification.
[The capture-contract candidate](design/capture-contract.md) develops the small
record/property distinction and ownership rules, with [review findings](papers/reviews/capture-contract.md).

Read [blackbird.md](blackbird.md) for the working doctrine. This is a relative
symlink to `../blackbird.md` in the operator's workspace.

Crates.io returned HTTP 404 for `deleuze` on 2026-10-23 UTC. This is an availability
observation, not a reservation. No crate has been published yet.
