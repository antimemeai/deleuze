# Cryptographic boundary

2026-10-23. The operator approved the official BLAKE3 implementation after
discussion, and explicitly required restraint in protocol composition: a good
primitive does not make an assembled protocol secure.

For the first core, use the documented fixed-length unkeyed BLAKE3 API to compute
`property_hash` over the exact retained property bytes. The native encoding must
be precisely specified and tested; the digest does not repair an ambiguous
encoding. No custom cryptographic implementation or arbitrary hasher callback.

This field supports content-integrity checking. It is not authentication, proof
of producer identity, proof of completeness, authorization, replay prevention or
protection against a writer who can change both data and its unkeyed hash.
ABAC decisions never derive from possession of a property hash.

The earlier custom cryptographic header digests and segment-tip hash-link design
are withdrawn. Ordinary framing, strict parsing and standard non-cryptographic
checksums can detect accidental metadata damage without inventing a security
protocol. Their scope and failure model must stay explicit.

Do not add signing, MACs, key management, Merkle anchoring, hash-chain security or
another composed cryptographic protocol by assembling primitives ad hoc. If a
concrete requirement later needs those guarantees, evaluate an established,
reviewed protocol and maintained implementation as a separate design decision.
Correct tests are useful engineering checks, not cryptanalysis or proof that a
novel composition is secure.

Dependency maintenance evidence and exact source restoration are recorded in
[the public BLAKE3 note](../papers/survey/dependencies/blake3.md). The official
published test vectors inform direct hash/encoding oracles; no popularity count
is treated as a cryptographic assurance.
