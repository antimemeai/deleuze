# Fast capture, tracing, and replay: research lane

2026-10-23. This corpus studies low producer overhead, sustained capture, causal
context, and the boundary needed to reconstruct observations. It does not choose
a Deleuze design or adopt a dependency.

- [Bibliography](bibliography.tsv): 39 primary works/specifications; 36 original
  documents acquired, 3 unavailable originals recorded explicitly. Fifteen have
  core sections read, six have selected sections read, three have only front
  matter/abstracts read, and twelve remain acquired but unread. None of these
  labels means a whole dissertation or manual was read.
- [Study report](study.md): literature lineages, conflicts with capture of every
  gate arrival, measured boundaries, and links to local implementations.
- [Reference manifest](references.md), [machine-readable manifest](references.tsv):
  eight pinned source references, original archives, license locations, and
  restoration instructions. Per-reference TSVs list SHA-256, byte count, and path
  for all 16,940 extracted ordinary files.
- [Reading ledger](reading-ledger.md): exact sections/code paths examined and the
  distinction between acquisition, static reading, and execution.
- [Access gaps and further searches](gaps.md).
- [Recent forward-citation supplement](recent-forward-citations.md), with its
  [bibliography](recent-bibliography.tsv): explicit 2024–2026 searches, verified
  citation edges, five additional primary PDFs, and an upcoming-work announcement.

Original papers and captured official HTML are in ignored
`quarantine/archives/survey-fast-capture/literature/`. Derived PDF/HTML text is in
`quarantine/sources/survey-fast-capture/literature/`. Reference source archives and
extractions occupy the corresponding `references/` directories. The references
occupy about 73 MB compressed; they are reading material, not build dependencies.

The bibliography records authors, year/venue, confirmed DOI where available,
primary URL, final URL, local original path, acquisition time, SHA-256, access
state, and reading state. Blank DOI means unconfirmed or not assigned, not a
fabricated identifier. For mutable official documentation, the recorded hash
identifies the captured original; a future download may differ. Retain the
originals separately from Git if exact restoration is required.
