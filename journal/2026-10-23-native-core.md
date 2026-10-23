# First native core

The operator approved the official BLAKE3 crate after maintenance discussion,
then required restraint in cryptographic composition. The property hash uses
the documented ordinary BLAKE3 hash of retained property bytes. Custom header
cryptography and linked-history protocols were withdrawn.

The operator then excluded power-loss resilience from the current build. A
segment/reservation proposal had let that fault class distort a small program;
it was discarded before implementation. The revised design uses one append-only
file, ordinary I/O errors and clean reopen. No repair or history mutation API.

Independent design and plan review found no blockers. It called out independent
reader file cursors, which will be covered directly in validation. The native
unit is implemented before network gates; BLAKE3 is the only approved direct
dependency. Transport choices remain proposals. Private research stays external.

The first native crate now implements owned text/bytes/properties/opaque packing,
private record metadata, synchronous append, clean reopen, linear read/query and
result-bounded pagination. It uses only the approved BLAKE3 direct dependency.
The default completes file writes; optional per-append sync calls the ordinary
standard-library method. The crate is not published; licensing remains unset.

Validation passed 20 tests and three compile-fail doctests. Independent byte
fixtures check the format; direct I/O tests check short writes, interruptions,
write/sync failures and stopped-writer behavior. Other checks cover malformed
content, unchanged damaged files, capacity versus corruption, sequence/time
boundaries, metadata authority, immutable observations, pagination, ownership
locking and concurrent reads/appends. Formatting and Clippy with warnings denied
passed. No benchmark or deployment-reliability result is claimed.

HTTP/gRPC gates and optional profile extensions are separate Beads issues.
Network dependencies remain proposals, with the operator discussion pending.
The custom-adapter guide describes the common append path. No additional
request archival or persistence subsystem was added.
