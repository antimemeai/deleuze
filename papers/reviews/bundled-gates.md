# Bundled gate review

2026-10-23. Independent design/plan review accepted bounded owned acquisition
followed by one blocking operation per request through public Log::append.
It required explicit unknown > failed > retained aggregation; successful later
messages cannot erase earlier failures.

Client review reproduced two bugs: case-sensitive HTTP receipt names and Node
accepting framed gRPC success without status trailers. Direct fixtures failed
before correction and passed afterward.

Source review and direct regressions corrected HTTP route misclassification,
case-sensitive media types, discarded content-coding identifiers, outer HTTP
compression being mistaken for gRPC framing, and falsely labeling flag0 message
bytes as compressed. Six gate regressions failed before their fixes. Opaque
encoded entities replace undecodable content; no request/body duplicate or
decompression subsystem was introduced.

Closure review read the revised paths and assertions and found no remaining
concrete blocker. Native metadata stays private, all payloads use the same append
operation, record count and request outcomes remain separate, and shutdown drains
owned operations. The HTTP parser's pre-service rejection coverage and offered-load
performance remain subsequent measurement work; review establishes neither nines
nor throughput.
