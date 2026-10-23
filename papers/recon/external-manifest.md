# External acquisition manifest

Acquired 2026-10-23 for source research only. No dependency adoption or production reuse is authorized by acquisition. Archives and PDF originals remain intact; extracted trees omit nested `.git` and filesystem detritus. All SHA-256 values below describe the acquired original bytes, not derived PDF text. Live documentation is identified by acquisition date and content hash rather than an invented release number. Restoring a live URL later can produce different bytes; compare the digest before relying on this snapshot.

All restoration commands run from the repository root. Download commands use `curl -fL`; create parent directories first. Repository archives do not include submodule contents; see the shopping list for scope.

## arrow-columnar

- Source/original URL: https://arrow.apache.org/docs/format/Columnar.html
- Authors: Apache Arrow contributors; Apache Software Foundation
- Revision: Upstream living documentation; exact bytes pinned by SHA-256
- License/rights: Apache project documentation; page copyright/terms retained in original
- Original: `quarantine/sources/external/documents/arrow-columnar.html` (184,463 bytes)
- SHA-256: `af2a4cdfea443c758f5ebad5f0f5ef77f74e61cb8d0af585ae175b3176b30194`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://arrow.apache.org/docs/format/Columnar.html' -o 'quarantine/sources/external/documents/arrow-columnar.html'
printf '%s  %s\n' 'af2a4cdfea443c758f5ebad5f0f5ef77f74e61cb8d0af585ae175b3176b30194' 'quarantine/sources/external/documents/arrow-columnar.html' | shasum -a 256 -c -
```

## capnproto-encoding

- Source/original URL: https://capnproto.org/encoding.html
- Authors: Kenton Varda and Cap’n Proto contributors
- Revision: Upstream living documentation; exact bytes pinned by SHA-256
- License/rights: Page-specific documentation terms not independently established; retained original
- Original: `quarantine/sources/external/documents/capnproto-encoding.html` (28,614 bytes)
- SHA-256: `422b8c96e35e7a307ba05d5705aa9aae352de21f0f42527ca9030ffc1b67c1e4`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://capnproto.org/encoding.html' -o 'quarantine/sources/external/documents/capnproto-encoding.html'
printf '%s  %s\n' '422b8c96e35e7a307ba05d5705aa9aae352de21f0f42527ca9030ffc1b67c1e4' 'quarantine/sources/external/documents/capnproto-encoding.html' | shasum -a 256 -c -
```

## foundationdb-sigmod21

- Source/original URL: https://www.foundationdb.org/files/fdb-paper.pdf
- Authors: Jingyu Zhou; Meng Xu; Alexander Shraer; Bala Namasivayam; Alex Miller; Evan Tschannen; Steve Atherton; Andrew J. Beamon; Rusty Sears; John Leach; Dave Rosenthal; Xin Dong; Will Wilson; Ben Collins; David Scherer; Alec Grieser; Young Liu; Alvin Moore; Bhaskar Muppana; Xiaoge Su; Vishesh Yadav
- Revision: SIGMOD 2021; DOI 10.1145/3448016.3457559
- License/rights: Copyright owner/authors; publication rights licensed to ACM; research/classroom-copy notice in original
- Original: `papers/pdfs/foundationdb-sigmod21.pdf` (1,053,117 bytes)
- SHA-256: `ce786e9965dbc04f3e5ae1d57d6a895ee2dafb1b89f8f9c82355502895562b26`

Restore:

```sh
mkdir -p 'papers/pdfs'
curl -fL 'https://www.foundationdb.org/files/fdb-paper.pdf' -o 'papers/pdfs/foundationdb-sigmod21.pdf'
printf '%s  %s\n' 'ce786e9965dbc04f3e5ae1d57d6a895ee2dafb1b89f8f9c82355502895562b26' 'papers/pdfs/foundationdb-sigmod21.pdf' | shasum -a 256 -c -
mkdir -p quarantine/sources/external/documents
pdftotext -layout 'papers/pdfs/foundationdb-sigmod21.pdf' 'quarantine/sources/external/documents/foundationdb-sigmod21.txt'
```

## lamport-time-clocks

- Source/original URL: https://lamport.azurewebsites.net/pubs/time-clocks.pdf
- Authors: Leslie Lamport
- Revision: Communications of the ACM 21(7), July 1978, pp. 558–565
- License/rights: Copyright ACM 1978; research-copy permission stated in paper
- Original: `papers/pdfs/lamport-time-clocks.pdf` (854,562 bytes)
- SHA-256: `c55e7cab4230aa3d7126748a149b2db6f0d7a67296d5eccfdd50a210299a96b2`

Restore:

```sh
mkdir -p 'papers/pdfs'
curl -fL 'https://lamport.azurewebsites.net/pubs/time-clocks.pdf' -o 'papers/pdfs/lamport-time-clocks.pdf'
printf '%s  %s\n' 'c55e7cab4230aa3d7126748a149b2db6f0d7a67296d5eccfdd50a210299a96b2' 'papers/pdfs/lamport-time-clocks.pdf' | shasum -a 256 -c -
mkdir -p quarantine/sources/external/documents
pdftotext -layout 'papers/pdfs/lamport-time-clocks.pdf' 'quarantine/sources/external/documents/lamport-time-clocks.txt'
```

## leveldb

- Source: https://github.com/google/leveldb
- Authors: The LevelDB Authors; Google
- Revision: `bb74ef739973a70ca9f0d90788a1da89b788c0ad` (resolved upstream HEAD at acquisition)
- License: BSD-3-Clause (LICENSE)
- Original archive: `quarantine/archives/external/leveldb-bb74ef739973a70ca9f0d90788a1da89b788c0ad.tar.gz` (245,522 bytes)
- Working copy: `quarantine/sources/external/leveldb/`
- Archive URL: https://codeload.github.com/google/leveldb/tar.gz/bb74ef739973a70ca9f0d90788a1da89b788c0ad
- SHA-256: `fa5cad533bef27fb0d693c10fe14b40c4b920813340d7c8aeccfa17c4fd2a73d`

Restore:

```sh
mkdir -p quarantine/archives/external quarantine/sources/external/leveldb
curl -fL 'https://codeload.github.com/google/leveldb/tar.gz/bb74ef739973a70ca9f0d90788a1da89b788c0ad' -o 'quarantine/archives/external/leveldb-bb74ef739973a70ca9f0d90788a1da89b788c0ad.tar.gz'
printf '%s  %s\n' 'fa5cad533bef27fb0d693c10fe14b40c4b920813340d7c8aeccfa17c4fd2a73d' 'quarantine/archives/external/leveldb-bb74ef739973a70ca9f0d90788a1da89b788c0ad.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/external/leveldb-bb74ef739973a70ca9f0d90788a1da89b788c0ad.tar.gz' --strip-components=1 -C 'quarantine/sources/external/leveldb'
find 'quarantine/sources/external/leveldb' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/external/leveldb' -name .DS_Store -delete
```

## opentelemetry-proto

- Source: https://github.com/open-telemetry/opentelemetry-proto
- Authors: OpenTelemetry Authors
- Revision: `6c9dd787c3c5a5dae7d18ece0927581d5687f9c0` (resolved upstream HEAD at acquisition)
- License: Apache-2.0 (LICENSE)
- Original archive: `quarantine/archives/external/opentelemetry-proto-6c9dd787c3c5a5dae7d18ece0927581d5687f9c0.tar.gz` (124,743 bytes)
- Working copy: `quarantine/sources/external/opentelemetry-proto/`
- Archive URL: https://codeload.github.com/open-telemetry/opentelemetry-proto/tar.gz/6c9dd787c3c5a5dae7d18ece0927581d5687f9c0
- SHA-256: `6ac565e5ab038f2e8c0d282d4e5b9034f646a3a54f75bf55a4dd19c9eb2b8507`

Restore:

```sh
mkdir -p quarantine/archives/external quarantine/sources/external/opentelemetry-proto
curl -fL 'https://codeload.github.com/open-telemetry/opentelemetry-proto/tar.gz/6c9dd787c3c5a5dae7d18ece0927581d5687f9c0' -o 'quarantine/archives/external/opentelemetry-proto-6c9dd787c3c5a5dae7d18ece0927581d5687f9c0.tar.gz'
printf '%s  %s\n' '6ac565e5ab038f2e8c0d282d4e5b9034f646a3a54f75bf55a4dd19c9eb2b8507' 'quarantine/archives/external/opentelemetry-proto-6c9dd787c3c5a5dae7d18ece0927581d5687f9c0.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/external/opentelemetry-proto-6c9dd787c3c5a5dae7d18ece0927581d5687f9c0.tar.gz' --strip-components=1 -C 'quarantine/sources/external/opentelemetry-proto'
find 'quarantine/sources/external/opentelemetry-proto' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/external/opentelemetry-proto' -name .DS_Store -delete
```

## otel-logs-model

- Source/original URL: https://opentelemetry.io/docs/specs/otel/logs/data-model/
- Authors: OpenTelemetry Authors
- Revision: Upstream living specification; exact bytes pinned by SHA-256
- License/rights: Page-specific documentation terms not independently established; retained original
- Original: `quarantine/sources/external/documents/otel-logs-model.html` (94,742 bytes)
- SHA-256: `f3e1a033c715b091cfe67b9a33825bedd89dea430bc725aea51f430a3f8d995a`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://opentelemetry.io/docs/specs/otel/logs/data-model/' -o 'quarantine/sources/external/documents/otel-logs-model.html'
printf '%s  %s\n' 'f3e1a033c715b091cfe67b9a33825bedd89dea430bc725aea51f430a3f8d995a' 'quarantine/sources/external/documents/otel-logs-model.html' | shasum -a 256 -c -
```

## pillai-crash-consistency

- Source/original URL: https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-pillai.pdf
- Authors: Thanumalayan Sankaranarayana Pillai; Vijay Chidambaram; Ramnatthan Alagappan; Samer Al-Kiswany; Andrea C. Arpaci-Dusseau; Remzi H. Arpaci-Dusseau
- Revision: OSDI 2014, pp. 433–448
- License/rights: USENIX open-access original; specific redistribution license not independently established
- Original: `papers/pdfs/pillai-crash-consistency.pdf` (453,389 bytes)
- SHA-256: `44dd92938871b09d3233a09ca1234313e920c451863eb85e5cc719b61ccfc18b`

Restore:

```sh
mkdir -p 'papers/pdfs'
curl -fL 'https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-pillai.pdf' -o 'papers/pdfs/pillai-crash-consistency.pdf'
printf '%s  %s\n' '44dd92938871b09d3233a09ca1234313e920c451863eb85e5cc719b61ccfc18b' 'papers/pdfs/pillai-crash-consistency.pdf' | shasum -a 256 -c -
mkdir -p quarantine/sources/external/documents
pdftotext -layout 'papers/pdfs/pillai-crash-consistency.pdf' 'quarantine/sources/external/documents/pillai-crash-consistency.txt'
```

## redb

- Source: https://github.com/cberner/redb
- Authors: Christopher Berner and contributors
- Revision: `b04b3d557dbbe479d898f0e4a5375ce3667a3f7b` (resolved upstream HEAD at acquisition)
- License: MIT OR Apache-2.0 (LICENSE-MIT; LICENSE-APACHE)
- Original archive: `quarantine/archives/external/redb-b04b3d557dbbe479d898f0e4a5375ce3667a3f7b.tar.gz` (485,830 bytes)
- Working copy: `quarantine/sources/external/redb/`
- Archive URL: https://codeload.github.com/cberner/redb/tar.gz/b04b3d557dbbe479d898f0e4a5375ce3667a3f7b
- SHA-256: `7b09f6d86fd2085bb70b4cf5cf106c0fb5f4168aa47d8b770af7ae0d6c335134`

Restore:

```sh
mkdir -p quarantine/archives/external quarantine/sources/external/redb
curl -fL 'https://codeload.github.com/cberner/redb/tar.gz/b04b3d557dbbe479d898f0e4a5375ce3667a3f7b' -o 'quarantine/archives/external/redb-b04b3d557dbbe479d898f0e4a5375ce3667a3f7b.tar.gz'
printf '%s  %s\n' '7b09f6d86fd2085bb70b4cf5cf106c0fb5f4168aa47d8b770af7ae0d6c335134' 'quarantine/archives/external/redb-b04b3d557dbbe479d898f0e4a5375ce3667a3f7b.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/external/redb-b04b3d557dbbe479d898f0e4a5375ce3667a3f7b.tar.gz' --strip-components=1 -C 'quarantine/sources/external/redb'
find 'quarantine/sources/external/redb' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/external/redb' -name .DS_Store -delete
```

## rfc8949-cbor

- Source/original URL: https://www.rfc-editor.org/rfc/rfc8949.txt
- Authors: Carsten Bormann; Paul Hoffman
- Revision: RFC 8949, December 2020; STD 94
- License/rights: IETF Trust BCP 78 legal provisions; code components Simplified BSD per RFC notice
- Original: `quarantine/sources/external/documents/rfc8949-cbor.txt` (185,226 bytes)
- SHA-256: `f1164a5b31a39350ad46abe29b83575eb933ca6c45366989c118b6b1058a214a`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://www.rfc-editor.org/rfc/rfc8949.txt' -o 'quarantine/sources/external/documents/rfc8949-cbor.txt'
printf '%s  %s\n' 'f1164a5b31a39350ad46abe29b83575eb933ca6c45366989c118b6b1058a214a' 'quarantine/sources/external/documents/rfc8949-cbor.txt' | shasum -a 256 -c -
```

## rocksdb-wal

- Source/original URL: https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-File-Format
- Authors: RocksDB authors; wiki editor yiwu-arbug
- Revision: Page shows last edit 2020-10-19; fetched snapshot SHA-256
- License/rights: Wiki page terms not independently established; retained original
- Original: `quarantine/sources/external/documents/rocksdb-wal.html` (676,972 bytes)
- SHA-256: `8e986d29c8781a74bad055e24c0eaf32483699dcea8f98d90ed9e6ec349f23ce`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://github.com/facebook/rocksdb/wiki/Write-Ahead-Log-File-Format' -o 'quarantine/sources/external/documents/rocksdb-wal.html'
printf '%s  %s\n' '8e986d29c8781a74bad055e24c0eaf32483699dcea8f98d90ed9e6ec349f23ce' 'quarantine/sources/external/documents/rocksdb-wal.html' | shasum -a 256 -c -
```

## rr

- Source: https://github.com/rr-debugger/rr
- Authors: Robert O’Callahan and rr contributors
- Revision: `c145f2f80e63e9627b79deed6e0d3eae0d658e38` (resolved upstream HEAD at acquisition)
- License: MIT (LICENSE)
- Original archive: `quarantine/archives/external/rr-c145f2f80e63e9627b79deed6e0d3eae0d658e38.tar.gz` (1,851,990 bytes)
- Working copy: `quarantine/sources/external/rr/`
- Archive URL: https://codeload.github.com/rr-debugger/rr/tar.gz/c145f2f80e63e9627b79deed6e0d3eae0d658e38
- SHA-256: `aa68652ce6cd6eb099a54870c7bc0014ffb42144d3de4fb492cbe4f13fa4a937`

Restore:

```sh
mkdir -p quarantine/archives/external quarantine/sources/external/rr
curl -fL 'https://codeload.github.com/rr-debugger/rr/tar.gz/c145f2f80e63e9627b79deed6e0d3eae0d658e38' -o 'quarantine/archives/external/rr-c145f2f80e63e9627b79deed6e0d3eae0d658e38.tar.gz'
printf '%s  %s\n' 'aa68652ce6cd6eb099a54870c7bc0014ffb42144d3de4fb492cbe4f13fa4a937' 'quarantine/archives/external/rr-c145f2f80e63e9627b79deed6e0d3eae0d658e38.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/external/rr-c145f2f80e63e9627b79deed6e0d3eae0d658e38.tar.gz' --strip-components=1 -C 'quarantine/sources/external/rr'
find 'quarantine/sources/external/rr' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/external/rr' -name .DS_Store -delete
```

## sqlite-atomiccommit

- Source/original URL: https://www.sqlite.org/atomiccommit.html
- Authors: SQLite authors
- Revision: Upstream living documentation; exact bytes pinned by SHA-256
- License/rights: SQLite public-domain project; page-specific rights not independently established
- Original: `quarantine/sources/external/documents/sqlite-atomiccommit.html` (77,968 bytes)
- SHA-256: `5a5be7b660217c9408c8a81eaf2faa2d08832ed3790f895a0a713916ad856ea1`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://www.sqlite.org/atomiccommit.html' -o 'quarantine/sources/external/documents/sqlite-atomiccommit.html'
printf '%s  %s\n' '5a5be7b660217c9408c8a81eaf2faa2d08832ed3790f895a0a713916ad856ea1' 'quarantine/sources/external/documents/sqlite-atomiccommit.html' | shasum -a 256 -c -
```

## sqlite-wal

- Source/original URL: https://www.sqlite.org/wal.html
- Authors: SQLite authors
- Revision: Upstream living documentation; exact bytes pinned by SHA-256
- License/rights: SQLite public-domain project; page-specific rights not independently established
- Original: `quarantine/sources/external/documents/sqlite-wal.html` (42,143 bytes)
- SHA-256: `f3467b530b883d4a00574fe1a898b3d121ed72764ae28cf66941080ac0badb9e`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://www.sqlite.org/wal.html' -o 'quarantine/sources/external/documents/sqlite-wal.html'
printf '%s  %s\n' 'f3467b530b883d4a00574fe1a898b3d121ed72764ae28cf66941080ac0badb9e' 'quarantine/sources/external/documents/sqlite-wal.html' | shasum -a 256 -c -
```

## w3c-trace-context

- Source/original URL: https://www.w3.org/TR/2021/REC-trace-context-1-20211123/
- Authors: W3C Distributed Tracing Working Group
- Revision: W3C Recommendation, 2021-11-23
- License/rights: W3C document license; copyright notice retained in original
- Original: `quarantine/sources/external/documents/w3c-trace-context.html` (100,612 bytes)
- SHA-256: `9e7228d2a91c5aa4bef6e7f610366a2000274ee51699053e10c8ac3f0b8965be`

Restore:

```sh
mkdir -p 'quarantine/sources/external/documents'
curl -fL 'https://www.w3.org/TR/2021/REC-trace-context-1-20211123/' -o 'quarantine/sources/external/documents/w3c-trace-context.html'
printf '%s  %s\n' '9e7228d2a91c5aa4bef6e7f610366a2000274ee51699053e10c8ac3f0b8965be' 'quarantine/sources/external/documents/w3c-trace-context.html' | shasum -a 256 -c -
```

## rr-atc17

- Source/original URL: https://www.usenix.org/system/files/conference/atc17/atc17-o_callahan.pdf
- Authors: Robert O’Callahan; Chris Jones; Nathan Froyd; Kyle Huey; Albert Noll; Nimrod Partush
- Revision: USENIX ATC 2017, pp. 377–389
- License/rights: USENIX open-access original; specific redistribution license not independently established
- Original: `papers/pdfs/rr-atc17.pdf` (525,825 bytes)
- SHA-256: `589ec37978260f968312288061e710e64b58e2e652c8479649694403ac037772`

Restore:

```sh
mkdir -p papers/pdfs
curl -fL 'https://www.usenix.org/system/files/conference/atc17/atc17-o_callahan.pdf' -o 'papers/pdfs/rr-atc17.pdf'
printf '%s  %s\n' '589ec37978260f968312288061e710e64b58e2e652c8479649694403ac037772' 'papers/pdfs/rr-atc17.pdf' | shasum -a 256 -c -
```

## otel-logs-sdk

- Source/original URL: https://opentelemetry.io/docs/specs/otel/logs/sdk/
- Authors: OpenTelemetry Authors
- Revision: Living specification; acquired 2026-10-23; exact bytes pinned by SHA-256
- License/rights: Page-specific documentation terms not independently established; original retained
- Original: `quarantine/sources/external/documents/otel-logs-sdk.html` (111,862 bytes)
- SHA-256: `b70c1c4cd6e580a41ae68bf7bd21aa475d654f82a4ddb1960dfa5bd890956fdf`

Restore:

```sh
mkdir -p quarantine/sources/external/documents
curl -fL 'https://opentelemetry.io/docs/specs/otel/logs/sdk/' -o 'quarantine/sources/external/documents/otel-logs-sdk.html'
printf '%s  %s\n' 'b70c1c4cd6e580a41ae68bf7bd21aa475d654f82a4ddb1960dfa5bd890956fdf' 'quarantine/sources/external/documents/otel-logs-sdk.html' | shasum -a 256 -c -
```
