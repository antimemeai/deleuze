# Durable-log external source acquisitions

Acquired 2026-10-23, source-reading snapshots only. These archives are research material, not Deleuze dependencies or a license to reuse source. Original tarballs remain intact; extracted trees omit nested Git metadata and filesystem detritus. Build dependencies and submodules are outside these source-reading snapshots.

All restoration commands run from the Deleuze repository root. Compare original-byte SHA-256 values before reading a restored snapshot.

## bookkeeper

- Source: https://github.com/apache/bookkeeper
- Authors: Apache BookKeeper contributors
- Revision: `7b66b5d7631cecc9198c5d555d8b2d28d3b6525d` (upstream HEAD resolved 2026-10-23)
- License: Apache-2.0; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/bookkeeper-7b66b5d7631cecc9198c5d555d8b2d28d3b6525d.tar.gz` (4,656,903 bytes)
- SHA-256: `f050e858839c8a8f74bfc2053d0c05f3a024d9a3099b770c775c15691f878aa6`
- Extracted tree: `quarantine/sources/survey-durable/bookkeeper/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/bookkeeper
curl -fL 'https://codeload.github.com/apache/bookkeeper/tar.gz/7b66b5d7631cecc9198c5d555d8b2d28d3b6525d' -o 'quarantine/archives/survey-durable/bookkeeper-7b66b5d7631cecc9198c5d555d8b2d28d3b6525d.tar.gz'
printf '%s  %s\n' 'f050e858839c8a8f74bfc2053d0c05f3a024d9a3099b770c775c15691f878aa6' 'quarantine/archives/survey-durable/bookkeeper-7b66b5d7631cecc9198c5d555d8b2d28d3b6525d.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/bookkeeper-7b66b5d7631cecc9198c5d555d8b2d28d3b6525d.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/bookkeeper'
find 'quarantine/sources/survey-durable/bookkeeper' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/bookkeeper' -name .DS_Store -delete
```

## corfudb

- Source: https://github.com/CorfuDB/CorfuDB
- Authors: CorfuDB contributors
- Revision: `2513b5c61e19732376e8d34081b64695dd6f30cd` (upstream HEAD resolved 2026-10-23)
- License: Apache-2.0 (LICENSE.txt inspected); retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/corfudb-2513b5c61e19732376e8d34081b64695dd6f30cd.tar.gz` (5,999,127 bytes)
- SHA-256: `7582418da908b5ba0ae43034f8b4e0019b2eea0637bf0b0b5eea40c460731a64`
- Extracted tree: `quarantine/sources/survey-durable/corfudb/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/corfudb
curl -fL 'https://codeload.github.com/CorfuDB/CorfuDB/tar.gz/2513b5c61e19732376e8d34081b64695dd6f30cd' -o 'quarantine/archives/survey-durable/corfudb-2513b5c61e19732376e8d34081b64695dd6f30cd.tar.gz'
printf '%s  %s\n' '7582418da908b5ba0ae43034f8b4e0019b2eea0637bf0b0b5eea40c460731a64' 'quarantine/archives/survey-durable/corfudb-2513b5c61e19732376e8d34081b64695dd6f30cd.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/corfudb-2513b5c61e19732376e8d34081b64695dd6f30cd.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/corfudb'
find 'quarantine/sources/survey-durable/corfudb' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/corfudb' -name .DS_Store -delete
```

## etcd-raft

- Source: https://github.com/etcd-io/raft
- Authors: etcd Raft contributors
- Revision: `1931ff5beacc1e2539003c0c70f096285f04c425` (upstream HEAD resolved 2026-10-23)
- License: Apache-2.0; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/etcd-raft-1931ff5beacc1e2539003c0c70f096285f04c425.tar.gz` (315,426 bytes)
- SHA-256: `760c4ea7956f9e5638142d86f698d9c1eda481c37dae09f3e17d76b9672cf6ea`
- Extracted tree: `quarantine/sources/survey-durable/etcd-raft/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/etcd-raft
curl -fL 'https://codeload.github.com/etcd-io/raft/tar.gz/1931ff5beacc1e2539003c0c70f096285f04c425' -o 'quarantine/archives/survey-durable/etcd-raft-1931ff5beacc1e2539003c0c70f096285f04c425.tar.gz'
printf '%s  %s\n' '760c4ea7956f9e5638142d86f698d9c1eda481c37dae09f3e17d76b9672cf6ea' 'quarantine/archives/survey-durable/etcd-raft-1931ff5beacc1e2539003c0c70f096285f04c425.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/etcd-raft-1931ff5beacc1e2539003c0c70f096285f04c425.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/etcd-raft'
find 'quarantine/sources/survey-durable/etcd-raft' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/etcd-raft' -name .DS_Store -delete
```

## kafka

- Source: https://github.com/apache/kafka
- Authors: Apache Kafka contributors
- Revision: `da2224096692906b5e4398b2531fc4e810550c01` (upstream HEAD resolved 2026-10-23)
- License: Apache-2.0; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/kafka-da2224096692906b5e4398b2531fc4e810550c01.tar.gz` (15,453,794 bytes)
- SHA-256: `564eeaa6f9bee0b0b50d3db0d45f1caf06bb79eed1679faa5497e364733d24d9`
- Extracted tree: `quarantine/sources/survey-durable/kafka/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/kafka
curl -fL 'https://codeload.github.com/apache/kafka/tar.gz/da2224096692906b5e4398b2531fc4e810550c01' -o 'quarantine/archives/survey-durable/kafka-da2224096692906b5e4398b2531fc4e810550c01.tar.gz'
printf '%s  %s\n' '564eeaa6f9bee0b0b50d3db0d45f1caf06bb79eed1679faa5497e364733d24d9' 'quarantine/archives/survey-durable/kafka-da2224096692906b5e4398b2531fc4e810550c01.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/kafka-da2224096692906b5e4398b2531fc4e810550c01.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/kafka'
find 'quarantine/sources/survey-durable/kafka' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/kafka' -name .DS_Store -delete
```

## logdevice

- Source: https://github.com/facebookincubator/LogDevice
- Authors: Facebook/Meta and LogDevice contributors
- Revision: `ce7726050edc49a1e15d9160e81c890736b779e2` (upstream HEAD resolved 2026-10-23)
- License: BSD-3-Clause; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/logdevice-ce7726050edc49a1e15d9160e81c890736b779e2.tar.gz` (11,442,590 bytes)
- SHA-256: `ff14a9e52a46d8df044dbfc24223236f8cd0269d14e2982f947674ca2a3135ac`
- Extracted tree: `quarantine/sources/survey-durable/logdevice/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/logdevice
curl -fL 'https://codeload.github.com/facebookincubator/LogDevice/tar.gz/ce7726050edc49a1e15d9160e81c890736b779e2' -o 'quarantine/archives/survey-durable/logdevice-ce7726050edc49a1e15d9160e81c890736b779e2.tar.gz'
printf '%s  %s\n' 'ff14a9e52a46d8df044dbfc24223236f8cd0269d14e2982f947674ca2a3135ac' 'quarantine/archives/survey-durable/logdevice-ce7726050edc49a1e15d9160e81c890736b779e2.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/logdevice-ce7726050edc49a1e15d9160e81c890736b779e2.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/logdevice'
find 'quarantine/sources/survey-durable/logdevice' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/logdevice' -name .DS_Store -delete
```

## pmdk

- Source: https://github.com/pmem/pmdk
- Authors: PMDK contributors
- Revision: `52b9545b8d47d6689d957e096a5bc8a2cc8ac395` (upstream HEAD resolved 2026-10-23)
- License: BSD-3-Clause; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/pmdk-52b9545b8d47d6689d957e096a5bc8a2cc8ac395.tar.gz` (1,459,807 bytes)
- SHA-256: `9ea4071565c0aa119b3861471972850a040b51fa017c79e43442c9c4ce083c52`
- Extracted tree: `quarantine/sources/survey-durable/pmdk/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/pmdk
curl -fL 'https://codeload.github.com/pmem/pmdk/tar.gz/52b9545b8d47d6689d957e096a5bc8a2cc8ac395' -o 'quarantine/archives/survey-durable/pmdk-52b9545b8d47d6689d957e096a5bc8a2cc8ac395.tar.gz'
printf '%s  %s\n' '9ea4071565c0aa119b3861471972850a040b51fa017c79e43442c9c4ce083c52' 'quarantine/archives/survey-durable/pmdk-52b9545b8d47d6689d957e096a5bc8a2cc8ac395.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/pmdk-52b9545b8d47d6689d957e096a5bc8a2cc8ac395.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/pmdk'
find 'quarantine/sources/survey-durable/pmdk' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/pmdk' -name .DS_Store -delete
```

## pravega

- Source: https://github.com/pravega/pravega
- Authors: Pravega contributors
- Revision: `3142a01075bfae78d66588e46ff87e6e10225744` (upstream HEAD resolved 2026-10-23)
- License: Apache-2.0; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/pravega-3142a01075bfae78d66588e46ff87e6e10225744.tar.gz` (7,999,469 bytes)
- SHA-256: `41f4adba8ee133107763fee6f051514ebbb94c6a3527f0d3fe3e15285c167160`
- Extracted tree: `quarantine/sources/survey-durable/pravega/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/pravega
curl -fL 'https://codeload.github.com/pravega/pravega/tar.gz/3142a01075bfae78d66588e46ff87e6e10225744' -o 'quarantine/archives/survey-durable/pravega-3142a01075bfae78d66588e46ff87e6e10225744.tar.gz'
printf '%s  %s\n' '41f4adba8ee133107763fee6f051514ebbb94c6a3527f0d3fe3e15285c167160' 'quarantine/archives/survey-durable/pravega-3142a01075bfae78d66588e46ff87e6e10225744.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/pravega-3142a01075bfae78d66588e46ff87e6e10225744.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/pravega'
find 'quarantine/sources/survey-durable/pravega' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/pravega' -name .DS_Store -delete
```

## redpanda

- Source: https://github.com/redpanda-data/redpanda
- Authors: Redpanda contributors
- Revision: `4254992f6115e92c1dbd86f90bb0aee0b869f15e` (upstream HEAD resolved 2026-10-23)
- License: Mixed; inspect root and subtree LICENSE/NOTICE files before any use; retained originals govern individual files
- Original archive: `quarantine/archives/survey-durable/redpanda-4254992f6115e92c1dbd86f90bb0aee0b869f15e.tar.gz` (11,078,580 bytes)
- SHA-256: `06f12e0063895402fdd93ba195a9cb857bd9aeb02c2d137230da4fcefbdbcdf9`
- Extracted tree: `quarantine/sources/survey-durable/redpanda/`

Restore:

```sh
mkdir -p quarantine/archives/survey-durable quarantine/sources/survey-durable/redpanda
curl -fL 'https://codeload.github.com/redpanda-data/redpanda/tar.gz/4254992f6115e92c1dbd86f90bb0aee0b869f15e' -o 'quarantine/archives/survey-durable/redpanda-4254992f6115e92c1dbd86f90bb0aee0b869f15e.tar.gz'
printf '%s  %s\n' '06f12e0063895402fdd93ba195a9cb857bd9aeb02c2d137230da4fcefbdbcdf9' 'quarantine/archives/survey-durable/redpanda-4254992f6115e92c1dbd86f90bb0aee0b869f15e.tar.gz' | shasum -a 256 -c -
tar -xzf 'quarantine/archives/survey-durable/redpanda-4254992f6115e92c1dbd86f90bb0aee0b869f15e.tar.gz' --strip-components=1 -C 'quarantine/sources/survey-durable/redpanda'
find 'quarantine/sources/survey-durable/redpanda' -type d -name .git -prune -exec rm -rf {} +
find 'quarantine/sources/survey-durable/redpanda' -name .DS_Store -delete
```

## Literature originals

The tracked [bibliography](bibliography.tsv) records every source URL, original path, SHA-256, access outcome, and reading status. For each acquired row:

```sh
mkdir -p quarantine/archives/survey-durable/documents
# Substitute that row's exact URL, original_path, and sha256:
curl -fL 'URL' -o 'original_path'
printf '%s  %s\n' 'sha256' 'original_path' | shasum -a 256 -c -
```

PDF reading text can be regenerated with `pdftotext -layout ORIGINAL.pdf quarantine/sources/survey-durable/text/ID.txt`. The saved original is authoritative for citation. Living pages can change behind the same URL; a future URL fetch is a new revision unless its hash matches.

## ARIES auxiliary original

The initial scan is retained intact, separately from the searchable original used in the reading report.

- Source: https://people.eecs.berkeley.edu/~brewer/cs262/Aries2.pdf
- Authors: C. Mohan; Don Haderle; Bruce Lindsay; Hamid Pirahesh; Peter Schwarz
- Version: 1992 TODS paper, image-only two-page scan (35 PDF pages); not used for text reading
- Rights: ACM notice retained in original
- Original path: `quarantine/archives/survey-durable/documents/aries.pdf`
- SHA-256: `1007fcedb6b8b465f97ee1c4e4d88420450b50b18ff5ad400a130b21e4961817`

```sh
mkdir -p quarantine/archives/survey-durable/documents
curl -fL 'https://people.eecs.berkeley.edu/~brewer/cs262/Aries2.pdf' -o quarantine/archives/survey-durable/documents/aries.pdf
printf '%s  %s\n' '1007fcedb6b8b465f97ee1c4e4d88420450b50b18ff5ad400a130b21e4961817' 'quarantine/archives/survey-durable/documents/aries.pdf' | shasum -a 256 -c -
```

## Exact literature restoration commands

Originals below are identified by acquired bytes. Living endpoints and unversioned preprint URLs can change; a mismatch is a new snapshot, not the same acquisition. Existing first-pass originals use their linked manifest to avoid duplicating retained files.

### aether

Authors: Ryan Johnson; Ippokratis Pandis; Radu Stoica; Manos Athanassoulis; Anastasia Ailamaki. Revision: 2010; PVLDB 3(1-2). Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.vldb.org/pvldb/vol3/R61.pdf' -o 'quarantine/archives/survey-durable/documents/aether.pdf'
printf '%s  %s\n' '9c0b5aa69388abd7b381edb969334e1deb646240e19c54e1983a250db8b05c5e' 'quarantine/archives/survey-durable/documents/aether.pdf' | shasum -a 256 -c -
```

### arcadia

Authors: Shashank Gugnani; Scott Guthridge; Frank Schmuck; Owen Anderson; Deepavali Bhagwat; Xiaoyi Lu. Revision: 2022; arXiv:2206.12495. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://arxiv.org/pdf/2206.12495' -o 'quarantine/archives/survey-durable/documents/arcadia.pdf'
printf '%s  %s\n' '4ceec5d5db95430e0f0b12b73054f9b3d65ea55b04a36810c57e188c2f6f0241' 'quarantine/archives/survey-durable/documents/arcadia.pdf' | shasum -a 256 -c -
```

### aries

Authors: C. Mohan; Don Haderle; Bruce Lindsay; Hamid Pirahesh; Peter Schwarz. Revision: 1992; ACM TODS 17(1). Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://people.eecs.berkeley.edu/~brewer/cs262/Aries.pdf' -o 'quarantine/archives/survey-durable/documents/aries-readable.pdf'
printf '%s  %s\n' 'a1a659b0ccdce57af80386d336bb8d0bcae666ea8fc733924844c04963e48b56' 'quarantine/archives/survey-durable/documents/aries-readable.pdf' | shasum -a 256 -c -
```

### bookkeeper-ledger

Authors: Apache BookKeeper contributors. Revision: 4.11.1; Project API specification. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://bookkeeper.apache.org/docs/4.11.1/api/ledger-api/' -o 'quarantine/archives/survey-durable/documents/bookkeeper-ledger.html'
printf '%s  %s\n' '47fd68543dbed062d7156ee4bbc8b6aabd4e40b0da3bc02f1ced42919f624bd5' 'quarantine/archives/survey-durable/documents/bookkeeper-ledger.html' | shasum -a 256 -c -
```

### bookkeeper-protocol

Authors: Apache BookKeeper contributors. Revision: living; Project protocol specification. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://bookkeeper.apache.org/docs/development/protocol/' -o 'quarantine/archives/survey-durable/documents/bookkeeper-protocol.html'
printf '%s  %s\n' 'f1393fcab9d322a54fdf26f651ec4aabed11697daf230c1e3f695d6badfe626f' 'quarantine/archives/survey-durable/documents/bookkeeper-protocol.html' | shasum -a 256 -c -
```

### btrlog

Authors: Maximilian Kuschewski; Lam-Duy Nguyen; Matthias Jasny; Tobias Ziegler; Viktor Leis; Muhammad El-Hindi. Revision: 2026; arXiv:2606.27051. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://arxiv.org/pdf/2606.27051' -o 'quarantine/archives/survey-durable/documents/btrlog.pdf'
printf '%s  %s\n' 'd45936ecd478bac70c23d8f6c76ed58b5eeeb250dfd9ff7ad10ee25dab057e24' 'quarantine/archives/survey-durable/documents/btrlog.pdf' | shasum -a 256 -c -
```

### chain-replication

Authors: Robbert van Renesse; Fred B. Schneider. Revision: 2004; OSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/event/osdi04/tech/full_papers/renesse/renesse.pdf' -o 'quarantine/archives/survey-durable/documents/chain-replication.pdf'
printf '%s  %s\n' 'af379b56d4098009ae1f2f19a7535754f2afd09af0077760d1b5a18a40328dc6' 'quarantine/archives/survey-durable/documents/chain-replication.pdf' | shasum -a 256 -c -
```

### corfu

Authors: Mahesh Balakrishnan; Dahlia Malkhi; Vijayan Prabhakaran; Ted Wobber; Michael Wei; John D. Davis. Revision: 2012; NSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/conference/nsdi12/nsdi12-final30.pdf' -o 'quarantine/archives/survey-durable/documents/corfu.pdf'
printf '%s  %s\n' '46177642ac6d636b68f03fe8e33d7ce135dc4b7736f17fd46dd375676abb8d09' 'quarantine/archives/survey-durable/documents/corfu.pdf' | shasum -a 256 -c -
```

### corruption

Authors: Lakshmi N. Bairavasundaram; Garth R. Goodson; Bianca Schroeder; Andrea C. Arpaci-Dusseau; Remzi H. Arpaci-Dusseau. Revision: 2008; FAST. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/legacy/event/fast08/tech/full_papers/bairavasundaram/bairavasundaram.pdf' -o 'quarantine/archives/survey-durable/documents/corruption.pdf'
printf '%s  %s\n' '9d20b444842fdee705aadf43c6e92cdb066792a6561ba3aa3aef5a5c1c52fd8b' 'quarantine/archives/survey-durable/documents/corruption.pdf' | shasum -a 256 -c -
```

### craq

Authors: Jeff Terrace; Michael J. Freedman. Revision: 2009; USENIX ATC. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/event/usenix09/tech/full_papers/terrace/terrace.pdf' -o 'quarantine/archives/survey-durable/documents/craq.pdf'
printf '%s  %s\n' '43b0c05d694d602dbf6e085d8a3e91a4aa7fc1d7a55945b3bc97c68203d37df6' 'quarantine/archives/survey-durable/documents/craq.pdf' | shasum -a 256 -c -
```

### crashmonkey

Authors: Jayashree Mohan; Ashlie Martinez; Soujanya Ponnapalli; Pandian Raju; Vijay Chidambaram. Revision: 2018; OSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/osdi18-mohan.pdf' -o 'quarantine/archives/survey-durable/documents/crashmonkey.pdf'
printf '%s  %s\n' '9c40ba65270483ab7802ffc32acb1860b1f9f19d541309ea721415def2d46128' 'quarantine/archives/survey-durable/documents/crashmonkey.pdf' | shasum -a 256 -c -
```

### delos

Authors: Mahesh Balakrishnan; Jason Flinn; Chen Shen; Mihir Dharamshi; Ahmed Jafri; Xiao Shi; Santosh Ghosh; Hazem Hassan; Aaryaman Sagar; Rhed Shi; Jingming Liu; Filip Gruszczynski; Xianan Zhang; Huy Hoang; Ahmed Yossef; Francois Richard; Yee Jiun Song. Revision: 2020; OSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/osdi20-balakrishnan.pdf' -o 'quarantine/archives/survey-durable/documents/delos.pdf'
printf '%s  %s\n' 'c635433a8d49ba419e16298f3bbb636ad9fc07fc172e957d7ce455209fc144c8' 'quarantine/archives/survey-durable/documents/delos.pdf' | shasum -a 256 -c -
```

### disk-failure

Authors: Bianca Schroeder; Garth A. Gibson. Revision: 2007; FAST. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://static.usenix.org/events/fast07/tech/schroeder/schroeder.pdf' -o 'quarantine/archives/survey-durable/documents/disk-failure.pdf'
printf '%s  %s\n' '237868378cdc09e720f6cf29926d2cabee0e08751d00213e8bdc604723553bc6' 'quarantine/archives/survey-durable/documents/disk-failure.pdf' | shasum -a 256 -c -
```

### end-to-end

Authors: J. H. Saltzer; D. P. Reed; D. D. Clark. Revision: 1984; ACM TOCS 2(4). Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://web.mit.edu/Saltzer/www/publications/endtoend/endtoend.pdf' -o 'quarantine/archives/survey-durable/documents/end-to-end.pdf'
printf '%s  %s\n' '3204a1a562d9d3b72ba112f20caddfd34991d6e1e39939ace6a3e7c016519768' 'quarantine/archives/survey-durable/documents/end-to-end.pdf' | shasum -a 256 -c -
```

### failureminer

Authors: Shuyang Wang; Yuqi Zhang; Haonan Luo; Kangkang Liu; Gil Kim; JongSung Na; Claude Kim; Geunrok Oh; Kyle Choi; Ni Xue; Xing He. Revision: 2026; FAST. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/fast26-wang-shuyang.pdf' -o 'quarantine/archives/survey-durable/documents/failureminer.pdf'
printf '%s  %s\n' '6f1d513098b7b63c07124fb59f0cb2efc12d28e345a2ac5e1bfc64d62169f31a' 'quarantine/archives/survey-durable/documents/failureminer.pdf' | shasum -a 256 -c -
```

### fuzzylog

Authors: Joshua Lockerman; Jose M. Faleiro; Juno Kim; Soham Sankaran; Daniel J. Abadi; James Aspnes; Siddhartha Sen; Mahesh Balakrishnan. Revision: 2018; OSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/osdi18-lockerman.pdf' -o 'quarantine/archives/survey-durable/documents/fuzzylog.pdf'
printf '%s  %s\n' '5988f2f0ec597a1cd5bb8b44862f0bd2f95689c3328cdde20ffa224adbfc4c5e' 'quarantine/archives/survey-durable/documents/fuzzylog.pdf' | shasum -a 256 -c -
```

### kafka-2011

Authors: Jay Kreps; Neha Narkhede; Jun Rao. Revision: 2011; NetDB. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.microsoft.com/en-us/research/wp-content/uploads/2017/09/Kafka.pdf' -o 'quarantine/archives/survey-durable/documents/kafka-2011.pdf'
printf '%s  %s\n' '4abdeba2503eb20a5d7ed84aa8e7680bcbe3088541712626315deae0b07c2821' 'quarantine/archives/survey-durable/documents/kafka-2011.pdf' | shasum -a 256 -c -
```

### kafka-durability

Authors: Apache Kafka contributors. Revision: 4.1; Versioned project documentation. Rights: Project documentation; original copyright/terms retained. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://kafka.apache.org/41/operations/hardware-and-os/' -o 'quarantine/archives/survey-durable/documents/kafka-durability.html'
printf '%s  %s\n' '169a680ebf8ef9e41f2774134111724ffd30517e955b0b695e4b1ccf15a34096' 'quarantine/archives/survey-durable/documents/kafka-durability.html' | shasum -a 256 -c -
```

### lazylog

Authors: Xuhao Luo; Shreesha G. Bhat; Jiyu Hu; Ramnatthan Alagappan; Aishwarya Ganesan. Revision: 2024; SOSP author copy. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://dassl-uiuc.github.io/pdfs/papers/lazylog.pdf' -o 'quarantine/archives/survey-durable/documents/lazylog.pdf'
printf '%s  %s\n' '1a8ea38ce3fa3deaba9cf6f3674e239e5e20c6c8d94be82a8c776584b132af98' 'quarantine/archives/survey-durable/documents/lazylog.pdf' | shasum -a 256 -c -
```

### lfs

Authors: Mendel Rosenblum; John K. Ousterhout. Revision: 1991; Author preprint dated 1991-07-24. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://people.eecs.berkeley.edu/~brewer/cs262/LFS.pdf' -o 'quarantine/archives/survey-durable/documents/lfs.pdf'
printf '%s  %s\n' 'cc6a7d7e52cd9182368f42024188bfbcf899f3fc47ce8b87c33a1aabffa286b7' 'quarantine/archives/survey-durable/documents/lfs.pdf' | shasum -a 256 -c -
```

### logdevice-architecture

Authors: LogDevice authors. Revision: living; Project documentation. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://logdevice.io/docs/Concepts.html' -o 'quarantine/archives/survey-durable/documents/logdevice-architecture.html'
printf '%s  %s\n' '3c447fc97719d72434d9a250b219a599f563953ce6507ddb2cd5af113ad4b30d' 'quarantine/archives/survey-durable/documents/logdevice-architecture.html' | shasum -a 256 -c -
```

### logdrive

Authors: Gardner Vickers; Lucas Bradstreet; Mahesh Balakrishnan; Prince Mahajan; David Mao; Xavier Léauté; Ismael Juma; Nikhil Bhatia; Jack Vanlightly; Prateek Jindal; Sumit Arrawatia; Andrew Grant; Dhruvil Shah; Dimitar Dimitrov; Gaurav Badoni; Shimiao Zhang; Yang Yu. Revision: 2026; OSDI. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/osdi26-vickers.pdf' -o 'quarantine/archives/survey-durable/documents/logdrive.pdf'
printf '%s  %s\n' '650bafa7338cbd92228184f8fd248fffd61bb78534a66d40149e36b6a54bcfa9' 'quarantine/archives/survey-durable/documents/logdrive.pdf' | shasum -a 256 -c -
```

### nova

Authors: Jian Xu; Steven Swanson. Revision: 2016; FAST. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/conference/fast16/fast16-papers-xu.pdf' -o 'quarantine/archives/survey-durable/documents/nova.pdf'
printf '%s  %s\n' '5c320273da2f40f959aba2503bf146d58713e416ff773f9d986d5d8323c0a320' 'quarantine/archives/survey-durable/documents/nova.pdf' | shasum -a 256 -c -
```

### nvme-field

Authors: Ruiming Lu; Erci Xu; Yiming Zhang; Zhaosheng Zhu; Mengtian Wang; Zongpeng Zhu; Guangtao Xue; Minglu Li; Jiesheng Wu. Revision: 2022; USENIX ATC. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/atc22-lu.pdf' -o 'quarantine/archives/survey-durable/documents/nvme-field.pdf'
printf '%s  %s\n' 'ebf545a78830a8e316965cdf7145fe9cdbfc5bafca51e67c68c978ff4c90549f' 'quarantine/archives/survey-durable/documents/nvme-field.pdf' | shasum -a 256 -c -
```

### palf

Authors: Fusheng Han; Hao Liu; Bin Chen; Debin Jia; Jianfeng Zhou; Xuwang Teng; Chuanhui Yang; Huafeng Xi; Wei Tian; Shuning Tao; Sen Wang; Quanqing Xu; Zhenkun Yang. Revision: 2024; PVLDB 17(12). Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.vldb.org/pvldb/vol17/p3745-xu.pdf' -o 'quarantine/archives/survey-durable/documents/palf.pdf'
printf '%s  %s\n' 'f09909ce10bf2bab81899e0ec09461f42b24f5bff41953c9d570b66948de48b2' 'quarantine/archives/survey-durable/documents/palf.pdf' | shasum -a 256 -c -
```

### paxos-simple

Authors: Leslie Lamport. Revision: 2001; SIGACT News 32(4). Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://lamport.azurewebsites.net/pubs/paxos-simple.pdf' -o 'quarantine/archives/survey-durable/documents/paxos-simple.pdf'
printf '%s  %s\n' 'c83b5141257730d6fd20097ef17ebd02a02dfcb1809bad5873b7739659bb3f2b' 'quarantine/archives/survey-durable/documents/paxos-simple.pdf' | shasum -a 256 -c -
```

### pmdk-libpmem

Authors: PMDK authors. Revision: v1; Project API manual. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://pmem.io/pmdk/manpages/linux/v1/libpmem.3/' -o 'quarantine/archives/survey-durable/documents/pmdk-libpmem.html'
printf '%s  %s\n' 'b46e5d4b9fd67841345e68bed7eac33416303ee59de5838352a66c2652021c40' 'quarantine/archives/survey-durable/documents/pmdk-libpmem.html' | shasum -a 256 -c -
```

### pmem-io

Authors: Alexander van Renen; Lukas Vogel; Viktor Leis; Thomas Neumann; Alfons Kemper. Revision: 2019; arXiv:1904.01614. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://arxiv.org/pdf/1904.01614' -o 'quarantine/archives/survey-durable/documents/pmem-io.pdf'
printf '%s  %s\n' '54366a84b3c8758edbe312fce7aa4f1748de652a97cc7bc329499e242c622bd8' 'quarantine/archives/survey-durable/documents/pmem-io.pdf' | shasum -a 256 -c -
```

### poplar

Authors: Huan Zhou; Jinwei Guo; Huiqi Hu; Weining Qian; Xuan Zhou; Aoying Zhou. Revision: 2019; arXiv:1901.06491. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://arxiv.org/pdf/1901.06491' -o 'quarantine/archives/survey-durable/documents/poplar.pdf'
printf '%s  %s\n' '217559e1934b4c76e7af46a5ec7cbdeb0b1a541ffc13976bd36f51671f63fb59' 'quarantine/archives/survey-durable/documents/poplar.pdf' | shasum -a 256 -c -
```

### pravega-segment-store

Authors: Pravega Authors. Revision: 3142a01075bfae78d66588e46ff87e6e10225744; Pinned project architecture documentation. Rights: Apache-2.0 notice in original. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://raw.githubusercontent.com/pravega/pravega/3142a01075bfae78d66588e46ff87e6e10225744/documentation/src/docs/segment-store-service.md' -o 'quarantine/archives/survey-durable/documents/pravega-segment-store.md'
printf '%s  %s\n' 'b124009d2f9ced7fdcefdda37afa51b8369471a832b93fb5cec327242fbdde88' 'quarantine/archives/survey-durable/documents/pravega-segment-store.md' | shasum -a 256 -c -
```

### raft

Authors: Diego Ongaro; John Ousterhout. Revision: 2014; USENIX ATC; extended author version. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://raft.github.io/raft.pdf' -o 'quarantine/archives/survey-durable/documents/raft.pdf'
printf '%s  %s\n' '5a5c679b6a7c88007faeff8c3eef19a77e9e5caa8bd7390922584fe8ecf0ce1f' 'quarantine/archives/survey-durable/documents/raft.pdf' | shasum -a 256 -c -
```

### redpanda-write-caching

Authors: Redpanda authors. Revision: living; Project documentation. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://docs.redpanda.com/streaming/current/develop/manage-topics/config-topics/' -o 'quarantine/archives/survey-durable/documents/redpanda-write-caching.html'
printf '%s  %s\n' 'da64581764bb69f1b6622ed1649341dbbd8297b3923143f8ef8691a5dc0e67fa' 'quarantine/archives/survey-durable/documents/redpanda-write-caching.html' | shasum -a 256 -c -
```

### rifl

Authors: Collin Lee; Seo Jin Park; Ankita Kejriwal; Satoshi Matsushita; John Ousterhout. Revision: 2015; SOSP. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://web.stanford.edu/~ouster/cgi-bin/papers/rifl.pdf' -o 'quarantine/archives/survey-durable/documents/rifl.pdf'
printf '%s  %s\n' 'f609f9508beaf936027f31428a8f06dc86d8201faadc5b340ffead7706863969' 'quarantine/archives/survey-durable/documents/rifl.pdf' | shasum -a 256 -c -
```

### rl-watchdog

Authors: Jin Yong Ha; Sangjin Lee; Heon Young Yeom; Yongseok Son. Revision: 2024; USENIX ATC. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/atc24-ha.pdf' -o 'quarantine/archives/survey-durable/documents/rl-watchdog.pdf'
printf '%s  %s\n' '4f67f329603576d74ebe0bb3f9453b57af3b8c3c7055a39a83f87d2427b2401b' 'quarantine/archives/survey-durable/documents/rl-watchdog.pdf' | shasum -a 256 -c -
```

### scalog

Authors: Cong Ding; David Chu; Evan Zhao; Xiang Li; Lorenzo Alvisi; Robbert van Renesse. Revision: 2020; NSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/nsdi20-paper-ding.pdf' -o 'quarantine/archives/survey-durable/documents/scalog.pdf'
printf '%s  %s\n' '45e29724e4691984bf2fd1a2d9b7a7a7f076612313c448a4fc3ca83efadc6a46' 'quarantine/archives/survey-durable/documents/scalog.pdf' | shasum -a 256 -c -
```

### silor

Authors: Wenting Zheng; Stephen Tu; Eddie Kohler; Barbara Liskov. Revision: 2014; OSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/conference/osdi14/osdi14-paper-zheng_wenting.pdf' -o 'quarantine/archives/survey-durable/documents/silor.pdf'
printf '%s  %s\n' 'd88cfc330371336d25ada52180af39aafd892c6c5cf8c8f61fccd1f2b2db9cdb' 'quarantine/archives/survey-durable/documents/silor.pdf' | shasum -a 256 -c -
```

### speclog

Authors: Shreesha G. Bhat; Tony Hong; Xuhao Luo; Jiyu Hu; Aishwarya Ganesan; Ramnatthan Alagappan. Revision: 2025; OSDI. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/osdi25-bhat.pdf' -o 'quarantine/archives/survey-durable/documents/speclog.pdf'
printf '%s  %s\n' '2e8eecd160e595a5d817f983d8eb1384a72487aee2b66eff5820a4a36557f5ed' 'quarantine/archives/survey-durable/documents/speclog.pdf' | shasum -a 256 -c -
```

### ssd-power-fault

Authors: Mai Zheng; Joseph Tucek; Feng Qin; Mark Lillibridge. Revision: 2013; FAST. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/conference/fast13/fast13-final80.pdf' -o 'quarantine/archives/survey-durable/documents/ssd-power-fault.pdf'
printf '%s  %s\n' '42174236846ef0231755a92d7141c79395a6b9543598c827882e72f8220fa52f' 'quarantine/archives/survey-durable/documents/ssd-power-fault.pdf' | shasum -a 256 -c -
```

### tango

Authors: Mahesh Balakrishnan; Dahlia Malkhi; Ted Wobber; Ming Wu; Vijayan Prabhakaran; Michael Wei; John D. Davis; Sriram Rao; Tao Zou; Aviad Zuck. Revision: 2013; SOSP. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.microsoft.com/en-us/research/wp-content/uploads/2013/11/Tango.pdf' -o 'quarantine/archives/survey-durable/documents/tango.pdf'
printf '%s  %s\n' 'e80852388b50cbae7800e082707e156e37df8753e54a3138c6a66655c912ccc9' 'quarantine/archives/survey-durable/documents/tango.pdf' | shasum -a 256 -c -
```

### taurus

Authors: Yu Xia; Xiangyao Yu; Andrew Pavlo; Srinivas Devadas. Revision: 2020; PVLDB 14(2). Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.vldb.org/pvldb/vol14/p189-xia.pdf' -o 'quarantine/archives/survey-durable/documents/taurus.pdf'
printf '%s  %s\n' '6d95dea37172eff05b0afb64ac6c74c22f67189315265a4363b94a6155249ed0' 'quarantine/archives/survey-durable/documents/taurus.pdf' | shasum -a 256 -c -
```

### vcorfu

Authors: Michael Wei; Amy Tai; Christopher J. Rossbach; Ittai Abraham; Maithem Munshed; Medhavi Dhawan; Udi Wieder; Scott Fritchie; Steven Swanson; Michael J. Freedman; Dahlia Malkhi. Revision: 2017; NSDI. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.cs.princeton.edu/~mfreed/docs/vcorfu-nsdi17.pdf' -o 'quarantine/archives/survey-durable/documents/vcorfu.pdf'
printf '%s  %s\n' 'dbfc1d04491b179286459068c350a44f153ad384e0e9e0760b680ba6d3d87a83' 'quarantine/archives/survey-durable/documents/vcorfu.pdf' | shasum -a 256 -c -
```

### xll

Authors: John Shawger; Arnav Jhingran; Andrea Arpaci-Dusseau; Remzi Arpaci-Dusseau. Revision: 2026; NSDI. Rights: Primary paper original; authors/publisher rights retained; no reuse license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.usenix.org/system/files/nsdi26-shawger.pdf' -o 'quarantine/archives/survey-durable/documents/xll.pdf'
printf '%s  %s\n' '2ebf411f615a7bdf7fd5ae963678d993878636eee0abd9e5828d738a1bd525f0' 'quarantine/archives/survey-durable/documents/xll.pdf' | shasum -a 256 -c -
```

### zab

Authors: Flavio P. Junqueira; Benjamin C. Reed; Marco Serafini. Revision: 2011; DSN. Rights: Original research/documentation; copyright and reuse terms retained in original; no software license inferred. See the catalogue for precise edition/reading notes.

```sh
mkdir -p 'quarantine/archives/survey-durable/documents'
curl -fL 'https://www.cs.cornell.edu/courses/cs6452/2012sp/papers/zab-ieee.pdf' -o 'quarantine/archives/survey-durable/documents/zab.pdf'
printf '%s  %s\n' '9ebaf5a803fd9e03512c8d2c7edd0e8f04e829d4dbf4334b346b5a66961a6db4' 'quarantine/archives/survey-durable/documents/zab.pdf' | shasum -a 256 -c -
```

## Existing first-pass acquisitions

SQLite atomic commit/WAL, the Pillai paper, and the LevelDB format/tree remain at their original paths. Restore using [papers/recon/external-manifest.md](../../recon/external-manifest.md); the bibliography records exact original-file hashes and the pinned LevelDB revision. The earlier redb source and documentation are additional linked comparison material; no second copy was ingested.

## Licensing checks

Root license originals were inspected for all eight repositories. Kafka, BookKeeper, Pravega, etcd Raft, and CorfuDB use Apache-2.0 notices in the inspected roots. LogDevice and PMDK retain BSD-style three-clause notices. Redpanda has BSL/RCL and third-party licensing material under `licenses/`; individual-file headers govern. This is a private reading corpus, not an adoption or redistribution decision.
