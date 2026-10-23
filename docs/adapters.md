# Writing an adapter

An adapter delivers producer content to `Log::append`. It does not construct a
record, choose a sequence, set record timestamps or write storage directly.
Use owned text, bytes, native properties or explicitly labelled opaque bytes.
Producer timestamps and identifiers stay in properties. Paths and URIs stay
ordinary supplied values; Deleuze does not fetch their targets. Producers apply
their own masks.

Choose the content mapping at the boundary. For example, a text-emitting tool
can send its text directly; a binary protocol can supply its payload bytes. A
structured adapter should preserve distinctions its declared mapping supports.
If parsing would lose content, retain opaque content instead. Retain the content
once, without an additional copy of the request, headers or lifecycle history.

Call the same public append operation as a native Rust producer. The returned
record describes a log write, never the time or order of the producer's action.
A producer property named `seq` or `created_at` has no authority over that record.
Later assertions use another append; there is no update or deletion interface.

For an asynchronous gate, give the content an owner that outlives response
cancellation before handing work to a worker. Do not let a disconnected client
implicitly cancel an append already in progress. A lost response does not mean
a failed write. Report the actual storage result when known, and uncertainty
when it is unknown. Do not retry automatically: a caller retry is another
arrival, including when its content has the same property hash.

Declare where the gate observes input, its size limits, and what it reports if
reading content fails. Full/failing-gate arrivals remain capture failures in
measurement. Do not silently count only accepted or successfully parsed input.
An incomplete observed payload is still content; status and storage outcomes
are separate. Gate limits do not establish infinite capacity.

Append completion means the configured native boundary: a completed file write,
or a completed batch write followed by `File::sync_all` in Synced mode.
Concurrent adapter submissions can share that synchronization. Neither is a
claim about arbitrary hardware failures. Power-loss resilience is outside the
current build. An append error returns the owned content and distinguishes
pre-write failure from an uncertain write result. Adapters must preserve that
distinction instead of inventing a definitive absence after an I/O error.

The bundled HTTP/gRPC gates use this same append operation. See
[the gate guide](gates.md) for running the optional service and Python/Node
clients, and [the implementation design](../design/gate-implementation.md) for
request ownership, bounds and outcome aggregation.
