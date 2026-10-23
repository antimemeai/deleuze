# Public protocol and boundary reading

2026-10-23. Eight retained primary originals inform the
[capture-contract candidate](../../../design/capture-contract.md). This is a
targeted study of event-content conversion and ownership, not a request-archival
proposal. The later [bundled-gate implementation](../../../design/gate-implementation.md)
selects the optional runtime/transport stack; this source ledger predates it.

The later operator clarification separates record fields from associated property
data. Earlier examples involving an outer `properties` member concerned a
provisional transport shape, not a chosen record-schema field. The one-field
event proposal is withdrawn; value/container choices remain open.

| Source | Selected reading and relevance |
| --- | --- |
| [gRPC over HTTP/2](https://grpc.github.io/grpc/core/md_doc__p_r_o_t_o_c_o_l-_h_t_t_p2.html) | Request/message framing, compression and failures. Message boundaries differ from DATA-frame boundaries and compression contexts are per message. This informs extraction of event content; it does not require retaining transport framing or headers. |
| [Protobuf encoding](https://protobuf.dev/programming-guides/encoding/) | Repeated singular values, maps and ordering. A decoded object follows specified merge/last-value semantics and need not preserve original serialization. The relevant requirement is the event's declared content mapping, not a history of all wire occurrences. |
| [ProtoJSON](https://protobuf.dev/programming-guides/json/) | Duplicate keys, aliases, null, numbers and presence. Generic parsing can erase content distinctions needed by the candidate event type. Unknown or duplicate outer fields need an explicit normal mapping or content fallback. |
| [RFC 8259](https://www.rfc-editor.org/rfc/rfc8259.html) | Objects, numbers, strings and parser limits. Duplicate-name behavior varies; numeric precision and malformed Unicode need declared interpretation. An unrepresentable property name can require fallback for its enclosing content container. |
| [Struct/Value source](https://raw.githubusercontent.com/protocolbuffers/protobuf/main/src/google/protobuf/struct.proto) | Actual declarations read. Struct uses a map; Value has a double number branch and no distinct integer or byte branch. That domain alone does not cover the proposed ordered, duplicate-preserving typed properties. |
| [Rust visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) | Default visibility, access and exports. Supported API boundaries need private engine operations and careful re-exports. Private wrappers also need ownership that does not leak raw state handles. This is an API boundary, not a claim of process isolation. |
| [Rust Future](https://doc.rust-lang.org/std/future/trait.Future.html) | Inert futures and polling. The native capture operation must establish observation and ownership before returning a droppable completion observer; a never-polled future cannot be the sole ingress mechanism. |
| [RFC 9110](https://www.rfc-editor.org/rfc/rfc9110.html#name-202-accepted) | Selected §15.3.3 only. HTTP 202 is noncommittal about eventual processing; a protocol status does not independently define Deleuze's retention boundary. Exact endpoint/status choices are open. |

These are bounded section/source readings. No transport parser, generated codec,
Rust compiler behavior or server was tested. No entire HTTP specification was
read. The failure cases below are review inputs rather than reproduced bugs.

## Content cases used in review

- Duplicate property entries and escaped-equivalent names; no generic object-map
  conversion may discard entries promised by the event contract.
- An integer beyond binary64's exact range, unsigned-64 maximum, a large decimal,
  signed zero and distinct floating-point widths/NaN bit patterns.
- Invalid UTF-8 or an escaped unpaired surrogate in a value or property name;
  value-only fallback cannot repair an invalid name slot.
- Unknown outer fields, repeated outer `properties` fields, null versus an
  explicitly empty property sequence, and late conversion failure.
- Invalid/unsupported content compression and expansion limits. Keep enough
  representation information with affected content to interpret its encoded form;
  do not archive the transport request for that purpose.
- No content observed versus known-empty content, incomplete acquisition and
  successful retention followed by a lost completion reply.

All cases preserve the operator's contents-only boundary. Normalization already
defined by a declared transport/content mapping is not automatically loss of
event content. An adapter must not promise distinctions its decoder has already
discarded. Opaque representation replaces affected content; it does not add a
second persisted request or duplicate event.

## Originals and restoration

[bibliography.tsv](bibliography.tsv) records each primary URL, acquisition time,
resolved URL, original path, byte count, SHA-256 and reading depth. Original HTML,
text and the source definition are ignored under
`quarantine/archives/gate-protocols/`. Public copyright/license notices remain in
the originals; no redistribution license is inferred. Living documentation is
identified by retained bytes, not an assumption that its URL stays unchanged.

To restore an original, fetch its recorded primary URL into its recorded local
path and compare SHA-256. A mismatch requires investigation; do not silently
replace the manifest's hash. This acquisition supplies documentation, not an
executable build closure or a dependency decision.
