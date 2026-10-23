# 2026-10-23 — Fast capture, tracing, and replay literature

Continued the current-project acquisition lane with a distinct research unit:
low-overhead/high-throughput capture and boundary-faithful replay. Root owns
issues, synthesis, and operator discussion. No production implementation,
dependency, source-project edit, commit, or reference benchmark was authorized
or performed.

Built a tracked 39-entry primary bibliography spanning logging, tracing,
shared-memory buffers, causal context, record/replay, queues, and kernel I/O.
Acquired 36 primary original PDFs/official HTML documents, preserving original
bytes under ignored quarantine and recording source URL, authors/year/venue,
confirmed DOI, acquisition time, access state, and SHA-256. Three paper originals
remain inaccessible: DoublePlay, Michael–Scott queues, and Axboe's 2019 io_uring
article. Recorded actual HTTP/TLS failures and routes to acquire them; did not
disable certificate verification or substitute tertiary summaries.

Read core sections of fifteen works/specs and selected sections of six more;
three have only front matter/abstract reading, and twelve remain acquired unread.
Saved exact extents in the bibliography and a reading ledger. The LTTng
dissertation and official manual are deliberately marked partial reading.

Acquired eight pinned implementations: NanoLog, LTTng-UST, Perfetto, rr,
Hindsight, the Brown Pivot/X-Trace framework, liburing, and a Linux v6.12
tracing/event/I/O source selection. Preserved original downloaded archives;
Linux's archive is an export of selected tracked paths from the named commit.
Stripped nested Git metadata, filesystem detritus, and symlinks only from working
extractions. Tracked source revisions, license locations/scopes, original hashes,
and per-file inventories for 16,940 ordinary files. The Linux filtered clone's
archive export fetched additional Git objects; the retained source archive is
still only the documented selection. No installed kernel was changed.

Read producer reservation/commit, buffer handoff, saturation, failure, and I/O
paths. Important distinctions reported to root: NanoLog's default file flags
include O_DSYNC, so its aggregate flushed throughput is not merely a memory
benchmark; its per-call nanosecond latency is still a producer-call measurement.
Hindsight deliberately evicts traces and uses a null buffer on exhaustion.
Perfetto has drop/stall/stall-then-drop behavior with loss reasons and explicit
IPC-thread deadlock handling. Kernel rings have specific nesting/progress
assumptions rather than a universal lock-free property. The old rr report's ARM
limitation differs from its current source's AArch64 support.



Root then requested explicit recent forward-citation checks. Searched 2024–2026
successors and recorded the exact queries and closure limits in a separate
supplement. Acquired five more primary PDFs: Astraea, TraceMesh, Tracezip,
Nõu et al.'s tracing-overhead study, and PrvTel. Verified citation edges in the
first four PDFs rather than relying on related-paper suggestions. μSlope's
publisher abstract was read but PDF acquisition returned HTTP 403. Captured
T-PACK's primary author announcement for upcoming NSDI 2027; the July 2026 news
entry is an announcement date, not a 2026 publication or completed reading.

Recent study separates instrumentation/sampling selection, reconstructive
compression with synchronized dictionaries, and generative query approximations.
Tracezip's label of an “open-loop” Locust workload was recorded as requiring
artifact inspection, not accepted or rejected without evidence. Root was told
that overload/tail-latency evaluation is a useful next independent conceptual
research unit.

Verified all recorded acquired-original hashes and every reference archive,
inventory, and extracted-file byte. Checked for forbidden nested Git metadata
and filesystem detritus. No reference tests or performance claims were executed;
acquisition verification only identifies the corpus. The main bibliography and
recent supplement together contain 46 records, including three unavailable
root originals, one unavailable recent PDF, and one author announcement rather
than a full paper.
