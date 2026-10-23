# Capture contract — candidate for review

Current implementation scope is [the small native core](native-core.md).
The broader candidates below are research, including managed artifacts and
buffering; they are not requirements for this build. Power-loss resilience is
outside the current build.

2026-10-23. This develops the operator's requirements in
[gates and events](gates-and-events.md). It proposes an event shape and ownership
contract for pressure testing. It does not select frameworks, codecs, a storage
engine or dependencies, and it is not an implementation plan.

## Retain the content

Persist event contents only. There is no request archive, extra original-body
copy, or second capture layer. HTTP/gRPC request objects, headers and framing
are transport machinery rather than a second retained object. This is an explicit
operator requirement, not a configurable extra fidelity mode.

Producers normally submit their existing output without constructing Deleuze
events. Deleuze packs the record at the gate and associates the content's
properties. When normal conversion
cannot preserve some content, opaque bytes or a reference represent that content
instead. A conversion diagnostic alone cannot substitute for it. The same content
is not also archived beside its properties.

A gate may hold a content buffer temporarily while converting it. Success releases
the input buffer after ownership of the resulting properties is safe; failure
transfers the affected content into its opaque representation. Temporary input
ownership is not a second persisted copy. For large streamed content, the operation
can choose a content artifact/reference as its retained representation and keep
any decoded view derived. It must not persist both full representations.

## Record schema and associated properties

The operator clarified the logical model: an event has schema-defined record
fields, accompanied by arbitrary property key/value pairs. The property entries
are data associated with the record; they are not members of the record schema.
Adding or changing a property key does not add or change a schema field.

The previous `Event { properties }` proposal is withdrawn. The operator has
specified core fields: Deleuze assigns `seq` on write, computes `property_hash`
for integrity, and sets `created_at` for log write. `received_at` is a candidate
additional Deleuze-assigned gate timestamp. Producer timestamps stay in
properties; no other program sets record times. **Write sequence does not assign
any temporal ordering or causality to the described actions.**

Event types, producer IDs and support for producer-masked references are configurable features with
defaults. See [the record profile](record-profile.md) for core semantics, proposed
defaults and remaining choices. Integer width, time representation, hash
algorithm and property encoding remain undecided.

This distinction is logical. It does not prescribe inline versus referenced
storage, a separate table, another event stream, or a second persisted copy.
Record fields and associated properties participate in the same content capture
and retention contract.

Stored records and properties are immutable. A later assertion that something
does not exist never changes an earlier assertion that it exists. Corrections
append new records. The public log API exposes append/read/query, with no update,
delete or equivalent mutation path; current-state interpretation is a derived
view. See [append-only observations](record-profile.md#append-only-observations).

## Property representation remains a separate choice

Property names/values and their encoding need an explicit preservation rule.
The earlier ordered-entry container, recursive value union and managed-carrier
shape were proposals, not consequences of the record schema. Their adoption is
not established by this clarification. In particular, choosing record fields
and choosing property ordering/duplicate-name behavior are different decisions.

Properties cannot impersonate or overwrite schema-defined record fields. A
property whose key happens to match a record field remains property data. The
supported interface must make that separation unambiguous, without allocating a
new record-schema member for each property name.

A property representation must account for the content the producer supplied,
including producer-masked values, without silently
rounding numbers, replacing malformed text or losing distinctions its mapping
promises. Opaque content or references remain available where ordinary conversion
cannot preserve the content. These replace the affected content representation;
they do not add a request archive or another event.

Fallback must still produce a valid record under the chosen record schema and
associate the retained contents with it. Masking is the producer's responsibility;
Deleuze retains supplied masked content and offers configured ABAC-aware reference
assistance for authorized readers. It does not redact on ingest or fetch unmasked
source data to construct a fallback. Exact field encodings, extension policies and
fallback mappings remain to be designed. The earlier fallback
expressed as a one-field Event with a `content` property is withdrawn with the
one-field schema; it does not define a special new record shape.

The [protocol study](../papers/survey/gate-protocols/README.md) informs value
mapping and conversion risks. It does not choose the record schema, require a
particular property container, or prescribe the physical layout.

## Gate observation

HTTP, gRPC, native Rust and custom adapters all obtain the same capture capability.
They have no engine storage handles, public unchecked append operation, or public
sub-stages for bypassing the capture contract. Builders of data are distinct from
operators that mutate engine state. Internal record producers also use the
common operation; recursion/error reporting must not invent an unchecked writer.

A gate hands observed event content to the common operation before cancellable
or lossy content conversion can discard it. Format errors and application rejection
cannot redefine an observed arrival out of the capture population. The supported
adapter interface must keep ownership through those branches.

This obligation concerns event contents. It does not require retaining routing,
authentication headers, HTTP/2 framing, a parent RPC record or request-start/end
records. Transport failures that expose no event content must be reported honestly;
we do not fabricate payload events to simulate request archival.

The request's arrival still counts before content becomes available. A known
empty content value and absence of observed content are different cases. A gate
error cannot remove the arrival from the population or turn unavailable content
into a successful empty event. The completion/failure outcome can express this
without persisting another request record.

Each integration must name both the request-arrival observation point and its
first content observation, and explain how the operation bridges that interval.
Rejection before content is exposed still receives failed/unavailable accounting;
it neither disappears nor creates a fictional payload event. A callback that sees
only a decoded object cannot claim
to preserve distinctions already discarded upstream. Its supported content mapping
must be adequate for the promised property domain, or choose opaque content before
that loss. The exact framework hooks remain to be established.

For the first gate candidate, one HTTP event submission or unary RPC carries one
event. Streaming, if shipped, sends its event contents through the same operation;
partial contents and conversion failure still need truthful outcomes. It must not
add a second persisted history of the carrying RPC. Concrete stream/batch API
selection remains open.

## Ownership begins synchronously

The logical public operation starts capture at invocation and takes responsibility
for its input before reporting a successful handoff. It can accept native content,
owned encoded content or a readable stream through the supported interface. These
representations join one operation, not independent write paths. A real signature
and runtime integration remain to be designed.

The first content-observation callback invokes this operation synchronously with
owned content or its reader, before starting cancellable conversion. Successful
handoff also establishes an engine-owned execution lifetime. Merely moving bytes
into a task still owned by the caller's request scope is not sufficient. Failure
to establish that lifetime is reported synchronously with recoverable input where
possible; it remains a capture failure for the observed arrival.

The returned completion handle can be awaited or dropped. Dropping it,
disconnecting the client, reaching an RPC deadline or failing to write a response
does not cancel an already owned recording operation. A lazy future whose body
has never run cannot be the only gate-observation mechanism. Rust futures are
inert until polled; see the [primary documentation](https://doc.rust-lang.org/std/future/trait.Future.html).

The operation must keep an owner for the event and any acquired content chunks
through conversion, handoff, queueing, encoding, storage, failure and shutdown.
Conversion runs against operation-owned input; it cannot destroy the only content
representation before a successful conversion or opaque handoff. A decoder that
has already overwritten values cannot supply a later fallback. It cannot
retain a pointer into caller memory that may be reused when the call returns.
Native ownership transfer can avoid an extra copy; zero-copy opportunities must
be justified by actual lifetime and storage ownership, not an API label.

If resources prevent a successful ownership transfer, report capture failure
without relabeling the request as never having arrived. Preserve recoverable
owned input where the interface permits. A full queue, an exhausted fallback
store or process death remains within the funded reliability assessment.

Content acquisition and retention have separate cancellation rules. A disconnect,
deadline or configured acquisition limit may end further reads; already acquired
content remains engine-owned and continues toward retention. A stalled reader
must not hold resources forever by inheriting unlimited caller lifetime. Exact
acquisition budgets are deployment settings still to be designed. A budget cannot
relabel observed-but-discarded content as retained or incomplete content as complete.

Engine shutdown first stops ingress and accounts for races already observed by a
gate. It then drains owned operations or reports their failed/unknown outcomes.
Dropping a request runtime must not silently destroy engine-owned work. Process or
engine failure remains part of the stated failure model, not a cancellation
guarantee. The worker/runtime mechanism has not been selected.

Exhaustion handling is nonrecursive. The same operation keeps ownership while
waiting for funded capacity or ends with its actual failed/unknown outcome.
Reporting that failure cannot submit another failure event which recursively
fails through the same exhausted path. Ordinary events still use the one capture
operation; operational health/failure reporting need not create another event.
No diagnostic or counter counts as the missing content being retained.

## Retention and request interpretation are independent

Track what happened to the content independently of whether the transport or
event format was valid. The following are distinct facts, not one success flag:

| Fact | What it establishes |
| --- | --- |
| Observed | The request reached the named gate boundary, before decoding or successful acquisition. |
| Owned | The common operation holds the input/chunks it has acquired. This is not a durable-storage claim. |
| Retained | The promised content and its required references reached the configured persistence boundary. |
| Capture failed | Some required observed content was not retained; preserve known partial work and the fault where possible. |
| Outcome unknown | The record might have reached storage, but the observer cannot establish the result. A timeout is not proof of absence. |

Incomplete content must not be labeled complete. Conversion or cancellation
outcomes describe the same content capture; they do not create a second history
of the request or its response. Content the gate observed and then discarded
through its own limit remains a capture failure.

No success described as retained may be published before the configured storage
boundary. A storage write completion and a synchronized or replicated completion
are different conditions. Batch synchronization may complete several operations
together; this does not require per-event synchronization. Exact native return
types and HTTP/gRPC status mappings remain open.

Malformed event content can be retained successfully while the adapter returns an
error response. Valid content can fail capture. A lost reply can follow successful
retention. Every retry is a new arrival; a separate logical-operation identifier
must not deduplicate capture history.

Retention is an engine fact; unknown completion is observer-relative. They may
both apply when the engine retained content but the caller lost the reply. A
response send result does not establish what the client actually received.

## Large content

A path/URI in caller properties remains an ordinary retained value. It does not
silently trigger file reads, network fetches or a promise to keep its target.
An adapter that wants target bytes retained supplies those bytes through the
same capture operation, including its streaming form.

The engine may keep large owned content in an internal artifact and encode a
reference without materializing one giant in-memory value. Such a reference must
resolve to content owned under the same operation and retention policy; callers
cannot manufacture a trusted managed reference by supplying a string. The public
carrier and artifact format are unresolved.

For owned content, publish retained completion only after both the content and
the event/reference reach the promised boundary. Interrupted event publication
may leave an unreferenced artifact for later safe reclamation; a published record
must not falsely promise an artifact that has not been retained. Cancellation and
restart must preserve ownership or report the actual failed/unknown outcome.

Managed content is immutable. It remains protected while any in-flight or
uncertain event publication may reference it. Recovery must resolve publication
uncertainty before classifying content as unreferenced and reclaiming it; lack of
a visible event at one instant is not sufficient. Reference construction remains
internal, and reader access cannot mutate the stored content.

## Next design obligations

The candidate needs adversarial review of its representation and ownership rules.
Then choose concrete gate observation hooks, the value/extension encoding,
resource/fallback behavior, managed-artifact lifecycle and completion interface.
Those choices must be compared against the direct-append and buffered-recorder
alternatives in the [starting point](starting-point.md), with the same arrival
population and failure model. Only a reviewed design and reviewed implementation
plan authorize production code under the working doctrine.
