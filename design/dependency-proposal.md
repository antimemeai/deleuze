# Initial implementation dependency proposal

2026-10-23. The operator approved the record profile and requested progress.
AGENTS.md requires discussing dependency choices before adoption. The operator
accepted the official BLAKE3 choice after that discussion. Other dependencies
remain proposals; no transport stack has been adopted.

| Component | Proposal | Purpose and boundary |
| --- | --- | --- |
| Native core | `blake3` 1.8.7, plus Rust standard library | Ordinary property integrity through the documented hash API. Standard non-cryptographic framing checks remain separate; no composed cryptographic security protocol. The core stays independent of an async runtime, HTTP and gRPC. |
| Optional HTTP gate | Tokio 1.x and Axum 0.8.x | Existing HTTP/1 and HTTP/2 transport/runtime infrastructure, kept out of default native-only builds. |
| Optional gRPC gate | Evaluate Tonic 0.14.x on that stack versus a small raw-content service using HTTP/2 framing | Ordinary content must not require a Deleuze record object. A generated decoder alone cannot account for partial/pre-decoder failures. The concrete plan must resolve this before choosing extra codec/codegen dependencies. |

Primary API documentation checked: [BLAKE3](https://docs.rs/blake3/latest/blake3/),
[Tokio](https://docs.rs/tokio/latest/tokio/),
[Axum](https://docs.rs/axum/latest/axum/) and
[Tonic](https://docs.rs/tonic/latest/tonic/). Observed versions were 1.8.7, 1.54.1,
0.8.9 and 0.14.6 respectively; Cargo compatibility and lock resolution still need
verification when dependencies are adopted.

BLAKE3 provides an established incremental hashing implementation instead of
adding a homegrown cryptographic hash to Deleuze. No performance number is claimed
for this crate yet. The first native slice can be implemented and validated
independently while the transport plan resolves its concrete observation hooks,
content ownership and interoperability. Network dependencies stay optional.

Transport plumbing and any additional direct dependencies must be listed by the
concrete gate plan rather than hidden as implicit choices. No policy engine,
database engine, JSON internal model or producer-side masking dependency is
proposed. Publishing a meaningful crate remains separate from adopting packages;
this does not publish a placeholder or reserve a registry name.

The operator's follow-up requires restraint in cryptographic composition; see
[the cryptographic boundary](cryptographic-boundary.md). The first native design
uses plain property hashing, not custom cryptographic header domains, segment-tip
hash linking, signing or authentication.
