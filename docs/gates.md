# HTTP and gRPC gates

The optional `gates` feature provides HTTP and unary gRPC gates using Tokio/Axum;
native-only builds keep their synchronous API without a network runtime.
Both protocols feed `Log::append`. Producers supply existing content bytes,
without a Deleuze record, sequence or timestamp wrapper.

## Run locally

```sh
cargo run --features gates --example gates -- ./events.dlz 127.0.0.1:3000
```

The example prints its bound address. Use `127.0.0.1:0` for an ephemeral port.
Ctrl-C closes ingress and drains owned operations. Reopen the same clean log
on the next run. The example uses the core's default complete-write boundary;
it does not enable per-append synchronization.

```sh
printf 'ordinary producer output' | python3 examples/clients/capture.py http http://127.0.0.1:3000
printf 'ordinary producer output' | node examples/clients/capture.mjs grpc http://127.0.0.1:3000
```

Both clients support `http` and `grpc`. Node needs no npm packages. Python HTTP
uses the standard library; its gRPC mode needs `grpcio`. The examples use local
cleartext transport. Supply already masked content when the producer requires
masking. A supplied path or URI remains a value; nothing fetches its target.

## Wire interface

HTTP: `POST /v1/capture` with any body, including empty or malformed content.
This explicit HTTP route retains bytes regardless of the declared Content-Type.
The body is stored as bytes without parsing, normalization or decompression.
HTTP201 means complete acquisition and successful native append. Protocol
rejections can accompany retained content; check the capture metadata too.

Unary gRPC: `/deleuze.v1.Capture/Append`, using HTTP/2 and
`application/grpc+deleuze` (or `application/grpc` with a custom serializer).
The request message bytes are the content. The response is one empty message
followed by standard `grpc-status: 0` trailers. HTTP200 alone is insufficient.
Only identity compression is supported. This raw-byte profile uses custom
serializers; it has no generated protobuf message or reflection service.

Shared response headers describe capture independently from protocol status:

| Header | Meaning |
| --- | --- |
| `deleuze-capture-outcome` | `retained`, `failed` or `unknown` for the whole request. |
| `deleuze-seq` | Last record written by this request, when one exists. |
| `deleuze-property-hash` | That record's BLAKE3 property digest in hexadecimal. |
| `deleuze-created-at-seconds` | Deleuze's signed Unix write-start seconds. |
| `deleuze-created-at-nanos` | Normalized fractional nanoseconds. |

A protocol-invalid multi-message unary call may retain several payload records.
Its receipt describes the last one; its gRPC status remains nonzero. A later
successful message never erases an earlier failed or uncertain capture.
There is no additional request, body archive or parent lifecycle record.

## Ownership, limits and outcomes

`Gate::new(Arc<Log>, GateOptions)` creates the dedicated ingestion service.
`Gate::serve(listener, shutdown)` owns transport shutdown and drains work.
Custom hosts can call `close()` and then `drain()` before stopping their runtime.
Dropping a response observer does not cancel the owned body/storage task.
The body-read deadline stops acquisition; it never cancels a native append.

Defaults are 64 requests in flight, 1MiB acquired wire bytes per request and a
30-second body-read budget. Configure these through `GateOptions`. The gate
reports overload/closed arrivals as failures. An acquired prefix survives an
acquisition error or limit with associated `capture_incomplete=true`; complete
retention is never acknowledged for that prefix. Unavailable payload and a known
complete empty payload remain distinct. Content and record metadata are separate.

`stats()` exposes total request arrivals, active requests, retained/failed/unknown
requests, retained record count and closed state. These counters are in memory,
not a second audit stream. Protocol rejections and capture outcomes are separate;
record count is not the arrival denominator. Requests rejected by the HTTP parser
before its service callback are an observation limit. The gate does not claim
exact request counts for those parser failures or measured reliability nines.

## Validation

```sh
cargo test
cargo test --features gates
python3 tests/test_clients.py
cargo test --features gates --test polyglot -- --ignored
```

The last check requires Node and Python with grpcio. It drives both protocols
from both languages against a live listener, then checks exact native content,
receipts and request counts. The regular Rust gate checks cover framing, partial
content, response cancellation, limits and shutdown. See the
[custom adapter guide](adapters.md) for integration rules.
