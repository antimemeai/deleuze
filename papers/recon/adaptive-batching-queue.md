# Adaptive batching: queue and throughput controller originals

Acquired 2026-10-24 UTC. Public source-reading material only; no dependency
adopted or source copied into production. Original bytes remain in ignored
`quarantine/sources/adaptive-batching/root/`. Reports identify selected reading,
not whole-system validation. Re-fetching mutable documents may yield new bytes.

| Original | Source / version | Bytes | SHA-256 |
| --- | --- | ---: | --- |
| `seda.pdf` | [Matt Welsh author copy](https://mdw.la/papers/seda-sosp01.pdf), SOSP 2001; Welsh, Culler, Brewer | 305982 | `916a1e3216cc809600e187592d54e97d87c8c02747c36ccf9de38e4029bd5a5d` |
| `seda-release-20020712.tar.gz` | [Original SEDA 3.0 release](https://downloads.sourceforge.net/project/seda/seda/3.0/seda-release-20020712.tar.gz), Matt Welsh / UC Regents | 326630 | `7e5f2fc1212fda9fbc088a1b5441f9fa3cfaa84c96a8883ed8fb386bb44ea9ee` |
| `seda-javadoc.html` | [Berkeley AggThrottle docs](https://people.eecs.berkeley.edu/~jonah/javadoc/seda/sandStorm/internal/AggThrottle.html), undated | 26598 | `23cb5d6c3fb88eb165984b379ba1711cc1eadb4f23493ff0bd633597afa15faf` |
| `rfc896.txt` | [RFC Editor original](https://www.rfc-editor.org/rfc/rfc896.txt), John Nagle, January 1984 | 26782 | `9f50ffb6d2557be2727bb0f4ca9b5dec9777b3617c66e47822d2ba28df2dfda3` |

SEDA paper carries ACM copyright and personal/classroom copying terms. Source
headers carry the UC Regents permissive notice and warranty disclaimer; read
individual files before reuse. Javadoc has no separately verified redistribution
license. RFC retained for research; no software license inferred from the memo.

Restore from repository root:

```sh
mkdir -p quarantine/sources/adaptive-batching/root
curl -fL https://mdw.la/papers/seda-sosp01.pdf -o quarantine/sources/adaptive-batching/root/seda.pdf
curl -fL https://downloads.sourceforge.net/project/seda/seda/3.0/seda-release-20020712.tar.gz -o quarantine/sources/adaptive-batching/root/seda-release-20020712.tar.gz
curl -fL https://people.eecs.berkeley.edu/~jonah/javadoc/seda/sandStorm/internal/AggThrottle.html -o quarantine/sources/adaptive-batching/root/seda-javadoc.html
curl -fL https://www.rfc-editor.org/rfc/rfc896.txt -o quarantine/sources/adaptive-batching/root/rfc896.txt
shasum -a 256 quarantine/sources/adaptive-batching/root/seda.pdf quarantine/sources/adaptive-batching/root/seda-release-20020712.tar.gz quarantine/sources/adaptive-batching/root/seda-javadoc.html quarantine/sources/adaptive-batching/root/rfc896.txt
```

Compare hashes with the table before extraction. Extract the release beneath
`quarantine/sources/adaptive-batching/root/seda/`; retain its original archive.
Remove `.git`, `CVS`, `.svn`, `__MACOSX` and `.DS_Store` metadata from the working
tree. `pdftotext -raw` creates the selected-reading `seda-raw.txt`; PDF bytes are
the original. Read source paths relative to
`seda/seda-release-20020712/`: `seda/src/seda/sandStorm/internal/`
`AggThrottle.java`, `ThreadPool.java`, `TPSThreadManager.java`.

Existing BookKeeper/Kafka source and Aether PDF provenance is in
[durable references](../survey/durable-logs/references-manifest.md) and
[bibliography](../survey/durable-logs/bibliography.tsv); no second snapshots.
Linux TCP v6.12 raw source provenance is in the
[control manifest](adaptive-batching-control.md).

An attempted primary-author Harvard PDF URL returned 404; the author's current
domain supplied the SEDA PDF. The CiteSeerX full-text URL for *Group Commit Timers
and High-Volume Transaction Systems*, Tandem TR 88.1 (1988), returned 404. The
latter is an acquisition gap, not a read paper or validated algorithm here.
