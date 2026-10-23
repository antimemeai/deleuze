# What to take from Verbs as optimization objectives



The founding brief is orientation, not settled policy. Its §2 framing—subjects,
invariants, transformations and preservation—is useful. Its open-obligation metric,
agent evaluation plan, database/cache proposals and C++ realization sketches were
substantially narrowed or replaced in the later spec. The current spec and journal
must be read alongside it; quoting the old brief as Verbs' current behavior would
be misleading. No language, storage or verification-tool decision transfers from
Verbs into Deleuze just because these projects share goals.

| Objective for studying Deleuze | Grounding in Verbs | Implication to investigate, without selecting a design |
| --- | --- | --- |
| Remove invalid internal representations | Brief §2; current SPEC §§1–3 | Encode lengths, ranges, identities and legal states where representations or type checks can enforce them. Determine which checks disappear from the event hot path and which still belong at unchecked boundaries. |
| Make ownership and transfer explicit | Operator decisions, Scope of invariant claims; SPEC §2.1 rules 3–5 | Identify who owns caller bytes, queue slots, recorder buffers and retained pages at each transition. A borrowed view cannot silently outlive its owner. |
| Preserve invariants across every transformation | Brief §2 | Consider emission, handoff, encoding, I/O completion, error, shutdown, cancellation and recovery. Some useful claims concern before/after relations even though early Verbs itself checks only single-state predicates. |
| Establish narrow, checked unchecked-to-native boundaries | SPEC §3.1 and enum ingress examples in §1.1 | Avoid drift between declared domains and decoder checks. Preserve the event presented to the gate; this objective does not authorize a semantic filter that quietly excludes arrivals from capture accounting. |
| Check before mutating when failure permits it | SPEC §3.1 strong guarantee | Study staging before publication and explicit treatment of partially completed I/O. A storage error after publication requires a different claim from a value rejected before mutation. |
| Keep escapes and weakening visible | SPEC §§4.6–4.8 | An unsafe buffer path, erased FFI pointer, changed durability setting or disabled check creates a concrete obligation. A fast implementation should expose that boundary for review. |
| Prefer mechanisms that discharge obligations without event-time work | Brief §2 representation framing; SPEC §§3,10 | Study type/ownership/compile-time constraints and erased proof state before adding repeated dynamic bookkeeping. Quantify resulting code shape and actual costs; a source-level zero-cost claim is not a measured Deleuze result. |
| Give agents concrete mechanical feedback early | README; SPEC §6 | Make claims and failures easy to locate and act on. The latest Verbs design uses deterministic corpus checks, replacing its earlier agent experiment; neither mechanism is ready to be imported here. |
| Keep claimed evidence current and scoped | SPEC §6.2 | Missing or stale analysis must not become a pass. For recorder research, similarly distinguish a published prefix from bytes merely observed in memory or a partially written file. This is an analogy, not a shared storage algorithm. |
| Test the contract with independent oracles | SPEC §10 | Compare implementation histories against a simpler model, decode against independently specified expected values, and inject the actual failure class. Avoid measuring a mechanism with its own unchecked output as ground truth. |
| Minimize ceremony and unnecessary abstraction | SPEC §1, annotations clarify; operator decisions, Basics first | A requirement should buy a concrete invariant, fault observation, latency improvement or consumer capability. Avoid importing a broad annotation/proof backend framework merely to resemble Verbs. |
| Keep correctness goals independent of model availability | Operator decisions, Second round; SPEC §10.3 | Human-readable contracts and deterministic checks must remain usable without an LLM service. Model assistance can help research and review. |

## Boundaries that matter

Verbs' early checker explicitly excludes lock-scoped/lock-free shared mutation,
memory-ordering correctness, exception unwinding during mutation and moved-from
use. Those are current tool boundaries, not acceptable assumptions to silently
attach to a fast concurrent Deleuze recorder. Concurrent queues, publication,
lifetimes and persistence ordering need their own suitable evidence. The nearby
literature includes RustBelt, Iris, RefinedRust, Verus, SquirrelFS, Perennial,
GoJournal and crash-consistency models for precisely this reason.

Verbs' derived build facts are disposable and can be deleted and rebuilt after a
read/format failure. Captured event originals have a different role: deleting them
may destroy the thing Deleuze exists to retain. Rebuildable indexes and retained
observations must therefore be distinguished in the study.

Verbs' latest C++26/toolchain choice, 10-second edit-feedback budget, placeholder
registrations, suppression rules and exact vocabulary belong to Verbs. They do
not become Deleuze's Rust toolchain, runtime latency target, publication plan or
value model. The transferable objectives concern representation, ownership,
preservation, feedback, ergonomics and justified claims. Their relative weight
is governed by gate capture reliability, speed and the deployment budget.
