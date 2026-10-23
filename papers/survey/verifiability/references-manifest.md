# Verification reference acquisitions

2026-10-23. These are source-reading acquisitions, not dependency choices. Historical papers and current code revisions are distinct. Perennial default HEAD is the Lean branch without crash logic; its separately retained master/Rocq branch carries crash frameworks. No proof or reference program was run. Exact selections inspected are identified in report.md.

## verus

- Source: https://github.com/verus-lang/verus
- Revision: `8be02ff08cd3602be2969fe146a651f1a9c11da4`
- Original: `quarantine/archives/verifiability/verus-8be02ff08cd3602be2969fe146a651f1a9c11da4.tar.gz`
- SHA-256: `2fc01a4ef3028b3f4d02ff4bcf25b57981ed39cf2b609dbbd1ff088aeb87a32b`
- Extraction: `quarantine/sources/verifiability/verus`
- License evidence files: LICENSE
- Reading: selected-source-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/verus-lang/verus/tar.gz/8be02ff08cd3602be2969fe146a651f1a9c11da4' -o 'quarantine/archives/verifiability/verus-8be02ff08cd3602be2969fe146a651f1a9c11da4.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/verus`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## perennial

- Source: https://github.com/mit-pdos/perennial
- Revision: `f97c83addc2f66f8d58004321f0b1f055c046c30`
- Original: `quarantine/archives/verifiability/perennial-f97c83addc2f66f8d58004321f0b1f055c046c30.tar.gz`
- SHA-256: `305c64ced706c342b55d3a78df520a56db7e1ab4bd938a80cefd486bcf14383c`
- Extraction: `quarantine/sources/verifiability/perennial`
- License evidence files: not established
- Reading: readme-and-layout-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/mit-pdos/perennial/tar.gz/f97c83addc2f66f8d58004321f0b1f055c046c30' -o 'quarantine/archives/verifiability/perennial-f97c83addc2f66f8d58004321f0b1f055c046c30.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/perennial`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## go-journal

- Source: https://github.com/mit-pdos/go-journal
- Revision: `1d09d436fcbd943296f43372c3896714254838c0`
- Original: `quarantine/archives/verifiability/go-journal-1d09d436fcbd943296f43372c3896714254838c0.tar.gz`
- SHA-256: `e499017ab628bf3da952f7ce37fce0d64655d3a2f12b60907bbbee5734e63aa5`
- Extraction: `quarantine/sources/verifiability/go-journal`
- License evidence files: LICENSE
- Reading: selected-source-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/mit-pdos/go-journal/tar.gz/1d09d436fcbd943296f43372c3896714254838c0' -o 'quarantine/archives/verifiability/go-journal-1d09d436fcbd943296f43372c3896714254838c0.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/go-journal`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## squirrelfs

- Source: https://github.com/utsaslab/squirrelfs
- Revision: `e49c0125effc7e3ebd3b834d554ac4a498667b83`
- Original: `quarantine/archives/verifiability/squirrelfs-e49c0125effc7e3ebd3b834d554ac4a498667b83.tar.gz`
- SHA-256: `925f99d01ef654327c9062d579d777434901db14f2d76d79d50b56cad1636401`
- Extraction: `quarantine/sources/verifiability/squirrelfs`
- License evidence files: linux/COPYING; linux/LICENSES/preferred/GPL-2.0; squirrelfs module SPDX GPL-2.0
- Reading: selected typestate marker and PM wrapper source inspected; broader source unread

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/utsaslab/squirrelfs/tar.gz/e49c0125effc7e3ebd3b834d554ac4a498667b83' -o 'quarantine/archives/verifiability/squirrelfs-e49c0125effc7e3ebd3b834d554ac4a498667b83.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/squirrelfs`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## flux

- Source: https://github.com/flux-rs/flux
- Revision: `a636dfb1737f9b93a824a4ac42445a8152b6f4e1`
- Original: `quarantine/archives/verifiability/flux-a636dfb1737f9b93a824a4ac42445a8152b6f4e1.tar.gz`
- SHA-256: `cade98b351867fbad8096fdd76d88a6736ee58be9864e7bc2004b86927ba17a1`
- Extraction: `quarantine/sources/verifiability/flux`
- License evidence files: LICENSE
- Reading: readme-and-layout-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/flux-rs/flux/tar.gz/a636dfb1737f9b93a824a4ac42445a8152b6f4e1' -o 'quarantine/archives/verifiability/flux-a636dfb1737f9b93a824a4ac42445a8152b6f4e1.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/flux`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## ferrite

- Source: https://github.com/uwplse/ferrite
- Revision: `923736167fb00aec979f244787a8c33e48e1c551`
- Original: `quarantine/archives/verifiability/ferrite-923736167fb00aec979f244787a8c33e48e1c551.tar.gz`
- SHA-256: `7000d082d2fe1cacc641389ac50620c8fca169557dd73376956549d6470c3a11`
- Extraction: `quarantine/sources/verifiability/ferrite`
- License evidence files: LICENSE
- Reading: readme-and-layout-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/uwplse/ferrite/tar.gz/923736167fb00aec979f244787a8c33e48e1c551' -o 'quarantine/archives/verifiability/ferrite-923736167fb00aec979f244787a8c33e48e1c551.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/ferrite`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## perennial-coq

- Source: https://github.com/mit-pdos/perennial
- Revision: `3e3e0a484d61d0a15b578f5d5887ead84f145d71`
- Original: `quarantine/archives/verifiability/perennial-coq-3e3e0a484d61d0a15b578f5d5887ead84f145d71.tar.gz`
- SHA-256: `02cd4d448690f2862e84f00a05f5847b37842a2f980bd4de1b2d8b78b90a3c91`
- Extraction: `quarantine/sources/verifiability/perennial-coq`
- License evidence files: LICENSE
- Reading: readme-and-layout-inspected

Restore original:

```sh
mkdir -p quarantine/archives/verifiability
curl -fL 'https://codeload.github.com/mit-pdos/perennial/tar.gz/3e3e0a484d61d0a15b578f5d5887ead84f145d71' -o 'quarantine/archives/verifiability/perennial-coq-3e3e0a484d61d0a15b578f5d5887ead84f145d71.tar.gz'
```

Compare archive SHA-256 above before extraction. Extract the archive's single top-level source prefix into `quarantine/sources/verifiability/perennial-coq`, omitting nested Git metadata and filesystem detritus. Archive bytes remain intact. Code histories and submodule/toolchain/build closures were not acquired.

## Scope

The SquirrelFS original contains a modified Linux 6.3.0 tree plus file-system code and model/evaluation material. Its full retained archive is about 896 MiB. It is a source artifact, not an installed kernel or an executable experiment. RefinedRust historical artifact (DOI 10.5281/zenodo.10912439), foundational RustBelt/RustHornBelt developments and exact FSCQ/IronFleet artifacts remain acquisition fronts; papers alone do not supply those proofs.
