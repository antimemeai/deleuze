# Capture contract development and review

Continued from the operator's small-envelope and sole-integration-path requirements.
Drafted design/capture-contract.md: one properties field, ordered entries with
typed/encoded values, common capture ownership, cancellation and failure behavior,
and managed-content/reference publication rules. This is a candidate rather than
a selected codec or implementation plan.

The operator rejected preserving requests in their entirety. Removed the optional
extra-body-copy branch and request-archival implications. Event contents are the
only retained object. Opaque content replaces an otherwise lossy representation;
there is no second capture layer or mandatory request-lifecycle stream. Updated
project instructions to preserve that boundary.

Read and retained eight public primary originals covering gRPC, Protobuf,
ProtoJSON/JSON, Rust API visibility/futures and HTTP status semantics. Reading
depths and hashes are in papers/survey/gate-protocols/. No parser/server/compiler
experiment or performance measurement was run and no dependency was adopted.

Two independent bounded reviews attacked representation/protocol conversion and
ownership/cancellation/resource failures. Integrated all valid findings and their
counterexamples; papers/reviews/capture-contract.md records dispositions. Final
review closed the original counterexamples at written-contract level, leaving
concrete gate hooks, execution lifetime, capacity and recovery mechanisms for
later selection and scrutiny. Private source information remains outside the
repository; the stopped research CLI was not relaunched.
