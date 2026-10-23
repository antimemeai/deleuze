# BLAKE3 dependency evaluation

2026-10-23. The proposed dependency is the official
[`blake3` Rust crate](https://github.com/BLAKE3-team/BLAKE3), not an independent
third-party reimplementation. The operator accepted that choice after discussing
maintenance, then emphasized avoiding incorrect/custom protocol composition.

## Primary evidence checked

- The [official project](https://github.com/BLAKE3-team/BLAKE3) identifies the Rust
  crate, reference implementation, specification and published test vectors.
- [Release 1.8.7](https://github.com/BLAKE3-team/BLAKE3/releases/tag/1.8.7) was
  published 2026-08-20. Its change removes `arrayref` following a crates.io owner
  compromise. This does not claim that a malicious dependency payload was used.
- GitHub's public contributors API returned 77 contributor accounts; most activity
  is concentrated in a small core. Accounts/contribution counts are maintenance
  evidence, not a count of independent cryptographic reviewers.
- Public commit activity includes September 2026 contributions and an October 1
  [soundness fix](https://github.com/BLAKE3-team/BLAKE3/commit/f55849f89cd85083c1c9daa2c5de20766f309c00).
  Its upstream explanation says a hidden platform API could write out of bounds
  for a short output buffer; the documented public API is unaffected. The planned
  use is the documented fixed-length hash API, not hidden/platform/XOF helpers.
  This is a bounded upstream-claim review, not an independent code audit.

The conclusion is actively maintained with community contributions and a small
core team. No independent audit, certification or protocol-security conclusion is
inferred. See [the cryptographic boundary](../../../design/cryptographic-boundary.md).

## Retained reference

- Version/tag: `1.8.7`.
- Source archive: `https://codeload.github.com/BLAKE3-team/BLAKE3/tar.gz/refs/tags/1.8.7`.
- Original: `quarantine/archives/dependencies/blake3-1.8.7.tar.gz`.
- SHA-256: `c6782a28842b1c0478524ac06a4f2ede784038ee298d6e2162c0b089c4306a3c`.
- Extracted reference: `quarantine/sources/dependencies/blake3-1.8.7/` (105 files).
- Rights evidence retained: `LICENSE_A2`, `LICENSE_A2LLVM`, `LICENSE_CC0`.
- Selected reading: project identity/release notes, relevant upstream fix,
  documented hashing API and official vector layout. Full optimized implementation
  and cryptanalysis have not been independently audited here.

Restore the archive from its recorded URL and compare SHA-256 before extracting
regular members without the top-level directory prefix. Strip Git metadata and
filesystem detritus only from the extracted reference; retain original bytes.
No acquired implementation code is copied into Deleuze source.
