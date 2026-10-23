# HTTP, gRPC and custom adapters: proposed first slice

2026-10-23. A concrete transport candidate and implementation sequence for review.
The operator approved shipping HTTP/gRPC gates and custom-adapter guidance;
frameworks, dependencies, signatures and this wire profile still require discussion.
No dependency is adopted by this document. The [capture contract](capture-contract.md)
and [record profile](record-profile.md) control record authority and ownership.

## Small first release

Ship one ordinary HTTP content submission and one unary gRPC content submission,
with a dedicated ingestion listener/service boundary. Producers send existing
output bytes; they do not construct a Deleuze Event or provide sequence, hashes
or authoritative timestamps. Structured interpretation can be added through the
common packing configuration; it is not required for this first transport slice.

The baseline retains supplied content once as bytes/opaque content or the core's
owned-content reference. A representation identifier describes content encoding;
it does not copy request headers, routing information or a request object into the
log. Opaque content is the event's content, not an accompanying archive. Arbitrary
properties and producer-provided timestamps remain property data. Deleuze's common
packing/write path assigns record fields under the selected profile. Transport
code cannot set `seq`, `property_hash` or record times.

Expose no update, delete, replace, truncate or administrative mutation endpoint.
The same restriction applies to native/custom capture capabilities. Read-side
producer-mask reference assistance remains separate, configuration-driven and
ABAC-authorized; these gates neither unmask nor resolve producer references.

First slice: no client/bidirectional streaming ingestion API, protobuf reflection,
request archives, request lifecycle records, remote artifact fetching, ingest-time
redaction or per-gate storage writers. Reading a large unary body incrementally
is content acquisition, not an extra streaming API.

## One public operation, independent execution ownership

Gate adapters receive only the supported public capture capability and completion
interface. Describe the required behavior as `capture(owned_content_source)` until
root chooses the concrete Rust signature. This is one operation accepting ordinary
native/encoded/streamed content representations; it is not a family of unchecked
storage entry points. No adapter receives writer handles, sequence allocators,
unchecked append functions or internal packing stages.

At request notification the gate establishes arrival/failure accounting. At first
content acquisition, or when its owned reader is available, it synchronously hands
the source to capture before cancellable conversion. Successful handoff starts an
engine-owned execution lifetime; returning an unpolled future is insufficient.
The response task holds a completion observer only. Dropping that observer does
not drop the source or the engine operation. An operation may stop acquiring new
content after a disconnect, deadline or acquisition budget; chunks already acquired
remain owned and proceed to retention or an actual failed/unknown outcome.

The reader adapts a transport body into event-content chunks. It has no engine
mutation authority. It may temporarily hold protocol parsing state, but only
content enters the packing operation. The operation owns conversion, property
representation, hashing and the common storage handoff. This makes a custom
adapter another source of content, not another recording implementation.

## HTTP wire candidate

`POST /v1/capture` accepts an ordinary request body. A producer can send text,
JSON, protobuf, binary output or an empty body without a Deleuze wrapper. The
baseline does not parse JSON and therefore does not round numbers, collapse keys
or replace malformed text. A declared media type is an interpretation hint only;
unknown or malformed event content can remain opaque.

For the first slice, HTTP content coding is identity only. Unsupported coding
returns 415 after the available encoded content is retained opaquely, or reports
capture failure if acquisition/retention fails. This is retention of submitted
content, not the transport request. No automatic decompression middleware runs
before capture ownership. Later compression support must retain a safe fallback
when interpretation fails or reaches its budget.

Observe arrival synchronously in the outer HTTP service's request notification,
before application routing, authentication, body extraction or application limits.
Move the body reader to the operation there. Normal handlers do not first collect
or decode it. Unknown methods/routes are terminal protocol outcomes; they cannot
remove an already observed arrival or silently discard acquired event content.
This dedicated ingestion service must not also treat unrelated read/control APIs
as content submissions.

A response indicating retained success waits for the core's configured storage
boundary. Proposed external mapping: 201 with the core receipt for a valid retained
submission; 4xx for protocol rejection even if its available content was retained;
503 for known acquisition/capture failure; and an explicit unknown outcome when
the operation's storage result cannot be established. Exact receipt encoding and
error response fields belong with root's core API decision. A disconnect may
prevent any response. A sent response is not proof that the peer received it.

## gRPC wire candidate and interoperability

Propose unary `/deleuze.v1.Capture/Append` using raw content messages and
`application/grpc+deleuze`, with ordinary gRPC HTTP/2 framing and trailers. The
request message bytes are producer content directly, not serialized record fields.
The response can be an empty message plus a normal retained receipt in response
metadata, after the core completion boundary. The exact receipt profile must be
shared with HTTP/native results. Nonzero `grpc-status` communicates protocol or
capture errors; HTTP 200 alone never means retained success.

Raw messages require clients that support custom serializers/codecs. They are not
a `.proto`-generated application message. Supply executable client examples for at
least two languages and verify their wire interoperability before claiming support.
Generated protobuf clients and descriptor-driven tools do not automatically know
this profile. If operator priorities require those clients, the alternative is a
tiny protobuf `bytes` content carrier and response receipt schema; that carrier
is transport packaging, not a producer-built Deleuze Event. It introduces a
protobuf build/runtime choice to discuss, even though `protoc` is available.

Tonic supports custom `Codec`, but its documented `Decoder` receives a complete
message after gRPC framing. A raw Codec alone therefore cannot observe interrupted
message payloads or every pre-codec rejection. The proposed first implementation
uses the HTTP/2 service body reader to frame this one unary method before losing
content ownership. A five-byte prefix identifies compressed flag and payload
length; keep only temporary framing state, and send payload chunks through the
same public capture operation. HTTP DATA boundaries are not message boundaries.
Identity compression only is the first advertised capability. Unsupported or
invalid compression must not turn available payload bytes into decoder-only loss.

This is a bounded gRPC profile, not a new general gRPC stack: delegate HTTP/2,
flow control and connection management to a reviewed transport dependency. Implement
only service validation, unary message framing, raw content handoff, receipt
framing and standard trailers. A Tonic-based implementation remains an alternative
if a verified pre-codec source/ownership hook satisfies the same cases without
parallel recording. Do not pretend a handler/interceptor solves that obligation.

A valid unary call supplies exactly one message. Zero messages are unavailable
content, distinct from one valid zero-length message. Extra messages make the
RPC invalid. Their already acquired payloads cannot be erased: retain each
available event-content item through capture, or report the actual capture
failure. This can yield multiple content records for an invalid multi-message
call; it does not create a parent RPC record. Request arrivals and content-item
counts remain separately observable. This behavior must be accepted as part of
the wire profile, rather than hidden behind a successful unary decoder.

## Failure population and partial content

All request notifications at the named gate boundary count, including full/failing
capacity, unsupported methods, invalid input and pre-content rejection. Record
counts are not the arrival denominator. Outcomes/failure reporting do not require
another persisted audit stream. A failure counter does not substitute for content
that should have been retained.

| Case | Required first-slice behavior |
| --- | --- |
| Empty, normally completed content | Capture the known empty content; do not confuse it with no acquired content. |
| Reader error/reset after content prefix | Keep acquired content, report incomplete acquisition, and never acknowledge complete retention. Any retained prefix must remain distinguishable as incomplete through the core's content representation/outcome contract. |
| Malformed event bytes | Retain opaque content once; no structured decoder is required in the baseline. |
| Invalid gRPC framing before a payload exists | Report failed/unavailable content; do not manufacture an empty event or persist framing bytes. |
| Invalid framing after available payload | Preserve available event content; protocol failure is independent of its storage result. |
| Budget/capacity exhausted | Keep ownership while waiting within configured budgets, or report failed/unknown capture with recoverable input where possible. Discarded observed content is capture failure. |
| Deadline/disconnect after ownership | Stop further acquisition if necessary; do not cancel already owned retention because the response observer disappeared. |
| Lost response after storage completion | Engine result remains retained; caller may have unknown completion. A retry is another arrival. |
| Shutdown | Stop ingress, account for observed races, drain owned operations or report failed/unknown outcomes; do not drop a request runtime first. |

Bound acquisition bytes, duration and concurrent retained-source work through
explicit configuration. These are operating bounds, not grounds to exclude failed
arrivals from reliability or load measurements. No unlimited body allocation or
infinite slow-reader lifetime is promised. Supporting large content without one
giant allocation requires the core's owned-content/reference facility; if that
facility is not in the first implementation, oversized submissions remain visible
capture failures rather than a claim of complete large-content support.

The HTTP library may reject malformed request syntax before exposing a request
object. Name the lower-level parser/error notifications available in the selected
library and report unavailable-content failures there. Do not claim exact request
counts where a connection error cannot identify individual requests. This remains
a measured coverage limitation until the transport hooks and fault tests establish
it; selecting a library is not evidence that every gate arrival is observed.

## Dependency discussion, before implementation

Candidate A is a Tokio-based HTTP/1+HTTP/2 service with a streaming body interface
and the bounded raw gRPC profile above. Hyper/body support would be proposed
explicitly with their actual feature/dependency costs; none is adopted here.
Axum can provide routing if justified, but ordinary `Bytes`/`Json` extraction is
not the capture boundary. Its documented default body limit can reject before
a handler and does not apply to direct body polling. Tonic can provide gRPC codec
machinery if its complete-message boundary is bridged honestly. Neither framework
is required merely because it was mentioned earlier.

Candidate B uses Axum/Tonic and a tiny protobuf content carrier for conventional
generated clients. It still needs the same outer arrival and partial-content
ownership mechanism; protobuf decoding does not remove those requirements.
Root/operator should choose the interoperability/dependency tradeoff before code.
Do not implement two independent gate pipelines while evaluating these candidates.

## Implementation sequence and direct conformance

1. Bind the public capture source/completion API to core ownership, partial-content
   representation and retained/failed/unknown results. Keep record authority in core.
2. Build the outer request observer/body-source transfer once; exercise synchronous
   handoff and response-future cancellation without opening a real network first.
3. Add the HTTP identity-body endpoint over that source; support opaque content and
   common receipt mapping. No Event/record input DTO or semantic JSON parser.
4. Add the unary gRPC framing/source adapter and standard result framing. Resolve
   raw-versus-protobuf client packaging before this step, not midway through it.
5. Add custom-adapter guidance and two-language examples using only the public
   capture surface; demonstrate producer masking as ordinary supplied content.
6. Run direct protocol/content/failure checks against a known offered input set;
   then measure offered-load retention, sustainable throughput and p50/p95/p99
   completion latency with payload sizes, budgets and storage boundary attached.

Required conformance cases: bytes including invalid UTF-8 and malformed JSON;
empty versus unavailable content; producer fields named `seq`, `created_at` and
`property_hash` retaining no record authority; binary payloads unchanged; known
partial prefixes; every split of a gRPC prefix across DATA chunks; multiple
messages per DATA chunk; zero/extra unary messages; invalid flag/length/coding;
overload before and after content handoff; deadline/reset after handoff; a retained
record followed by a lost response; and shutdown with operations in flight.

For each case, assert exact available content or declared incomplete prefix,
correct retained/failed/unknown completion, core-owned sequence/hash/timestamps,
unchanged append-only capability surface, and no duplicate persisted request/body
representation. Drive arrival count from the external test inputs, not only the
successful records. Compile-fail/API-surface checks ensure adapters cannot obtain
internal writers or update/delete methods. These checks bear on this slice's
claims; they are not an implementation of a second production capture ledger.

## Primary sources consulted

- [gRPC HTTP/2 protocol](https://raw.githubusercontent.com/grpc/grpc/master/doc/PROTOCOL-HTTP2.md): message framing, compression boundaries, status/trailers and DATA/message independence.
- [Tonic Codec](https://docs.rs/tonic/latest/tonic/codec/trait.Codec.html) and [Decoder](https://docs.rs/tonic/latest/tonic/codec/trait.Decoder.html): custom message encoding and full-message decoder boundary.
- [Hyper Service](https://docs.rs/hyper/latest/hyper/service/trait.Service.html): request notification returns a response future; work can occur later.
- [Axum DefaultBodyLimit](https://docs.rs/axum/latest/axum/extract/struct.DefaultBodyLimit.html) and [Body](https://docs.rs/axum/latest/axum/body/struct.Body.html): extractor limits versus direct body acquisition.

These docs were read as public API/protocol sources. They are not evidence of a
particular implementation's cancellation safety or complete malformed-input
coverage; the direct checks above must establish those properties for the chosen
version and integration.
