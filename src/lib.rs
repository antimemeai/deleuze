//! A small synchronous, append-only content log.
//!
//! Deleuze assigns write sequence, property hash and write-start time. Producer
//! times remain properties. Sequence orders file writes, not actions or causes.
//! The default completes full writes without synchronizing each record; enabling
//! [`Options::sync_each_append`] additionally calls [`std::fs::File::sync_all`].
//! Neither mode promises power-loss recovery. Damaged files are rejected unchanged.
//!
//! All fields of [`Record`] and [`WriteTime`] are read-only through the API.
//! ```compile_fail
//! let time = deleuze::WriteTime { unix_seconds: 0, nanos: 0 };
//! ```
//! ```compile_fail
//! fn change(record: &mut deleuze::Record) { record.seq = 0; }
//! ```
//! ```compile_fail
//! fn change(time: &mut deleuze::WriteTime) { time.nanos = 1; }
//! ```

mod codec;
mod storage;

use std::fmt;
use std::io;
use std::path::Path;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Ordered property entries. Duplicate names are preserved.
pub type Properties = Vec<(String, Value)>;

/// Owned content; Deleuze packs ordinary output into associated properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content {
    Text(String),
    Bytes(Vec<u8>),
    Properties(Properties),
    Encoded {
        representation: String,
        bytes: Vec<u8>,
    },
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}
impl From<Vec<u8>> for Content {
    fn from(value: Vec<u8>) -> Self {
        Self::Bytes(value)
    }
}
impl From<Properties> for Content {
    fn from(value: Properties) -> Self {
        Self::Properties(value)
    }
}

/// Native values; opaque encoded bytes are never interpreted or fetched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F32Bits(u32),
    F64Bits(u64),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Properties(Properties),
    Encoded {
        representation: String,
        bytes: Vec<u8>,
    },
}

/// Resource bounds and append completion behavior.
#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub sync_each_append: bool,
    /// Entire encoded property bytes, including names, lengths and type tags.
    pub max_content_bytes: u64,
    /// Native container nesting at ingest, at most the format maximum of 128.
    /// Text, byte and opaque contents are not parsed.
    pub max_depth: u32,
    pub max_query_records: u32,
    /// Sum of encoded property bytes returned per page, not exact heap usage.
    pub max_query_bytes: u64,
}
impl Default for Options {
    fn default() -> Self {
        Self {
            sync_each_append: false,
            max_content_bytes: 16 * 1024 * 1024,
            max_depth: 128,
            max_query_records: 1024,
            max_query_bytes: 16 * 1024 * 1024,
        }
    }
}
impl Options {
    fn validate(self) -> Result<Self, Error> {
        if self.max_content_bytes < 8
            || self.max_depth > 128
            || self.max_query_records == 0
            || self.max_query_bytes == 0
        {
            return Err(Error::InvalidOptions(
                "require content bound >= 8, depth <= 128 and nonzero query bounds",
            ));
        }
        Ok(self)
    }
}

/// Deleuze's clock observation immediately before a frame write starts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteTime {
    unix_seconds: i64,
    nanos: u32,
}
impl WriteTime {
    pub fn unix_seconds(&self) -> i64 {
        self.unix_seconds
    }
    pub fn nanos(&self) -> u32 {
        self.nanos
    }
}

/// Engine-assigned metadata, separate from associated property entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    seq: u64,
    property_hash: [u8; 32],
    created_at: WriteTime,
}
impl Record {
    pub fn seq(&self) -> u64 {
        self.seq
    }
    pub fn property_hash(&self) -> &[u8; 32] {
        &self.property_hash
    }
    pub fn created_at(&self) -> &WriteTime {
        &self.created_at
    }
}

/// A detached read result. Editing properties does not change the stored entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub record: Record,
    pub properties: Properties,
}

/// Exclusive lower and inclusive upper sequence bounds, with a result-count cap.
/// The default limit is 100; choose a lower explicit limit when configured
/// `max_query_records` is below 100, otherwise the query is rejected.
#[derive(Debug, Clone, Copy)]
pub struct Query {
    pub after: Option<u64>,
    pub through: Option<u64>,
    pub limit: u32,
}
impl Default for Query {
    fn default() -> Self {
        Self {
            after: None,
            through: None,
            limit: 100,
        }
    }
}

/// Sequence pagination; each call observes its own published snapshot.
#[derive(Debug)]
pub struct Page {
    pub entries: Vec<Entry>,
    pub next_after: Option<u64>,
}

/// Small, direct errors. Capacity limits do not imply corrupt stored data.
#[derive(Debug)]
pub enum Error {
    Busy,
    InvalidOptions(&'static str),
    InvalidQuery(&'static str),
    TooLarge,
    TooDeep,
    Allocation,
    ResourceLimit {
        seq: u64,
        required_encoded_property_bytes: u64,
        max_bytes: u64,
    },
    Corruption {
        offset: u64,
        reason: &'static str,
    },
    SequenceExhausted,
    ClockRange,
    Stopped,
    Io(io::Error),
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Busy => f.write_str("log is locked by another owner"),
            Self::InvalidOptions(s) => write!(f, "invalid options: {s}"),
            Self::InvalidQuery(s) => write!(f, "invalid query: {s}"),
            Self::TooLarge => f.write_str("encoded properties exceed the content bound"),
            Self::TooDeep => f.write_str("native value nesting exceeds the ingest bound"),
            Self::Allocation => f.write_str("allocation failed"),
            Self::ResourceLimit {
                seq,
                required_encoded_property_bytes,
                max_bytes,
            } => write!(
                f,
                "record {seq} requires {required_encoded_property_bytes} encoded property bytes; limit is {max_bytes}"
            ),
            Self::Corruption { offset, reason } => {
                write!(f, "corrupt log at byte {offset}: {reason}")
            }
            Self::SequenceExhausted => f.write_str("write sequence exhausted"),
            Self::ClockRange => f.write_str("write time is outside the supported range"),
            Self::Stopped => f.write_str("writer stopped after an uncertain operation or panic"),
            Self::Io(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Io(e) = self {
            Some(e)
        } else {
            None
        }
    }
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppendOutcome {
    NotWritten,
    Unknown,
}

/// Returned failures preserve supplied content. Unknown does not mean absent.
#[derive(Debug)]
pub struct AppendError {
    pub content: Content,
    pub outcome: AppendOutcome,
    pub error: Error,
}
impl fmt::Display for AppendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.outcome, self.error)
    }
}
impl std::error::Error for AppendError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}

/// An exclusive synchronous writer with independently opened read cursors.
///
/// No mutation operation other than append is exposed. Queries and startup are
/// linear scans. External file replacement/modification is unsupported.
pub struct Log {
    path: std::path::PathBuf,
    options: Options,
    writer: Mutex<storage::Writer<std::fs::File>>,
}
impl Log {
    /// Create a new file or validate an existing clean log, leaving damage untouched.
    pub fn open(path: impl AsRef<Path>, options: Options) -> Result<Self, Error> {
        let options = options.validate()?;
        let (path, writer) = storage::open(path.as_ref(), options)?;
        Ok(Self {
            path,
            options,
            writer: Mutex::new(writer),
        })
    }

    /// Own and pack content, then write one frame. Optional sync is part of success.
    ///
    /// This blocks the calling thread. Once I/O is attempted, any write/sync error
    /// stops this instance's writer; the outcome is unknown and is never auto-retried.
    pub fn append(&self, content: impl Into<Content>) -> Result<Record, AppendError> {
        let content = content.into();
        let result = (|| {
            let mut frame =
                codec::pack(&content, self.options).map_err(|e| (AppendOutcome::NotWritten, e))?;
            let property_end = frame.len() - storage::TRAILER_LEN;
            let property_hash = *blake3::hash(&frame[storage::HEADER_LEN..property_end]).as_bytes();
            let mut writer = self
                .writer
                .lock()
                .map_err(|_| (AppendOutcome::NotWritten, Error::Stopped))?;
            if writer.stopped {
                return Err((AppendOutcome::NotWritten, Error::Stopped));
            }
            let seq = writer
                .next
                .ok_or((AppendOutcome::NotWritten, Error::SequenceExhausted))?;
            let created_at =
                write_time(SystemTime::now()).map_err(|e| (AppendOutcome::NotWritten, e))?;
            let record = Record {
                seq,
                property_hash,
                created_at,
            };
            storage::fill_frame(&mut frame, &record);
            writer.append_frame(&frame, seq, self.options.sync_each_append, |f| f.sync_all())?;
            Ok(record)
        })();
        result.map_err(|(outcome, error)| AppendError {
            content,
            outcome,
            error,
        })
    }

    fn snapshot(&self) -> Result<u64, Error> {
        Ok(self
            .writer
            .lock()
            .map_err(|_| Error::Stopped)?
            .published_end)
    }

    /// Linear point lookup in a captured published prefix, using a fresh read file.
    pub fn read(&self, seq: u64) -> Result<Option<Entry>, Error> {
        let end = self.snapshot()?;
        let mut scan = storage::Scanner::open(&self.path, end, self.options.max_content_bytes)?;
        while let Some((entry, _)) = scan.next()? {
            if entry.record.seq() == seq {
                return Ok(Some(entry));
            }
            if entry.record.seq() > seq {
                break;
            }
        }
        Ok(None)
    }

    /// Linear range scan with result count/encoded-byte limits, not a scan-time bound.
    pub fn query(&self, query: Query) -> Result<Page, Error> {
        if query.limit == 0
            || query.limit > self.options.max_query_records
            || matches!((query.after, query.through), (Some(a), Some(b)) if a > b)
        {
            return Err(Error::InvalidQuery("invalid bounds or result limit"));
        }
        let end = self.snapshot()?;
        let mut scan = storage::Scanner::open(&self.path, end, self.options.max_content_bytes)?;
        let mut entries = Vec::new();
        let mut bytes = 0_u64;
        while let Some((entry, length)) = scan.next()? {
            let seq = entry.record.seq();
            if query.through.is_some_and(|v| seq > v) {
                break;
            }
            if query.after.is_some_and(|v| seq <= v) {
                continue;
            }
            if length > self.options.max_query_bytes - bytes {
                if entries.is_empty() {
                    return Err(Error::ResourceLimit {
                        seq,
                        required_encoded_property_bytes: length,
                        max_bytes: self.options.max_query_bytes,
                    });
                }
                return Ok(Page {
                    next_after: entries.last().map(|e: &Entry| e.record.seq()),
                    entries,
                });
            }
            entries.try_reserve(1).map_err(|_| Error::Allocation)?;
            bytes += length;
            entries.push(entry);
            if entries.len() == query.limit as usize {
                return Ok(Page {
                    next_after: entries.last().map(|e| e.record.seq()),
                    entries,
                });
            }
        }
        Ok(Page {
            entries,
            next_after: None,
        })
    }
}

fn write_time(time: SystemTime) -> Result<WriteTime, Error> {
    let (seconds, nanos) = match time.duration_since(UNIX_EPOCH) {
        Ok(d) => (i128::from(d.as_secs()), d.subsec_nanos()),
        Err(e) => {
            let d = e.duration();
            if d.subsec_nanos() == 0 {
                (-i128::from(d.as_secs()), 0)
            } else {
                (
                    -i128::from(d.as_secs()) - 1,
                    1_000_000_000 - d.subsec_nanos(),
                )
            }
        }
    };
    Ok(WriteTime {
        unix_seconds: i64::try_from(seconds).map_err(|_| Error::ClockRange)?,
        nanos,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn clock_regression_and_pre_epoch_subseconds_are_preserved() {
        let before = write_time(UNIX_EPOCH - Duration::from_nanos(1)).unwrap();
        assert_eq!((before.unix_seconds(), before.nanos()), (-1, 999_999_999));
        let later = write_time(UNIX_EPOCH + Duration::from_secs(20)).unwrap();
        let regressed = write_time(UNIX_EPOCH + Duration::from_secs(10)).unwrap();
        assert!(regressed.unix_seconds() < later.unix_seconds());
        assert_eq!(write_time(UNIX_EPOCH).unwrap().nanos(), 0);
    }

    #[test]
    fn invalid_options_fail_before_opening_a_file() {
        for options in [
            Options {
                max_content_bytes: 7,
                ..Options::default()
            },
            Options {
                max_depth: 129,
                ..Options::default()
            },
            Options {
                max_query_bytes: 0,
                ..Options::default()
            },
            Options {
                max_query_records: 0,
                ..Options::default()
            },
        ] {
            assert!(matches!(options.validate(), Err(Error::InvalidOptions(_))));
        }
    }
}
