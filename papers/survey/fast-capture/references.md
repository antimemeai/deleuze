# Pinned reference implementation manifest

Acquired 2026-10-23 UTC. These eight references comprise 16,940 extracted ordinary files and 73,114,919 compressed archive bytes. No reference was built, benchmarked, installed, or adopted as a dependency. The archive hashes identify originals; per-file inventories identify the stripped working copies.

| Reference | Source revision | License location and scope | Inventory |
| --- | --- | --- | --- |
| [nanolog](https://github.com/PlatformLab/NanoLog) | `2a94d70f9d1db4da416053b1b926387fa068a59b` | ISC; Stanford University (`LICENSE`) | [80 files](ref-nanolog-files.tsv) |
| [lttng-ust](https://github.com/lttng/lttng-ust) | `5d11feb7dbd25862da163880f89acb6a6d88754c` | LGPL-2.1-only tracer; MIT public headers/filter runtime; GPL-2.0-only UST control; other per-file notices (`LICENSE`) | [708 files](ref-lttng-ust-files.tsv) |
| [perfetto](https://github.com/google/perfetto) | `a04c38a0d3d4ec4a13f5b0453846ac8e49765b31` | Apache-2.0; included third-party notices remain applicable (`LICENSE`) | [13000 files](ref-perfetto-files.tsv) |
| [rr](https://github.com/rr-debugger/rr) | `c145f2f80e63e9627b79deed6e0d3eae0d658e38` | MIT main project; separate notices for incorporated files and GPL-2.0 third-party workaround (`LICENSE`) | [1670 files](ref-rr-files.tsv) |
| [hindsight](https://gitlab.mpi-sws.org/cld/tracing/hindsight) | `c534f2c752793c491768b51ae3db94b4780a7f8f` | BSD-3-Clause text; Max Planck Institute for Software Systems (`LICENSE`) | [93 files](ref-hindsight-files.tsv) |
| [pivot-tracing](https://github.com/brownsys/tracing-framework) | `016027d8dbba2795f1903fa80b5ab8073919ed83` | BSD-3-Clause text; Brown University (`LICENSE`) | [610 files](ref-pivot-tracing-files.tsv) |
| [liburing](https://github.com/axboe/liburing) | `f33bc2f24cf3b38df8553aab5f9714fc5cf0aff4` | LGPL-2.1 or MIT generally; exported Linux headers have syscall-note/GPL or MIT choices; see README (`LICENSE;COPYING.GPL;COPYING`) | [526 files](ref-liburing-files.tsv) |
| [linux-tracing](https://github.com/torvalds/linux) | `adc218676eef25575469234709c2d87185ca223a` | GPL-2.0 family and per-file SPDX notices; root COPYING and LICENSES preserve syscall-note context (`COPYING;LICENSES/`) | [253 files](ref-linux-tracing-files.tsv) |

[references.tsv](references.tsv) records origin URL, archive URL, revision, acquisition time, original archive SHA-256, original path, extraction path, and license file locations. The revisions were resolved from each remote HEAD at acquisition, except Linux, pinned deliberately to tag `v6.12` and its peeled commit. A current source revision is not necessarily the version evaluated in its historical paper.

## Archives and restoration

Archives live in `quarantine/archives/survey-fast-capture/references/NAME.tar.gz`; extracted copies live in `quarantine/sources/survey-fast-capture/references/NAME/`. Full upstream download archives remain intact. Only ordinary files enter the extracted reading corpus; nested Git metadata, `.DS_Store`, cache/build-environment detritus, and symlinks are omitted. Each retained file is listed in its TSV.

For seven repositories, the immutable revision archive URL in the TSV can be downloaded again. A changed compressed archive representation need not imply changed Git content; verify the source file inventory as well. Exact original compressed bytes require the retained archive.

The following standard-library recipe restores one missing extraction from its retained original. Run it from the Deleuze root after setting `name` to one row ID. It verifies the archive hash, validates the path set and every file hash, and refuses an existing extraction. Linux archives have no upstream root prefix; the others have one.

```python
import csv, hashlib, tarfile
from pathlib import Path
name = "nanolog"
base = Path("papers/survey/fast-capture")
with (base / "references.tsv").open(newline="") as f:
    meta = next(r for r in csv.DictReader(f, delimiter="\t") if r["id"] == name)
with (base / f"ref-{name}-files.tsv").open(newline="") as f:
    expected = {r["path"]: r for r in csv.DictReader(f, delimiter="\t")}
archive = Path(meta["localOriginal"])
assert hashlib.sha256(archive.read_bytes()).hexdigest() == meta["sha256"]
destination = Path(meta["localExtraction"])
assert not destination.exists(), "refusing existing extraction"
contents = {}
with tarfile.open(archive) as source:
    for member in source:
        parts = Path(member.name).parts
        parts = parts if name == "linux-tracing" else parts[1:]
        if not parts or not member.isfile():
            continue
        rel = Path(*parts)
        if str(rel) not in expected:
            continue
        assert not rel.is_absolute() and ".." not in rel.parts
        with source.extractfile(member) as stream:
            data = stream.read()
        row = expected[str(rel)]
        assert len(data) == int(row["bytes"])
        assert hashlib.sha256(data).hexdigest() == row["sha256"]
        assert str(rel) not in contents
        contents[str(rel)] = (data, member.mode & 0o777)
assert set(contents) == set(expected)
for rel, (data, mode) in contents.items():
    output = destination / rel
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_bytes(data)
    output.chmod(mode)
```

## Linux source selection

Linux was acquired in a temporary filtered shallow checkout of `v6.12`, then exported from commit `adc218676eef25575469234709c2d87185ca223a` with `git archive`. The retained original is a selected tracked-source export rather than a full Linux tarball. The selected path roots were:

```text
COPYING
LICENSES/
kernel/trace/
kernel/events/
kernel/relay.c
kernel/bpf/ringbuf.c
include/linux/ring_buffer.h
include/linux/tracepoint.h
include/linux/relay.h
include/linux/perf_event.h
include/uapi/linux/perf_event.h
Documentation/trace/
Documentation/filesystems/relay.rst
Documentation/userspace-api/perf_ring_buffer.rst
Documentation/bpf/ringbuf.rst
io_uring/
```

To recreate this selected source archive from a fresh checkout of the recorded commit, pass these roots to `git archive --format=tar.gz --output=linux-tracing.tar.gz COMMIT PATH...`; compare the extracted files with the tracked inventory. The generated compression wrapper may differ. No installed or running kernel was modified.

## Original archive SHA-256 values

- `nanolog`: `b40fe916530671f1b450e75a75a53657cbe060f369916ec435c743cf020dd5ef`
- `lttng-ust`: `c2d168abc8d8a2de917006ddc8c253ece97e03d5c77fd03a0d1966902bafd10e`
- `perfetto`: `9b150a2367d46d800004147617d6df1fb80891bbd07fb2f9d2778d99f78271e1`
- `rr`: `aa68652ce6cd6eb099a54870c7bc0014ffb42144d3de4fb492cbe4f13fa4a937`
- `hindsight`: `713ed8ab96f47393058e82d56b691ab9adab7d0f5316ca6894e069a4db9ed6c3`
- `pivot-tracing`: `c629b09ddb27fcb521cb73b29a7ba035f097e2219af16f7bb9afd9014cd28aef`
- `liburing`: `c780ef632d70516b559d69db9bcafedd3dd3f778c1fd718db2722b3de04489e7`
- `linux-tracing`: `e653e896da169f14666c4f8a6d77a4ed65ca4ecdbf44e76c7b7ae9053e99d1a0`
