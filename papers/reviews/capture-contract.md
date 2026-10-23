# Capture contract: bounded design review

2026-10-23. Two independent reviews pressure-tested the
[candidate contract](../../design/capture-contract.md): protocol/representation
and ownership/retention. They used public specifications and generic reasoning.
No implementation exists to test, and no runtime correctness or performance claim
follows from resolving a written-design counterexample.

The operator explicitly rejected whole-request preservation. That requirement
superseded the earlier optional-body-copy discussion. All dispositions below
concern event contents only, with one retained representation. There is no request
archive, additional evidence stream or mandatory request-lifecycle record.

| Finding and counterexample | Design disposition |
| --- | --- |
| P0: content observed, conversion starts, caller task cancelled before handoff | First content callback synchronously enters the common operation with owned content/reader; conversion operates under that ownership. |
| P0: bytes move into a task that still belongs to the caller's execution scope | Successful handoff establishes engine-owned execution. A droppable completion observer cannot own the only recording task. Failure to establish ownership is synchronous capture failure. |
| P0: fallback fails, then a failure event recursively enters the exhausted path | Nonrecursive failure handling: retain ownership while waiting or expose the actual failed/unknown outcome. Operational health reporting need not create another event. |
| P1: an incremental decoder releases content before a late conversion error | Keep a safe owned representation of uncommitted content until normal conversion or opaque handoff succeeds. Buffering is temporary; large streamed content can choose its artifact/reference representation once. |
| P1: cancelling a stalled input reader also destroys acquired content | Acquisition and retention cancellation are separate. Ending further reads preserves acquired contents; limits cannot turn discarded content into successful capture. |
| P1: uncertain event publication makes a referenced artifact look orphaned | Immutable managed content stays protected until recovery resolves publication uncertainty. Lack of a currently visible reference is insufficient for reclamation. |
| P1: generic map/double conversion erases duplicate entries, order, bytes or exact integers | Candidate Properties retains ordered pairs and native value distinctions. An adapter must satisfy that content mapping or use encoded content before destructive conversion. |
| Invalid name cannot fit Text; encoding only its value leaves the name invalid | Use encoded representation for the enclosing event-content container. The replacement is one Event, not an additional record. |
| Unknown/duplicate outer `properties`, or null silently becoming empty | Strict outer-schema handling requires a lossless mapping or enclosing-content fallback. A valid empty sequence is distinct. |
| Floating-point width or NaN bits change during widening/canonicalization | Candidate explicitly supports f32/f64 bit patterns; unsupported widths can remain encoded content. |
| Engine retained content but the client lost the completion reply | Engine retention and observer-relative unknown completion can both apply; a local send result does not prove delivery. |
| Request arrival and first content observation are named as though identical | Each integration must identify both and bridge the interval. Unavailable content does not become a successful empty value or a fabricated request record. |

The final passes found these original counterexamples addressed at contract
level. Concrete hooks, runtime ownership, capacity policy, recovery and reclamation
mechanisms remain implementation obligations once selected. Compression and extra
unary-message handling remain gate-codec obligations; they do not justify copying
transport requests.

The protocol review also narrowed an initially overbroad wire-retention concern:
normalization defined by a declared content mapping is not inherently event-content
loss. Protobuf scalar last-value behavior alone does not require retaining every
wire occurrence. The actual fault is losing distinctions the Event contract
promises before determining that conversion is safe.

The [public protocol study](../survey/gate-protocols/README.md) and retained primary
originals support the protocol claims. This review does not approve a storage
engine, dependency, binary codec, transport framework or implementation plan.
