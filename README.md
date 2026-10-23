# Deleuze

A standalone Rust audit log crate with one job: capture every event that reaches
its gate, extremely quickly. Reliability is a deployment-funded target measured
in nines. Throughput, tail latency, retention reliability, hardware and cost must
be assessed together. Events presented to a full or failing gate remain part of
that target; counting only successful queue admissions would change the claim.

Deleuze will support native Rust integration and polyglot interfaces. World-state
acquisition and evaluation fidelity are uses of the retained stream. Its capture
responsibility begins at the gate.

The project is in reconnaissance. Existing implementations are being acquired and
read before we choose a design, storage format, interfaces, or dependencies.

Start with [the public research map](papers/survey/README.md). Public reference
originals live in ignored `quarantine/`; tracked manifests describe restoration.
Private source material and identifying notes stay outside this repository.
Only generic lessons and public-source citations belong here. Research reports
live in `papers/`, session records in `journal/`, and issue tracking in `bd`.

[The design starting point](design/starting-point.md) turns the acquired material
into concrete comparisons for discussion; it is not an approved specification.

Read [blackbird.md](blackbird.md) for the working doctrine. This is a relative
symlink to `../blackbird.md` in the operator's workspace.

Crates.io returned HTTP 404 for `deleuze` on 2026-10-23 UTC. This is an availability
observation, not a reservation. No crate has been published in this research phase.
