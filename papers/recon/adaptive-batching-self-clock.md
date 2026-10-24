# Adaptive batching: self-clock acquisition manifest

2026-10-23. Public-only acquisition. Lane findings are in [self-clock-lane.md](../survey/adaptive-batching/self-clock-lane.md). No production source, dependency, private material or commit was added by this lane.

All paths below are relative to ignored `quarantine/sources/adaptive-batching/self-clock/`. Byte hashes are SHA-256 of original response/archive bytes. Derived text, tests and counterexamples are separate from originals.

| Original | Primary origin / pinned identity | Bytes | SHA-256 | Status |
| --- | --- | ---: | --- | --- |
| `paper.pdf` | `https://arxiv.org/pdf/2606.18187v1` | 591905 | `f840cb4b8eb97f6a8ad3e540a35809620f839d8c0930d547c7ae39448c9ffea9` | Valid PDF; all five pages read. Authors Madhulatha Mandarapu and Sandeep Kunkunuru;2026; arXiv:2606.18187v1; DOI10.48550/arXiv.2606.18187. arXiv license link is CC BY4.0. |
| `paper.html` | `https://arxiv.org/html/2606.18187v1` | 81122 | `4658e6d57964e354f32f0e0ac55edb5c8aab2951f53bdbe7da9a2b63855db24b` | Retained alternate rendering. |
| `group-commit-policy-c7d80f7d0e63ad1fa6bd4f8dba5853bcf31c5a28.tar.gz` | `https://codeload.github.com/samyama-ai/group-commit-policy/tar.gz/c7d80f7d0e63ad1fa6bd4f8dba5853bcf31c5a28` | 276531 | `8c9e6e10d9d77a0dcee096b97b3c40eab13d0efbbfdc5b1598b0a993b3180bcf` | Original archive retained;32 regular files extracted to `group-commit-policy/`; no nested Git directory. Apache-2.0, Copyright2026 VaidhyaMegha Private Limited. Research simulator/benchmarks, not a logger implementation. |
| `github-head.json` | `https://api.github.com/repos/samyama-ai/group-commit-policy/commits/main` | 2506 | `1ecfaca9ed2260b349881d3a59be11084e43c0888e04fbaabf09cb4e0aa6c8f9` | Resolves then-current main to the exact commit above, dated2026-06-17T02:00:24Z; parent `d03139245ebdfb53717e3df46ed2c7b4972bb8ae`. Restoration must use the pinned commit, not current main. |
| `paxos.pdf` | `https://d-nb.info/1129780104/34` | 937516 | `9e7f6deb4d2ed975ccd82cc0b0321887068a6e6916e2ce64ae9184646d1da12f` | Valid nine-page original publisher paper from German National Library repository; CC BY4.0. DOI10.1007/s41019-017-0039-z. Central §§2.2,4.2–4.3,6 read. Exact OceanBase0.4.2 source revision not acquired. |
| `laser-publisher.html` | `https://link.springer.com/chapter/10.1007/978-3-319-63579-8_25` | 293806 | `b9440770f84d587da7e28485091ad61565548eb10318a8dba57dd02b5dc55859` | Publisher abstract/reference preview acquired/read. Full chapter requires institutional access/purchase; not acquired. |
| `laser-pdf-response.html` | `https://link.springer.com/content/pdf/10.1007/978-3-319-63579-8_25.pdf` | 293806 | `e625dbe3c7fd18fea0b5788e755f2b5981a924f0271317b7af92f72665ac1fab` | **HTML, not PDF**: response starts `<!DOCTYPE html>`. Renamed to prevent false full-text acquisition. |
| `laser-crossref.json` | `https://api.crossref.org/works/10.1007%2F978-3-319-63579-8_25` | 4789 | `fe07c49b6bdb2f978d8b9132054f748b343ac3ff65a7b5da063b03a0ed510d76` | Publisher-deposited title/authors/pages/year/links. No abstract/full text in this response; TDM link is not an open license grant. |

Laser bibliographic identity: Huan Zhou, Huiqi Hu, Tao Zhu, Weining Qian, Aoying Zhou, Yukun He; *Laser: Load-Adaptive Group Commit in Lock-Free Transaction Logging*; APWeb/WAIM2017, LNCS10366,320–328; first online3August2017. Neither institutional login nor purchase was attempted. Bounded title/DOI/author searches found an institutional abstract record but no acquired public original chapter or exact reference implementation. This is an acquisition gap, not proof none exists.

## Source reading and verification

Selected prototype source: `group_commit/{sim,theory,device,experiment}.py`; `bench/{run_synthetic.py,run_real.py,measure_fsync.py,pg_commit_delay.sh}`; all three `tests/test_*.py`; README, PREREGISTRATION, REPRODUCIBILITY, project metadata/license; `results/{threshold,H5,pg_commit_delay_c2,pg_commit_delay_c32}.json`. Other archive files are preserved without a full-read claim. No upstream file was modified.

Useful independently checkable source hashes:

| Extracted file | SHA-256 |
| --- | --- |
| `LICENSE` | `c3f4100c89815992afd7ce539a37eae2a8f5b1d8760e30a47fbd53a6300b1e86` |
| `group_commit/theory.py` | `aa6b19842a5aa1261503e9fad5ca42ae14ac195293b97130a70634e73344ac31` |
| `group_commit/sim.py` | `1746caf97dbc735811e2a7af45ce9cc36962327f11c450598ba3d1d4b3a3113d` |
| `results/threshold.json` | `2fa77481e88252208a92fd550ab6c949591ad17f6b77d4ed2985e5afe726c12b` |
| `results/H5.json` | `b280ab4e006facd6c9b8732d7387017a1121ca2e5d126430980dfa10da8a03c5` |

Derived `paper.txt` and `paxos.txt` were generated with `pdftotext -layout`. `laser-publisher.txt` is a derived HTML text view. `counterexamples.json` and `counterexamples.py` hold the small executed policy probes; `test-run.txt` retains the command/output: `PYTHONDONTWRITEBYTECODE=1 python3 -m pytest -q -p no:cacheprovider`,14passed in3.07s. Existing environment packages were used; no installation or project dependency adoption occurred. No real-device/PostgreSQL experiment or complete figure regeneration was run.

## Restoration

Re-download the versioned arXiv PDF and the **pinned** codeload URL; verify SHA-256 before extracting. Extract archive regular files after removing its single outer directory component; reject absolute/`..` paths and do not create archive symlinks. The snapshot is all32 regular files, including its original LICENSE, CITATION, tests and stored results. The original archive remains authoritative; do not substitute a later main branch. Restore the Paxos PDF from its retained National Library URL and verify hash. Laser HTML/metadata are preview evidence only and cannot restore the missing chapter. Preserve the HTML-returned-as-PDF negative result as such.

The machine-readable `acquisition.json` beside the originals records the same origins/response hashes. This tracked manifest provides restoration without depending solely on ignored metadata. No broad citation-completeness claim is made.
