# First bundled gates

2026-10-23. Concrete selection from transport-gates.md after the operator said
"send it" to the optional Tokio/Axum proposal. One native append path; no new
persistence layer, request archive, semantic JSON model or security protocol.

## Public boundary and dependencies

Feature `gates` enables Tokio1.x (runtime, macros, net, time, sync, signal, I/O),
Axum0.8.x (HTTP/1, HTTP/2, Tokio), http-body1.x and tower-service0.3.x. The last
two are the standard transport body/service traits, explicitly discussed before
adoption. Default native-only builds still have BLAKE3 as their sole direct
dependency. No Tonic/prost/code generator. Raw gRPC clients use custom byte
serializers; Node can use its built-in HTTP/2 client, Python can use grpcio.

Expose cloneable `gates::Gate` holding Arc<Log>, gate options and shared lifecycle/
counts. Implement Service<Request<Body>>: call synchronously counts arrivals,
checks closed/in-flight capacity, registers work and spawns an owned task with
the body before returning a response future. The future only observes completion;
dropping it never cancels that owned task. No router/extractor runs before this
point. A small make-service implementation lets Axum serve the gate directly.
Gate::serve(listener, shutdown) closes admission, gracefully stops transport,
and drains owned operations before returning. Custom hosts must close/drain
before dropping their runtime. A mutex serializes admission versus close/drain;
a notification wakes drain after active work becomes zero. No unbounded queue.

Default options: 64 in-flight requests, 1MiB acquired body bytes, 30s body-read
budget. Bounds are configurable; zero/overflow options rejected. Saturated and
closed requests return known capture failure, counted in total arrivals. A body
limit or reader error retains an acquired prefix once with associated property
`capture_incomplete=true`, then reports failed capture; never acknowledge a prefix
as complete. No observed payload means no fabricated empty record. A known,
complete empty HTTP body or zero-length gRPC message is valid empty content.
The body deadline bounds acquisition, not synchronous storage completion.

Stats are in-memory request outcomes (arrived, active, retained, failed, unknown)
and retained record count, including rejected routes/capacity. Protocol rejection
is separate from capture outcome. No persisted request ledger. Transport syntax
errors rejected before Service::call are a documented observation limit; do not
claim exact request counts or nines for them.

## Wire profile

POST /v1/capture always selects HTTP content mode regardless of Content-Type,
and stores body bytes unchanged through Log::append. No parsing or
decompression. HTTP201 means complete content plus core success. HTTP404/405/415
can accompany retained contents for invalid routes/methods/coding. Body too large
returns413, timeout408, read/storage failure503; each is failed or unknown capture
as appropriate. Header `deleuze-capture-outcome` is retained/failed/unknown;
`deleuze-seq`, `deleuze-property-hash` and write-time metadata describe the last
stored record when one exists. No client can assign record metadata.

Unary POST /deleuze.v1.Capture/Append, HTTP/2, application/grpc+deleuze (also
application/grpc for clients with custom serializers). Only identity compression.
Acquire bounded content across arbitrary DATA splits, preserving its prefix on
error, then parse standard five-byte prefixes in the owned buffer.
Payload bytes alone enter append; framing and request headers are not persisted.
Retain each available message in an invalid multi-message call, then return
nonzero grpc-status. Partial payloads retain their acquired prefix as incomplete.
Invalid/incomplete prefix without payload reports unavailable capture. Compression
or invalid flag rejects protocol while retaining available payload as opaque
encoded content with a representation label; no decompression. Total acquired
wire bytes bound multi-message work. Route/method/content-type/version validation
does not discard an observed content body. grpc-timeout may tighten body budget.

On success, response DATA contains one empty message (five zero bytes), followed
by standard grpc-status:0 trailers. Nonzero status reports invalid protocol,
resource exhaustion, deadline or storage failure. HTTP200 alone is not success.
Aggregate one terminal request outcome: unknown if any append is unknown; else
failed if any content is unavailable, incomplete or unretained; else retained.
Later message success never erases an earlier failure. Count retained records
separately, even for failed requests. Shared deleuze-* metadata reports capture
independently. No protobuf
message schema/reflection is claimed. No update/delete/read-admin network API.

## Implementation and direct checks

After independent design/plan review: implement the owned request task/lifecycle,
then bounded HTTP acquisition, unary gRPC framing and response
trailers through the same path. Use spawn_blocking for the public synchronous
Log::append so storage never blocks the async reactor. Acquire/parse the bounded
wire buffer first, then use one blocking task per request to append its payloads;
body-read deadlines never cancel storage. Keep only acquired event
content and small parser state; avoid extra body copies. Add a runnable server
example, Python and Node clients, and update the custom adapter guide.

Test exact bytes/metadata and arrival/outcome counts, complete empty versus
unavailable, every prefix split, multiple messages in one chunk, invalid flags/
compression/lengths, partial/error/timeout prefixes, bad routes, overload, dropped
unpolled response futures and close/drain races. Run real HTTP/1 and HTTP/2 gRPC
clients in two languages against a live listener and verify resulting native
records. Run native-only and all-feature test/build/fmt/Clippy checks. No power-loss
simulation, exhaustive protocol stack or throughput/nines claim in this slice.
