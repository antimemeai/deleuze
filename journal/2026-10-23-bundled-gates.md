# Bundled HTTP and gRPC gates

The operator said "send it" after the optional Tokio/Axum proposal. The concrete
selection also explicitly discussed the standard http-body and tower-service
traits for trailers and synchronous request ownership. Native-only builds keep
only BLAKE3. Raw-content unary gRPC avoids requiring a producer-built event or
protobuf wrapper; Python and Node clients demonstrate the profile.

Independent design and implementation-plan review found no architectural
blocker. One concrete clarification was accepted: aggregate unknown > failed >
retained across every payload in a request. Later message success cannot erase
an earlier failure. Count terminal requests once, retained records separately.

The implementation uses bounded body acquisition, preserving acquired prefixes
on error, followed by one owned blocking task per request using public
Log::append. Response cancellation and acquisition deadlines never cancel storage.
Power-loss work, request archives and separate persistence paths remain excluded.

Independent client review reproduced two bugs: HTTP receipt names were treated
case-sensitively, and Node accepted a gRPC success status in initial headers
without the required trailing status. Two direct fixtures failed against the
original clients, then passed after lowercasing receipt names and requiring
status trailers for framed success. No dependency was added for these tests.

The operator requested many commits and liberal pushes. The reviewed plan,
clients and their regression fixtures were committed and pushed first; subsequent
coherent implementation/validation changes will follow the same cadence.

The optional transport implementation compiled with all features/targets and
was committed/pushed as its own milestone. The live polyglot check then passed:
Python and Node each submitted HTTP and gRPC content to a real listener. Native
reads matched all four payloads exactly (including binary, malformed and empty
content); receipt sequences/hashes and request/record counts matched too.

Source review corrected six content/protocol regressions, each observed red
before correction: HTTP route classification, media casing, HTTP/gRPC coding
hints, outer HTTP coding before framing and uncompressed flag0 labeling. All
18 direct gate tests passed. The fixes/tests were committed and pushed together,
and the live four-case polyglot check passed again after these source changes.
Independent closure review found no remaining concrete blocker.

Final checks passed: 39 all-feature runtime tests and three compile-fail
doctests; native-only 20 runtime tests and its doctests; formatting and Clippy
with warnings denied. The opt-in live polyglot test and two client fixtures pass
separately. No benchmark or reliability-nines result is claimed. Capture/latency/
cost measurement and pre-service parser observation remain tracked research;
optional profile extensions and publication remain separate issues.
