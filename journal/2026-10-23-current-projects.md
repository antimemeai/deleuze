# 2026-10-23 — Current projects reconnaissance

Read Deleuze `AGENTS.md` and the linked `blackbird.md`, then ran `bd prime`.
Root owns issue tracking and the overall acquisition discussion. This lane was
limited to current local projects and did not edit source repositories or commit.





Acquired 403 selected source/test/schema/document files from four distinct projects
into ignored `quarantine/sources/current/`; retained original selected-file tar.gz
archives in `quarantine/archives/current/`. Added the standard-library acquisition
script, tracked acquisition metadata and per-file SHA-256 inventories, an aggregate
manifest, and an independent static-reading report under `papers/recon/`.
Extraction was compared byte-for-byte with source. Every selected file and archive
has a recorded hash; metadata includes full dirty state and origin revision.



Verified inventories against both retained archive contents and extracted files,
checked for nested Git metadata/detritus, and checked acquisition-script syntax.
The restore command uses retained archives or byte-identical local selections;
it refuses to overwrite an existing extraction or claim a dirty snapshot can be
reconstructed from HEAD alone. Root will combine independent lanes and return
to the operator for the design discussion.
