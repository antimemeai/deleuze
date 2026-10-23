# Gate packing, core record fields and configurable defaults

The operator clarified that producers usually send existing output rather than
constructing events. Deleuze packs it at the gate. It assigns monotonically
increasing seq on write, computes property_hash for integrity and supplies
created_at for log write; received_at is a possible gate timestamp. Sequence is
write order, emphatically not temporal action order or causality. Producer time
values remain properties; only Deleuze sets record timestamps.

The operator also requested configurable event types, producer IDs and cooperation
with masked fields, with defaults. The operator then clarified that producers
perform masking; Deleuze only assists authorized readers in reaching the underlying
data when their ABAC permits. Wrote design/record-profile.md with those requirements
and explicitly proposed off defaults for optional features. Defaults are proposals,
not attributed to the operator. No integer width, hash, timestamp encoding,
property encoding, dependency or physical storage model was selected.

Documented consequences: unclassified/unattributed content remains capturable;
configuration applies through the common path; Deleuze stores the producer's
masked representation, with ABAC-aware reference assistance when configured;
the digest describes retained properties and is not a deduplication key. Sequence gaps/scope/recovery and exact
clock behavior remain design work. Updated active contract and orientation to
remove older statements treating these core fields as merely possible.

No production code, source-specific private details or request archive was added.
The stopped research CLI was not restarted.

The initial ingest-time masking proposal was an assistant misunderstanding and
was removed before publication. Deleuze does not scan/redact PII at ingest, fetch
unmasked source data for fallback, or rewrite stored masked values after an
authorized read. Reference assistance is a configurable integration, not a selected
policy engine. Its proposed default is passive storage with resolution off.
