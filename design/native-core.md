# Small first native core

The following describes the first direct-write build. Its sync flag and
per-record barrier policy are superseded by [batched sync](batched-sync.md).
The record/property encoding and append-only file format remain unchanged.

2026-10-23. Operator-directed scope reduction, independently reviewed and implemented in the first native build. Official Rust BLAKE3 1.8.7 is the sole direct dependency.

## Scope and ownership

One append-only binary file, one private writer mutex, and a lifetime exclusive standard-library file lock. No segments, predecessor links, epochs, durable reservations, directory-sync protocol, background queue, blob store or recovery subsystem. Power-loss recovery is outside this first slice; ordinary process execution, short writes and I/O errors are in scope. This core does not claim deployment nines or final throughput.

Producers supply ordinary owned text, bytes or properties. Deleuze packs them and assigns the three core fields. Everything uses the same synchronous append operation. No public file handles, unchecked stages, update/delete/truncate/clear methods or rewriting compaction. Later contradictions append new records. HTTP/gRPC/custom gates remain separate adapter work against this operation.

## Proposed native API

```rust
pub struct Log { /* private state and locked file */ }
pub struct Options { pub sync_each_append: bool, pub max_content_bytes: u64,
    pub max_depth: u32, pub max_query_records: u32, pub max_query_bytes: u64 }
pub enum Content { Text(String), Bytes(Vec<u8>), Properties(Properties),
    Encoded { representation: String, bytes: Vec<u8> } }
pub type Properties = Vec<(String, Value)>;
pub enum Value { Null, Bool(bool), I64(i64), U64(u64), F32Bits(u32),
    F64Bits(u64), Text(String), Bytes(Vec<u8>), List(Vec<Value>),
    Properties(Properties), Encoded { representation: String, bytes: Vec<u8> } }
pub struct Record { seq: u64, property_hash: [u8; 32], created_at: WriteTime }
pub struct WriteTime { unix_seconds: i64, nanos: u32 }
pub struct Entry { pub record: Record, pub properties: Properties }
pub struct Query { pub after: Option<u64>, pub through: Option<u64>, pub limit: u32 }
pub struct Page { pub entries: Vec<Entry>, pub next_after: Option<u64> }
impl Log {
    pub fn open(path: impl AsRef<std::path::Path>, options: Options) -> Result<Self, Error>;
    pub fn append(&self, content: impl Into<Content>) -> Result<Record, AppendError>;
    pub fn read(&self, seq: u64) -> Result<Option<Entry>, Error>;
    pub fn query(&self, query: Query) -> Result<Page, Error>;
}
```

Provide `From<String>`, `From<Vec<u8>>` and `From<Properties>` for `Content`. `Record`/`WriteTime` have private fields, internal constructors and read-only getters; no timestamp setters or external construction/deserialization constructors. Producer timestamps, including properties named `created_at`, remain properties. Types, producer IDs, `received_at` and reference assistance are off/absent. Producers mask data; paths/URIs remain supplied values, with no fetching or ingest redaction.

## Packing and file format

Text/bytes/encoded content maps to one associated property named `content`; properties input retains its entries directly. Preserve entry order, duplicate names, float bits, binary values and distinct empty values. Opaque encoded content replaces content outside the native mapping; no request archive, duplicate body or extra event stream.

Encode properties deterministically: `u64` count; each key is `u64` UTF-8 byte length plus bytes; values have a one-byte tag (enum order 0–10). Fixed numbers are little-endian; bool is 0/1; text/bytes have `u64` byte lengths; lists/property sequences have `u64` counts; encoded values contain length-delimited representation text and bytes. No sorting, normalization or number-to-string conversion. Reject malformed encodings on read.

`max_content_bytes` bounds the entire encoded properties, including keys/tags/lengths, not merely original input bytes. Propose 16 MiB default and native packing depth 128. The format maximum native nesting depth is 128; an ingest option may tighten it without making older valid records corrupt. Opaque/text/byte contents are not interpreted or depth-checked. Return oversized/deep native input as capture failure rather than replacing it with a diagnostic.

`property_hash = *blake3::hash(encoded_properties).as_bytes()`: exactly the retained property bytes and the ordinary 32-byte API, without custom prefixes, keys, chains or metadata concatenation. Equal hashes do not deduplicate arrivals. The first build pins the official crate release 1.8.7 with its default build settings. [Official API](https://docs.rs/blake3/latest/blake3/fn.hash.html)

The file starts with `DLZLOG01` (8 bytes), version `u16=1` and flags `u16=0`. A record is `DLZR` (4), property length `u64`, sequence `u64`, Unix seconds `i64`, nanoseconds `u32`, property hash (32), encoded properties, then `ENDR` (4) and total frame length `u64`. Integers are little-endian; fixed frame overhead is 76 bytes. No padding or native struct dumps.

Validate the fixed 64-byte record header, expected sequence, normalized time, checked lengths, available bytes and configured capacity before payload allocation. Then verify property hash, canonical decoding and trailer. Header validation here is structural: this first slice has no metadata checksum and does not claim detection of every metadata bit change. Add a conventional CRC32C only if independent review establishes a need; no checksum dependency is adopted. No authentication, signing or anchoring scheme.

## Append, reopen and errors

Require Rust 1.89+ for `File::try_lock`. Create the file exclusively when new; otherwise open without truncation. Acquire the lifetime exclusive lock before initializing/scanning; return `Busy` for another owner. Do not export/clone the lock handle. Initial target: tested local Linux/macOS filesystems; locking is cooperative, not protection from arbitrary external file mutation. [Standard locking](https://doc.rust-lang.org/std/fs/struct.File.html#method.try_lock)

On clean reopen, strictly scan the entire file and recover the last complete valid `seq`; next is checked `last + 1`, starting at 1 for an empty valid log. Reject overflow. An incomplete or corrupt tail is a clear open error; leave every byte untouched. Do not truncate, repair, resynchronize at later magic or append beyond damage. No power-loss guarantees about sequence reuse, lost suffixes or recovery are made.

Append takes ownership synchronously, packs bounded content, and writes under the mutex. Assign the next sequence at the write attempt; sample Deleuze's `SystemTime::now()` immediately before finalizing/writing the frame. `created_at` is this write-start observation, not occurrence, receipt or sync time. Preserve clock regressions; use normalized signed Unix seconds/nanoseconds, and fail cleanly on conversion overflow. Sequence expresses file-write order only, never action chronology or causality.

Use `write_all`; short writes and interruptions must be handled, and errors must not be mistaken for complete writes. If configured, call ordinary `File::sync_all` before returning success. Propose `sync_each_append=false` for the direct-write baseline; it promises successful complete writes, not synchronized storage. The enabled mode reports successful file synchronization, without power-loss promises or directory durability machinery. Close/drop is not a substitute for an explicitly configured sync. [Standard synchronization](https://doc.rust-lang.org/std/fs/struct.File.html#method.sync_all)

A write/sync error stops all later appends in that instance. `AppendError` returns the owned content, underlying reason and whether no frame I/O occurred or the result is unknown. Once frame I/O begins, conservatively report unknown on error. Do not auto-retry or emit a failure event; a caller retry is another arrival. Size/allocation/clock failures before writing leave the file and next sequence unchanged. Full/failing arrivals remain capture failures, not exclusions from the population. Use small ordinary errors distinguishing busy, invalid input/options, capacity, corruption with offset, stopped writer and I/O; no additional error subsystem.

## Reads and proportionate validation

Startup/read/query are linear scans. Capture the published byte boundary under the mutex, then release it before scanning private read handles. Never expose a concurrent partial append or hold the writer mutex for a full query. Capacity errors are distinct from corruption. Query uses exclusive `after`, inclusive `through`, increasing sequence and a record limit; defaults cap results at 1,024 entries and 16 MiB encoded returned properties. The byte cap does not claim exact heap or scan-cost bounds.

Return `Page { entries, next_after }`; continue after the last returned sequence. Leave a next entry that would exceed a nonempty page's budget for the next call. If the first matching entry exceeds the entire budget, return `ResourceLimit` identifying its sequence and required bytes; never skip it or return a non-advancing page. Return no continuation at observed EOF; an exactly full page may conservatively require one empty terminal call. No index/cursor infrastructure or multi-page snapshot promise.

The reviewed implementation plan used three steps: (1) native packing and strict binary parsing, (2) locked append/clean reopen and stop-on-error behavior, (3) linear reads and result-bounded pages. Use hand-assembled golden fixtures and official BLAKE3 vectors, exact content/metadata assertions and compile-fail checks for private timestamps. A private scripted I/O test boundary exercises short writes, interruptions, write/sync errors and refusal of later appends. Test clean restart, incomplete/corrupt-file rejection without mutation, second-owner locking, clock regression, sequence exhaustion, length/depth limits, pagination and concurrent snapshot reads. No power-loss simulation, exhaustive crash protocol or additional evidence layer.

Measure direct-write and sync-enabled modes separately against the same-boundary file baseline, reporting content size, producer count, tail latency, retained/failed/unknown/pending arrivals and cost. This small core is a measurable starting point; no performance result or reliability nines have yet been established.
