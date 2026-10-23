# Batched synchronization review

2026-10-23. Independent design/plan review accepted standard bounded group
commit with direct Written calls and explicit content batches. It required
Unknown after successful handoff when a reply disappears, rather than claiming
NotWritten. Count/byte targets, original queue age, permits and shutdown ownership
remain explicit; no new file/security/recovery protocol.

Source review found no ownership/publication/lifecycle defect. Final test review
checked one barrier for many frames, no early completion, short vectored writes,
failed synchronization without publication, attempted-group Unknown and untouched
queued NotWritten, accepted reply loss, permit release, aged-ready collection,
carry-over age, low-load waiting and closed-channel drain with missing observers.
Public batch tests preserve original contents/results, metadata, order, sequence
exhaustion, oversized singleton handling, clean reopen and file-lock release.
No concrete blocker remained. The test run reported 55 runtime tests and 3
compile-fail doctests passing; live Python/Node checks cover both completions.

The benchmark client separately received four direct lifecycle fixtures. Two
original cases ran red before correction: retired-session future offers and
pre-stream rejection classification. Corrected network trials preserve unknown
after stream handoff and report session/error counts; no retries or raw request
archive were added.

Independent result review recomputed 75 native, 30 corrected network and 42
historical probe rows. Counts, rates, whole-call quantiles, storage arithmetic and
uncertainty totals matched. One setup sentence was corrected to distinguish
Synced 256-call groups from Written sample counts. No numeric or unsupported
causal/reliability claim remained.
