# Batched writes and synchronized completion

2026-10-23. Operator: synchronization should definitely be batched. The first
baseline measured roughly594k/s written versus238/s with a barrier after each
128-byte event. Use ordinary group commit, with one immutable record per event.
No new dependency, file format, security protocol or power-loss subsystem.

## Completion and public surface

Replace the unpublished `Options::sync_each_append` flag with
`completion: Completion::{Written, Synced}` and `batch: BatchOptions`.
Written remains the default complete-file-write boundary. Synced success follows
one successful ordinary File::sync_all covering the record's batch. Do not return
an enqueue acknowledgment as either completion. Errors still return owned input.

Keep Log::append for ordinary content. Add
`append_batch(Vec<Content>) -> Vec<Result<Record, AppendError>>`, preserving result
order and an outcome for every supplied item. Empty input returns empty. A batch
is not a transaction or a new persisted event; validation failures do not erase
other valid arrivals. Bound packing/submission chunks using the batch limits;
never encode an arbitrarily large input vector all at once. The original input
is retained by the caller operation until each result is known, while an owned
encoded frame goes to storage. On failure, return the corresponding Content.

Defaults: max_records256, max_bytes1MiB, max_delay1ms, max_pending_batches1024,
max_pending_bytes128MiB. Limits are configurable and validated for zero/overflow.
A valid record larger than the target batch-byte limit travels alone, subject
to existing content bounds and pending-byte capacity. Queue limits count active
and queued encoded work as documented; they are not an exact process-RSS bound.

## One storage path, ordinary ownership

Use one internal bounded std-channel writer only in Synced mode. It owns encoded
submissions, batches by count/bytes/oldest enqueue age, then invokes the shared
native batch-write routine and completes waiters. Drain already-ready submissions up to the count/byte caps even when their oldest
queue age exceeds the delay (for example during a previous sync). The deadline
bounds waiting for additional work when the queue is empty, not draining ready
work. Do not reset the oldest collection deadline on each new submission; bounded
caps prevent a continuous stream from postponing flush forever. Delay bounds intentional collection,
not I/O time or scheduler delay. A group that does not fit is carried to the next
batch; retain ownership, order and its original age.

Written append keeps its caller-thread fast path with no worker hop. Written
explicit batches use the same batch-write routine with no sync. Synced append
submits one frame; independent concurrent calls and all HTTP/gRPC appends can
coalesce automatically. Synced append_batch fills groups even with one producer.
Sequential blocking append calls from one producer cannot coalesce future calls
that have not arrived. No adapter-specific writer or alternate mutation path.

Keep the lifetime file lock and writer mutex. Store the writer behind shared
private ownership so the worker and read snapshots refer to the same file/state.
Drop closes the worker channel, drains owned work and joins it before releasing
storage. The worker must not own Log itself and create a reference cycle. A
reply receiver disappearing never cancels its owned record. A failed worker or
closed/full queue reports stopped/full with recoverable input; do not leave
waiters blocked forever. Once handoff succeeds, a lost worker reply is Unknown,
unless an explicit worker result proves no I/O occurred. Rejected handoffs stay
NotWritten. Use byte permits for queued/in-flight encoded frames,
released on every success, failure and rejected handoff.

## Records, writes and ordinary errors

Pack/hash each record independently using the existing binary encoding and
ordinary BLAKE3 API. Assign seq only in the writer immediately before the actual
write attempt, in frame order. Sample Deleuze write time there; members may share
a batch-write timestamp. This orders log writes only, not actions or arrival.
Producer times remain properties. No batch digest, parent event or request copy.

Write complete existing frames together using std vectored I/O with short-write,
Interrupted and WriteZero handling. Then call sync_all once in Synced mode.
Publish the new readable prefix and successful results only after the selected
boundary succeeds. On an attempted batch write/sync error, stop the writer and
report Unknown for the attempted group; subsequent untouched queued groups get
NotWritten/Stopped. Never auto-retry. Sharing an ordinary I/O error across group
results may use Arc ownership; no extra error framework. Pre-write failures,
clock/sequence exhaustion and queue capacity errors remain NotWritten. Do not
wrap sequence or consume numbers for rejected inputs.

The existing file format remains readable. Clean reopen and strict unchanged
rejection of incomplete/corrupt tails remain; no repair/truncate path is added.
Read/query snapshots use only the published prefix. Failed/ambiguous writes are
never exposed as successfully synchronized records.

## Plan and direct validation

After independent design/plan review: implement shared multi-frame storage writes
and explicit content batches, then the bounded group-commit worker and lifecycle,
then update native/gate examples and measurement configuration. Preserve the old
written and single-barrier benchmark results as historical data. No new crate.

Use the existing private scripted I/O seam to assert one barrier covers many
frames, no acknowledgment/publication before that barrier, exact frame/hash/seq
contents, short vectored writes/interruption and write/sync failures. Test oldest
age and count/byte cutoffs, carry-over ownership, queue byte/full rejection,
completion order, worker close/disappeared observers, clock/sequence boundaries
and immutable clean reopen. Regression tests must fail on the old path where
applicable. Keep scope to ordinary operation and I/O, with no crash protocol.

Extend native measurement with explicit producer batch size and batch limits;
record per-event preparation-to-completion latency and exact outcomes. Compare
single-producer explicit batches against matching frame-sized file batches, and
concurrent single appends with automatic group commit. Re-run real HTTP/gRPC
Synced load at the same offered rates/capacity accounting. Report batch settings,
throughput and tail latency; do not multiply the old238/s by a batch size and
present it as a measurement. Keep written-mode regression checks and commit/push
coherent milestones frequently.

## Public grounding

The selected reading of Aether (§2.1, retained original under
quarantine/sources/survey-durable/text/aether.txt) distinguishes grouping flushes
from early acknowledgment and notes context-switch costs remain. The pinned
Apache BookKeeper Journal ForceWriteThread groups pending force-write requests,
synchronizes through the last, then sends completions. Its replication/storage
protocol is not imported. Source revision and restoration are in
[papers/survey/durable-logs/references-manifest.md](../papers/survey/durable-logs/references-manifest.md).
