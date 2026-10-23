# Gate-packed records and configurable features

2026-10-23. Operator requirements and proposed feature defaults. Integer widths,
time encoding and persistence choices for the first build are specified in
[native-core.md](native-core.md); optional profile extensions remain proposals.

## Deleuze packs the record

Producers normally submit whatever content they already produce. They do not
have to construct Deleuze events or supply record metadata. HTTP, gRPC, native
Rust and custom adapters feed that content into one Deleuze packing path at the
gate. Deleuze associates properties with a small, strictly schematized record.
The property pairs are data outside that record schema.

A producer may already emit a structured event or log record. It is still input
content: its fields do not acquire authority over Deleuze's record fields. The
native interface cannot provide a bypass to set write sequence, property hashes
or authoritative record times.

## Core record fields

| Field | Meaning and authority |
| --- | --- |
| `seq` | Deleuze assigns an increasing sequence on record write. It identifies write position/order in the log's sequence domain. Integers are the leading candidate; width and other representations remain open. |
| `property_hash` | Deleuze computes an integrity digest of the associated properties under their defined retained representation. It is a record field; the property pairs are not. |
| `created_at` | Deleuze's timestamp for log write. It is not producer occurrence time, gate receipt time or a synchronization-completion timestamp. |

`received_at` is a candidate additional timestamp for observation at the gate;
its inclusion is not settled. If present, Deleuze assigns it too.

Only Deleuze may populate record timestamps. Producer-supplied time values stay
in properties, including values named `created_at` or `received_at`. No option
may promote a producer timestamp into an authoritative record timestamp. Caller
data cannot overwrite `seq` or `property_hash` either.

**Write sequence does not order actions in time.** A lower `seq` means earlier
position in the log's write sequence. It does not mean the described action
occurred earlier, caused another action, or reached a gate earlier. Concurrency,
queues and delayed producer output must not become claims of action chronology.

Monotonic sequencing does not itself promise gap-free numbering. Scope across
writers/logs, restart recovery, failed writes and exhaustion need concrete rules.
The sequence must not silently become per-gate or wrap/reuse values contrary to
its promised scope. Clock observations need not be monotonic. Timestamp units,
precision, clock source, sampling point and clock-failure behavior remain open;
cross-clock subtraction does not automatically measure queue latency.

## Append-only observations

Stored records and their associated properties are immutable. If record A says
something exists and record B says it no longer exists, retain both as written.
The assertions concern the observations/times described by their content; a later
assertion does not modify an earlier record. Log sequence still means write order.

The public log API exposes append, read and query operations only. There are no
update, delete, overwrite, truncate, clear, retention-prune or history-rewriting
compaction methods, including aliases or raw storage handles that provide the
same capabilities. Corrections, retractions and later state are new records.
Record fields are read-only through the public interface.

Current-state calculations are derived views over that history. They may choose
how to interpret later assertions but cannot write those interpretations back
into earlier records. ABAC-aware reference assistance likewise leaves the stored
masked properties, digest, sequence and timestamps unchanged.

The first build reopens clean files and rejects incomplete or corrupt files
without changing them. It exposes no repair bypass. Power-loss resilience is
outside the current build.

## Configurable features with defaults

The operator requested toggles for event types, producer IDs and cooperation
with producer-masked data.
These defaults are proposals that let ordinary producers submit content without
supplying metadata or choosing a taxonomy.

| Feature | Proposed default | When enabled |
| --- | --- | --- |
| Event types | Off | A configured Deleuze mapping assigns the schema's event-type field, including a defined unclassified outcome. |
| Producer IDs | Off | Deleuze resolves the schema's producer-identity field using the configured mapping, including a defined unattributed outcome. |
| Masked-data reference support | Passive storage; automatic reference assistance off | Interpret the configured producer mask/reference convention and point readers to underlying data only when the configured ABAC authorizer permits it. |
| `received_at`, if adopted | Off | Deleuze samples its own gate-observation time. Producer time values remain properties. |

The core fields remain core; these toggles do not disable `seq`, `property_hash`
or `created_at`. Extension field names/types remain open. Enabling types or IDs
must not require ordinary producers to manufacture Deleuze records before sending
content. Unclassified/unattributed input still needs a valid record.

Configuration applies in the common Deleuze path. Adapters cannot bypass it or
override it per request through properties. These are configuration features,
not a decision to implement them as Cargo compile-time flags. Within a configured
schema/profile, record fields are fixed. Effective settings and defaults must be
available for interpreting retained data; changes must not silently alter layouts
or field meanings. Profile storage and change boundaries still need design.

## Integrity and producer-masked data

The producer is responsible for masking data before submission. Deleuze records
the supplied masked representation and associated properties/references. It does
not inspect for PII, redact fields, or repair a producer's masking policy on
ingest. The earlier ingest-time masking proposal was a misunderstanding and is
withdrawn.

`property_hash` covers the properties Deleuze actually retains, including the
producer's masked values and any references. It is not a digest of unmasked source
data fetched through a reference. Algorithm, digest encoding, property ordering
and canonicalization still need design. Equal property data may have equal hashes
on distinct records; the hash does not deduplicate arrivals. A reference hash does
not itself verify its target bytes or authenticate the other record fields.

Deleuze cooperates on read: with the relevant feature and integrations configured,
it can point an authorized reader to the underlying data when that reader's ABAC
is sufficient. Authorization comes from the configured ABAC integration, not from
a property claiming access rights. Without an affirmative authorization, the
reader keeps the masked view and receives no privileged resolution assistance.

The default is passive storage of supplied masked values and references, with no
automatic resolution. Enabling assistance requires a configured reference mapping
and ABAC authorizer. Concrete interfaces, pointer-disclosure rules and defaults
within that integration remain design work; no policy engine or dependency is
selected here.

Pointing to underlying data does not make Deleuze its owner or require Deleuze to
fetch, duplicate or persist it. Authorized access does not rewrite the original
record, replace its masked properties or change its property hash. There is no
request archive or additional unmasked copy in this design.

## Packing and write responsibilities

The gate owns submitted content and performs Deleuze packing under the effective
configuration. At write, Deleuze finalizes `seq` and `created_at`; the digest
corresponds to the final retained property representation. It may be computed
during packing rather than requiring another full pass.

Record fields, associated properties and any promised owned content share one
retention outcome. Capture remains contents-only through one operation, without
preserving producer request objects.
