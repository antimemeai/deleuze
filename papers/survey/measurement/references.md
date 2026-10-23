# Pinned measurement references and restoration

These are quarantined study references, not dependencies. Captured public upstream HEADs on 2026-10-23; source-to-paper release correspondence is not established unless specified. Full archive bytes remain untouched. Extracted ordinary files omit nested version-control history, generated dependency/cache directories, symlinks and special files. The original archives retain those members and all embedded third-party sources.

[references.tsv](references.tsv) is the tracked authority for origin URL, exact revision, archive URL/path/hash, working extraction path, file count, license paths and acquisition time. [bibliography.tsv](bibliography.tsv) records literature originals separately, including reading depth; [auxiliary-originals.tsv](auxiliary-originals.tsv) records the two additional retained author/publisher HTML pages.

| Reference | Revision | Ordinary extracted files | License reading | Selected paths inspected |
|---|---|---:|---|---|
| [wrk2](https://github.com/giltene/wrk2) | `44a94c17d8e6a0bac8559b53da76848e430cb7a7` | 239 | Apache-2.0 main; `NOTICE` enumerates embedded components | `src/wrk.c`, `src/wrk.h`, README/NOTICE |
| [HdrHistogram Java](https://github.com/HdrHistogram/HdrHistogram) | `de84b0a7de2378abfc405da503bf4898e84ea98e` | 104 | CC0/public domain; alternative BSD-2-Clause in `LICENSE.txt` | `src/main/java/org/HdrHistogram/AbstractHistogram.java`, README/LICENSE |
| [TailBench++](https://github.com/zliUPV/Tailbenchplusplus) | `1a707726ddd171ebf7e2bd3db52f5f5bbe4a9c7c` | 3,962 | MIT default; bundled masstree, silo, shore, moses and others retain their licenses | `harness/client.cpp`, `tbench_client_networked.cpp`, `msgs.h`, root README/LICENSE |

The original TailBench++ codeload archive is approximately 145 MB and contains nested Mercurial repositories. No embedded source project is presented as code authored for Deleuze. Per-file manifests enumerate exactly the stripped working snapshots: [wrk2](ref-wrk2-files.tsv), [HdrHistogram](ref-hdrhistogram-files.tsv), [TailBench++](ref-tailbench-plus-files.tsv).

The following Python standard-library recipe restores a working reference into a **new destination**, verifies the archive hash, omits the documented metadata/symlink categories, and verifies every extracted byte against the tracked inventory. Run from the repository root, changing `reference` and `destination`. It deliberately refuses an existing destination. It does not fetch moving HEADs or run any acquired code.

```python
from pathlib import Path
import csv, hashlib, shutil, tarfile, urllib.request

reference = 'wrk2'  # also hdrhistogram, tailbench-plus
destination = Path('/tmp/deleuze-measurement-restored-wrk2')
manifest_dir = Path('papers/survey/measurement')
rows = list(csv.DictReader((manifest_dir / 'references.tsv').open(), delimiter='\t'))
row = next(r for r in rows if r['id'] == reference)
archive = Path(row['localOriginal'])
if not archive.exists():
    archive.parent.mkdir(parents=True, exist_ok=True)
    with urllib.request.urlopen(row['archiveURL']) as source, archive.open('xb') as output:
        shutil.copyfileobj(source, output)
assert hashlib.sha256(archive.read_bytes()).hexdigest() == row['sha256']
assert not destination.exists(), 'refusing to overwrite destination'
skip = {'.git', '.hg', '.svn', '.DS_Store', '__pycache__', 'node_modules',
        'target', '.venv', '.deps', '.libs', 'autom4te.cache'}
destination.mkdir(parents=True)
with tarfile.open(archive) as source:
    for member in source:
        parts = Path(member.name).parts[1:]  # codeload's one top-level directory
        if not parts or any(p in skip for p in parts):
            continue
        assert '..' not in parts
        relative = Path(*parts)
        assert not relative.is_absolute()
        target = destination / relative
        if member.isdir():
            target.mkdir(parents=True, exist_ok=True)
        elif member.isfile():
            target.parent.mkdir(parents=True, exist_ok=True)
            with source.extractfile(member) as data, target.open('xb') as output:
                shutil.copyfileobj(data, output)
            target.chmod(member.mode & 0o777)
        # Links and special members remain only in the original archive.
items = list(csv.DictReader(
    (manifest_dir / f'ref-{reference}-files.tsv').open(), delimiter='\t'))
expected = {i['path'] for i in items}
actual = {str(p.relative_to(destination)) for p in destination.rglob('*') if p.is_file()}
assert actual == expected
for item in items:
    data = (destination / item['path']).read_bytes()
    assert len(data) == int(item['bytes'])
    assert hashlib.sha256(data).hexdigest() == item['sha256']
```

Literature originals can be reacquired using `resolvedURL` in the bibliography (or the auxiliary `primaryURL`) and checked against the stored hash. Living pages may change; a changed response is not the captured original. The two README originals were fetched from public `master` URLs, then byte-compared with their pinned source snapshots; those matches are recorded in the bibliography. For a stable fetch, replace `master` in the URL with the matching pinned revision. Original paper mirrors and preprint/journal version differences are explicit in [frontiers.md](frontiers.md).

Validation completed: all 18 main originals and two auxiliary originals hashed; all three reference archive hashes checked; all 4,305 ordinary source members matched between the preserved archives, tracked inventories and stripped working copies. This validates acquisition/restoration bytes only. It does not validate the acquired code, benchmark claims or paper proofs.
