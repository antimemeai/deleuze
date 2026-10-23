# Gate and event model clarification

The operator specified HTTP and gRPC gates plus a custom-adapter interface and
guide. The promise begins when a request reaches a gate. All integrations share
one path with a strict boundary against reaching into internals. Content is
arbitrary properties on an extremely small, strictly schematized event envelope;
whatever arrives must have a way to be persisted. Large content may use property
references.

Recorded these requirements in design/gates-and-events.md and updated the project
orientation. Distinguished gate observation from later successful decoding or
completion, opaque fallback from a diagnostic without retained content, and a
stored reference from an explicit promise to retain its target. Exact fields,
property types, codecs, frameworks, storage and acknowledgement semantics remain
open design questions. No dependencies or production code were added.

Private reading informed generic boundary principles only. No source identities,
provenance, quotations or source-specific findings are recorded here. The private
material remains outside the checkout and the stopped research CLI stays stopped.
