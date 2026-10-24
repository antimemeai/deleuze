# Adaptive batching control: public acquisition manifest

Acquired and verified 2026-10-23. Scope: two reference mechanisms, Linux Net DIM and Clipper batching. [Study and source-grounded conclusions](../survey/adaptive-batching/control-lane.md). Only public material is retained. No dependencies adopted, production files changed, reference code executed, or benchmarks run in this lane.

## Revisions and rights

| Package | Origin / primary source | Pin and scope | License / access |
| --- | --- | --- | --- |
| Linux Net DIM | [torvalds/linux](https://github.com/torvalds/linux), Tal Gilboa's [documentation](https://kernel.org/doc/html/v6.12/networking/net_dim.html) | v6.12; tag object `06090c9b622a7e1f797e775db4c035e0d779b76e`, peeled commit `adc218676eef25575469234709c2d87185ca223a`; selected exact raw files, not a whole buildable kernel | DIM source SPDX `GPL-2.0 OR Linux-OpenIB`; selected driver carries its license in its header. COPYING, GPL-2.0 and Linux-OpenIB texts retained. Other kernel files retain their own notices. Publicly accessible. |
| Clipper implementation | [ucbrise/clipper](https://github.com/ucbrise/clipper) | HEAD resolved to `9f25e3fc7f8edc891615e81c5b80d3d8aed72608`; original codeload archive and 421 ordinary extracted files | Root `LICENSE.txt`: Apache-2.0; bundled third-party notices retained, not relicensed. Publicly accessible. |
| Clipper paper | [NSDI 2017 publisher record](https://www.usenix.org/conference/nsdi17/technical-sessions/presentation/crankshaw) | Daniel Crankshaw, Xin Wang, Giulio Zhou, Michael J. Franklin, Joseph E. Gonzalez, Ion Stoica; *Clipper: A Low-Latency Online Prediction Serving System*, NSDI 17, pp.613–627, ISBN 978-1-931971-37-9. No DOI supplied by publisher record. | USENIX open-access original PDF retained; no general source-code license asserted for paper. |

The Clipper source pin is **not** asserted to be the paper's evaluated revision. Paper §4.3 describes default AIMD; this source implements regression plus stochastic exploration. The study keeps these accounts separate. Linux v6.12 is a selected reproducible revision; no latest-kernel coverage claim is made. Pins were independently resolved with `git ls-remote` against the public origins at acquisition.

## Original bytes

All paths in this table are relative to ignored `quarantine/sources/adaptive-batching/control/`. Raw Linux downloads and the PDF are unmodified originals. The source archive is intact; stripping applies only to extracted Clipper files. SHA256 is a reproducibility check, not a signature or publisher-authentication claim.

| Local original path | Bytes | SHA256 | Exact download URL |
| --- | ---: | --- | --- |
| `linux-v6.12/COPYING` | 496 | `fb5a425bd3b3cd6071a3a9aff9909a859e7c1158d54d32e07658398cd67eb6a0` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/COPYING |
| `linux-v6.12/LICENSES/preferred/GPL-2.0` | 18729 | `f6b78c087c3ebdf0f3c13415070dd480a3f35d8fc76f3d02180a407c1c812f79` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/LICENSES/preferred/GPL-2.0 |
| `linux-v6.12/Documentation/networking/net_dim.rst` | 9574 | `0d17bc4204d0826cd7a5cbc7b9ee99eba2e4067385d7ac0ec7daf6da6a1b5d34` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/Documentation/networking/net_dim.rst |
| `linux-v6.12/include/linux/dim.h` | 12971 | `9a0af2a01ef78c935105d2a5145038fa71fcfc016435b8533ec44312a23595da` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/include/linux/dim.h |
| `linux-v6.12/lib/dim/dim.c` | 2245 | `f735d676d4c7491066c35010b290b1e2f86e027a8bf15ff74061a27561eb9da6` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/lib/dim/dim.c |
| `linux-v6.12/lib/dim/net_dim.c` | 9444 | `41a274840766f7fa3cac823b0a7ef5b28ea7ef83dabe8286045342b4c5b4f728` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/lib/dim/net_dim.c |
| `linux-v6.12/lib/dim/Makefile` | 121 | `944adf102763d1f2cea86cbc9debed1cacad7f2dd2cb88163ca31648f16ca291` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/lib/dim/Makefile |
| `linux-v6.12/drivers/net/ethernet/mellanox/mlx5/core/en_dim.c` | 4069 | `705f014031f6d5d29b7c8ae198ddc748249ad44d2486e7f7b7da63022a7eea23` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/drivers/net/ethernet/mellanox/mlx5/core/en_dim.c |
| `originals/clipper-9f25e3fc.tar.gz` | 3051470 | `d7b47772583c6312677e2c86a70e583bbd941d64a8430b3eb3ced8bcd521d74e` | https://codeload.github.com/ucbrise/clipper/tar.gz/9f25e3fc7f8edc891615e81c5b80d3d8aed72608 |
| `literature/clipper-nsdi17.pdf` | 4170703 | `3575456d878346bb705a90bb0defbda84dc34b8f4b2cdbbf7a6bd3e570e4b132` | https://www.usenix.org/system/files/conference/nsdi17/nsdi17-crankshaw.pdf |
| `linux-v6.12/include/net/tcp.h` | 85652 | `cdf696125e9484f7a5579eb0fa0841acb92f39ba6cd2dab219670e837fcf1b8c` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/include/net/tcp.h |
| `linux-v6.12/net/ipv4/tcp_output.c` | 131725 | `81a3e778a6b698192745c4fc9c50c26245374ffd605b095d894659ca2cbc751a` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/net/ipv4/tcp_output.c |
| `linux-v6.12/LICENSES/deprecated/Linux-OpenIB` | 1317 | `8ae20d34c2a74c85ae2760dd0034baf18e44bee3289b9b3a4fe46768b4ea2cd6` | https://raw.githubusercontent.com/torvalds/linux/adc218676eef25575469234709c2d87185ca223a/LICENSES/deprecated/Linux-OpenIB |

The two TCP files are supplementary acquisitions requested by the storage/completion-driven batching lane. They were **not analyzed in this lane**. `originals/acquisition.tsv` is an ignored convenience inventory duplicating the original-byte table, not a required restoration input. `literature/clipper-nsdi17.txt` is a derived `pdftotext -layout` reading copy, not an original.

## Selected extracted code and reading depth

Clipper archive-to-extraction comparison verified all 421 retained files byte-for-byte. A deterministic extracted-tree digest is `11847877494881f0769215072803937ca23fae785daa55f8b510be3dcf67f283`: SHA256 of concatenated UTF-8 lines `file_sha256\tbyte_count\tposix_relative_path\n`, sorted lexicographically by relative path. The digest covers ordinary extracted files only. No `.git`, `.hg`, `.svn`, `.DS_Store`, `__pycache__`, `node_modules`, `target`, symlinks or special files are retained in that working copy.

| Selected Clipper path | Bytes | SHA256 |
| --- | ---: | --- |
| `LICENSE.txt` | 11357 | `b40930bbcf80744c86c46a12bc9da056641d722716c378f5659b9e555ef833e1` |
| `src/libclipper/src/containers.cpp` | 14154 | `ae23eb6ed1818d84e099e58ed8364a12657b6950abe20ea7baab624ab9c1dd47` |
| `src/libclipper/include/clipper/containers.hpp` | 6768 | `40b0a124c82b5201e65d2438d728b225684146f53ee05304bba781ab8cddef67` |
| `src/libclipper/include/clipper/task_executor.hpp` | 32906 | `b0f5ef58c237f8540fc537e4abe219f88ee917e5b9da4621b21fe1ea930cff74` |
| `src/libclipper/test/container_test.cpp` | 7104 | `c97943ae5dac179b8d8aef00d62b439c516e0a931f44167db16cd367a46a0c74` |

| Material | Read depth and limits |
| --- | --- |
| Linux `Documentation/networking/net_dim.rst` | Read introduction, algorithm, registration and application contract (lines19–120); tuning section consulted. API reference not exhaustively read. |
| Linux `lib/dim/net_dim.c` | Read all profile definitions and complete step/comparison/decision/measurement algorithm, especially lines15–57 and236–379. Device registration/accessor plumbing inspected rather than exhaustively studied. |
| Linux `lib/dim/dim.c`, `include/linux/dim.h` | Shared helpers read in full; constants, sample/state structures and sample-update functions read. Not all exported declarations studied. |
| Linux `drivers/net/ethernet/mellanox/mlx5/core/en_dim.c` | Read full 149-line file, with close reading of profile application and reset, lines36–63. Other driver datapath files not acquired or audited. |
| Clipper paper | Close reading §4.3, pp.618–619, including AIMD/regression comparison and delayed batching; front matter and experiment context consulted. Whole paper not claimed deeply read. No performance numbers imported. |
| Clipper `containers.cpp`, `containers.hpp` | Close reading constructor, completed-datum update, budget selection, regression fit, exploration/estimation, constants and mean/std helper. Active-container registry plumbing inspected. |
| Clipper `task_executor.hpp` | Close reading ModelQueue, deadline expiry and batch formation (lines135–191,247–262), dispatch timestamps and response observation/update (691–775). Rest of large executor not comprehensively audited. |
| Clipper `test/container_test.cpp` | Read batch-budget and estimator selection tests and iterative mean/std test, lines14–100. Read-only study; no test execution or reproduction claim. |
| Linux TCP header/output, remaining Clipper files | Acquired only; no reading claim. Clipper license read. Bundled libraries retained for coherent source context but not studied. |

## Restore originals and stripped source copy

From repository root, the following standard-library script reads the original-byte table above, refuses overwrite, checks every expected length/hash, then extracts only ordinary Clipper files with the documented exclusions. It does not fetch mutable branches. HTTPS availability and the listed public origins are required. Partial downloads are not installed. To regenerate the reading copy afterward: `pdftotext -layout quarantine/sources/adaptive-batching/control/literature/clipper-nsdi17.pdf quarantine/sources/adaptive-batching/control/literature/clipper-nsdi17.txt`.

```python
from pathlib import Path, PurePosixPath
import hashlib, tarfile, urllib.request

report = Path("papers/recon/adaptive-batching-control.md")
base = Path("quarantine/sources/adaptive-batching/control")
items = []
for line in report.read_text().splitlines():
    fields = [s.strip().strip("`") for s in line.strip("|").split("|")]
    if len(fields) != 4:
        continue
    name, size, digest, url = fields
    if not name.startswith(("linux-v6.12/", "originals/", "literature/")):
        continue
    items.append((name, int(size), digest, url))
assert len(items) == 13
assert not (base / "clipper").exists()
for name, size, digest, url in items:
    path = base / name
    if path.exists():
        raise FileExistsError(path)
    data = urllib.request.urlopen(url, timeout=60).read()
    assert len(data) == size and hashlib.sha256(data).hexdigest() == digest, name
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("xb") as out:
        out.write(data)
skip = {".git", ".hg", ".svn", ".DS_Store", "__pycache__", "node_modules", "target"}
with tarfile.open(base / "originals/clipper-9f25e3fc.tar.gz") as archive:
    for member in archive:
        source = PurePosixPath(member.name)
        parts = source.parts[1:]
        if not member.isfile() or not parts or any(p in skip for p in parts):
            continue
        if source.is_absolute() or ".." in parts:
            raise ValueError(member.name)
        path = base / "clipper" / Path(*parts)
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("xb") as out:
            out.write(archive.extractfile(member).read())
```

Verification performed: all 13 original sizes/SHA256 values match, all 421 extracted ordinary files match original archive members, and selected source hashes match the table. These are acquisition checks, not algorithm validation. No code from these references was copied into Deleuze production source. Coverage limits and unsearched fronts are stated in the study; this manifest does not imply a complete adaptive-batching bibliography.
