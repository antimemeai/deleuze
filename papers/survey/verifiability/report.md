# Representation, ownership and claims about fast durable capture

2026-10-23. This lane connects the operator's Verbs direction to primary
literature on ownership, functional verification, concurrency and crash behavior.
Deleuze's priorities remain retaining every gate arrival at deployment-funded
reliability and extreme speed. No tool, verification backend, data model or
recorder architecture is selected here.

Read [Verbs objectives](verbs-objectives.md) first. The
[bibliography](bibliography.tsv) records fifteen acquired primary originals and
bounded reading status. PDF originals are ignored. Section reading means the
specific introductory, mechanism, assumptions or limitations passages identified
below; it does not mean every proof or evaluation was checked. Prusti,
RustHornBelt and the 2026 RefinedRust successor received introduction reading.
No acquired tool, proof, benchmark or reference test was executed.

## Lineages and the distinct claims they support

| Lineage | Primary sources | Claim to investigate for Deleuze |
| --- | --- | --- |
| Ownership as modular resource reasoning | [Iris](https://www.mpi-sws.org/~dreyer/papers/iris-ground-up/paper.pdf), [RustBelt](https://plv.mpi-sws.org/rustbelt/popl18/paper.pdf) | Whether callers, producer slots and recorder buffers transfer rights without invalid aliases or lifetimes. Ownership safety alone does not assert retained-event completeness. |
| Functional constraints on Rust implementations | [Prusti](https://pm.inf.ethz.ch/publications/getpdf.php?bibname=Own&id=AstrauskasMuellerPoliSummers19b.pdf), [Flux](https://nilehmann.github.io/papers/pldi23-flux.pdf), [Verus](https://arxiv.org/abs/2303.05491), [RustHornBelt](https://people.mpi-sws.org/~dreyer/papers/rusthornbelt/paper.pdf), [RefinedRust](https://iris-project.org/pdfs/2024-pldi-refinedrust.pdf), [2026 successor](https://iris-project.org/pdfs/2026-oopsla-refinedrust2.pdf) | Whether a native state transition or unsafe abstraction satisfies its actual functional contract, with explicit language and trusted-tool boundaries. |
| Crash specifications composed with concurrent code | [FSCQ](https://pdos.csail.mit.edu/6.828/2018/readings/fscq-sosp15.pdf), [Perennial](https://people.csail.mit.edu/nickolai/papers/chajed-perennial.pdf), [GoJournal](https://pdos.csail.mit.edu/papers/gojournal:osdi21.pdf) | What can be retained, exposed and recovered after interruption, including crashes during recovery and concurrent operations. |
| Type-enforced persistence ordering | [SquirrelFS](https://www.usenix.org/system/files/osdi24-leblanc.pdf) | Which operation-order constraints can be enforced without repeated event-time dynamic checking, and what trusted operations still require validation. |
| Protocol-to-implementation refinement | [IronFleet](https://pdos.csail.mit.edu/6.824/papers/ironfleet.pdf) | How an implementation history relates to a simpler specification, and under which scheduling/network assumptions safety and liveness claims hold. |
| Persistence models and targeted falsification | [Ferrite](https://sandcat.cs.washington.edu/ferrite/ferrite-asplos16.pdf), [B3/CrashMonkey](https://www.usenix.org/system/files/osdi18-mohan.pdf) | Whether the assumed filesystem behaviors are meaningful, and which crash faults a concrete test actually exercises. |

These sources offer different mechanisms, not an ordinal proof-strength score.
A static type check, a theorem over a language model, a bounded counterexample
search and a device experiment have different premises and fault classes.

## Focused reading notes

**Iris, 2018, introduction and concurrent separation overview.** Ownership is a
logical resource, and communication can transfer it while physical memory is
shared. Invariants and ghost state let proofs expose abstractly separate rights
over shared structures. This is conceptual material for ring-slot and publication
reasoning. The paper's general logic is not a proof of a Rust queue, hardware
memory order or filesystem persistence model.

**RustBelt, 2018, introduction and safe/unsafe extension account.** A safe-facing
library can extend the ownership discipline using internally unsafe operations,
provided the implementation satisfies the semantic requirements of its API.
The work formalizes a Rust-like language rather than the entire current Rust
compiler. Its relevance is the obligation behind a fast unsafe implementation;
using Rust at the public boundary alone does not discharge that obligation.

**Prusti, 2019, introduction.** Rust's capability information is used to construct
proof obligations while users state functional properties at Rust-expression
level. The paper explicitly targets a subset without unsafe. This establishes a
nearby ergonomic goal, not coverage of a low-level recorder's unsafe or concurrent
implementation. Current Prusti capabilities were not assessed from this paper.

**Flux, 2023, §2 and selected formalization/limitations passages.** Refinements
ride on ownership, including strong updates and container constraints; refinement
parameters are ghost values rather than event-time data. The paper distinguishes
its lightweight targets from more expressive program logic and relies on a
borrowing model. Its benchmark gains concern verification work, not event-ingest
speed. Some illustrative overflow assumptions must remain attached to examples.

**Verus, arXiv v2 of 11 March 2023, modes and boundary sections.** Executable,
proof and specification modes have different linearity rules, and ghost code is
erased before machine-code generation. This offers a concrete approach to
expressing extra obligations without storing proof objects in events. The paper
states limits on its pointer/provenance model; the current pinned implementation
is a different revision. Erasure does not predict Deleuze throughput or remove
proof-maintenance cost.

**RustHornBelt, 2022, introduction.** It extends semantic typing to functional
specifications for safe interfaces implemented with unsafe code, using parametric
prophecies to describe mutable-borrow behavior. This addresses a different
obligation from memory safety alone. The formal development and end-to-end
connection to present-day verification front ends remain unreviewed here.

**RefinedRust, 2024, introduction, model gaps and related/future-work passages.**
The tool generates foundational Coq proofs for annotated safe/unsafe surface Rust
using refined ownership types and a more concrete operational model. It explicitly
contrasts that model with RustBelt's idealizations, and notes unsupported Rust
features and specification gaps. The useful research question is which small
critical recorder abstraction admits an accurate, economical model; this is not
a reason to select a general proof stack for the crate in advance.

**RefinedRust successor, October 2026, introduction.** The new paper extends traits,
closures and iterators and verifies parts of an ACE security monitor's memory
subsystem. This is a concrete forward citation beyond the 2024 prototype. Its
claims concern that development and supported semantics; it is introduction-read,
not a reviewed replacement backend or proof of arbitrary Rust application code.

**FSCQ, 2015, crash-logic and composition overview.** Crash Hoare Logic adds crash
conditions and recovery procedures to ordinary pre/post reasoning. Intermediate
storage states and repeated recovery are part of the specification. The paper
also makes storage assumptions such as atomic sector writes explicit. This
provides a way to ask what a recorder promises across an interrupted append;
it does not supply an unqualified statement about every deployed filesystem.

**Perennial, 2019, mechanism overview and §9.2.** Recovery leases, recovery helping
and versioned memory let crash reasoning coexist with concurrent operations.
The trusted boundary includes proof checking, the Goose model/translator,
compiler and stated primitive behavior. The historical paper even lists its
integer-overflow assumption. A proof claim has to name these premises, and a
current repository branch cannot be substituted for the historical system.

**GoJournal, 2021, introduction, specification assumptions and deferred-durability
limitation.** Logically atomic crash specs and crash framing reduce repeated
reasoning in clients. The paper assumes atomic 4 KiB block writes and a modeled
Go execution; callers still need a concurrency discipline. Its internal code and
proof support deferred durability, while the presented top-level spec immediately
flushes committed operations. Keeping these boundaries distinct is directly
relevant to a fast pipeline's producer completion and durable publication.

**IronFleet, 2015, §§2–3 and stated assumptions.** State-machine refinement is
combined with imperative verification, with trusted specifications and runtime
interfaces. Liveness additionally requires environmental assumptions such as
fair delivery/scheduling. This is useful when distinguishing a correct prefix
from eventual forward progress; it does not assign a numerical failure probability
or determine which distributed architecture Deleuze needs.

**SquirrelFS, 2024, §3 and especially §3.5.** Rust typestate restricts update order
for synchronous soft updates on byte-addressable persistent memory. The paper
explicitly says this checks ordering-based invariants, not the implementation
of each transition function. An Alloy model addresses the separate question of
whether the selected order is crash-consistent. Its x86 PM persistence model,
atomic update sizes and trusted operations prevent exporting the headline claim
to arbitrary file-backed logs. This is a particularly close bridge to Verbs.

**Ferrite, 2016, model/interface overview.** Crash-consistency models give explicit
allowed and forbidden persistent outcomes, with litmus tests and operational/
axiomatic descriptions. The artifact can compare these models with real
filesystems. This helps replace implicit rename/fsync folklore with a specific
model; the acquired historical model is not current behavior of every filesystem.

**B3/CrashMonkey, 2018, bounds and §4.4.** It searches small workloads around
explicit persistence points. Its completeness is bounded: it does not simulate
all mid-operation crash points or reorder all I/O. That makes it valuable direct
fault finding for its class, not evidence that every recorder/crash schedule is
covered. The wider durable-log lane independently acquired and read this work.

## Source correspondence: current branches can change the claim

Pinned source metadata is in the reference tables and manifest. Static source
inspection was limited to the following files; retained unmentioned files remain
unread.

- `go-journal/jrnl/jrnl.go` names unstable asynchronous operations and
  `CommitWait(wait)`. `wait=true` promises durability on return; `wait=false`
  requires a later flush. `wal/wal.go:164` waits for the requested durable log
  position. The current repository calls the wider system GoTxn, while retaining
  the journaling API. The 2021 paper and this source snapshot are separate
  revisions; we did not prove their equivalence.
- The latest `mit-pdos/perennial` default HEAD, branch `lean`, explicitly says
  it has **no crash/recovery reasoning**. Its README is not the contract of the
  2019/2021 crash proof. A separate pinned `master`/Rocq tree was acquired; its
  README and organization identify concurrency/crash frameworks and crash logic.
  Those proof bodies were not checked. The exact split is a major provenance
  finding, not a criticism of either branch.
- `verus/examples/atomic_increment.rs` exposes separate permission and logically
  atomic-update contracts, with a compare-exchange retry loop for the latter.
  An atomic variable's type is not by itself the application's operation contract.
  Only source structure was inspected; Verus was not executed on the example.
- Ferrite's README and example/model inventory were inspected, not its complete
  engine or filesystem experiments. Flux's project attribution and layout were
  inspected, not the soundness of its compiler implementation. SquirrelFS's `linux/fs/squirrelfs/typestate.rs` marker/trait declarations and
  `pm.rs` fence wrappers were also inspected. The marker file contains author
  TODO/FIXME notes about permissive state traits; these are open static concerns,
  not reproduced crash bugs. Transition bodies and model/proof results remain
  unreviewed.

## Remaining citation fronts

The source proofs, translator semantics and exact historical artifacts remain
separate work from reading the papers. Strong forward/backward fronts include
Grove, Trillium, Aneris, Argosy/DFSCQ, certified concurrent abstraction layers,
relaxed-memory logics, Stacked/Tree Borrows, bounded Rust model checking and
systematic schedule testing. Some are cited in the acquired originals but have
not been acquired or read in this lane. No claim of literature exhaustion or tool
coverage follows from the current fifteen-paper catalogue.
