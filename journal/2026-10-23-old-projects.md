# 2026-10-23 — old-project source lane



Created scripts/acquire-old.py as acquisition/restore tooling. Copied six Papier ZIPs intact into quarantine/archives/old; created eleven filtered source tar snapshots. Extracted 17 captures and 1,606 files into quarantine/sources/old, stripping Git/build/vendor/macOS detritus and excluding generated experiment results. Corrected the initial broad experiment selection to omit results and kept only source/test context. Selected code, tests, schemas, fixtures, and docs form research context; no production crate was implemented and no dependency was adopted.





Verification: ran python3 scripts/acquire-old.py --restore using retained archive bytes; it restored and hash-checked all 1,606 selected members across 17 captures against tracked manifests. No acquired application tests ran: this phase records static source findings, not runtime reliability claims. Handed archive revisions and strongest findings to the root lane for remote-variant comparison and synthesis. Remaining gaps: vendor/literature forests and rescue personal-data images intentionally excluded; archive dirty state unavailable; other historical variants and live services owned by separate lanes.
