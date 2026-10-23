use crate::{AppendOutcome, Entry, Error, Options, Record, WriteTime, codec};
use std::fs::{File, OpenOptions, TryLockError};
use std::io::{self, IoSlice, Read, Write};
use std::path::{Path, PathBuf};

pub(crate) const HEADER_LEN: usize = 64;
pub(crate) const TRAILER_LEN: usize = 12;
const FILE_HEADER: &[u8; 12] = b"DLZLOG01\x01\x00\x00\x00";

pub(crate) struct Writer<W> {
    pub(crate) io: W,
    pub(crate) next: Option<u64>,
    pub(crate) published_end: u64,
    pub(crate) stopped: bool,
}
impl<W: Write> Writer<W> {
    pub(crate) fn append_frame(
        &mut self,
        frame: &[u8],
        seq: u64,
        sync: bool,
        synchronize: impl FnOnce(&mut W) -> io::Result<()>,
    ) -> Result<(), (AppendOutcome, Error)> {
        self.write_group(&[frame], seq, sync, synchronize)
    }
    fn write_group(
        &mut self,
        frames: &[&[u8]],
        last_seq: u64,
        sync: bool,
        synchronize: impl FnOnce(&mut W) -> io::Result<()>,
    ) -> Result<(), (AppendOutcome, Error)> {
        if self.stopped {
            return Err((AppendOutcome::NotWritten, Error::Stopped));
        }
        let end = frames
            .iter()
            .try_fold(self.published_end, |end, frame| {
                end.checked_add(frame.len() as u64)
            })
            .ok_or((AppendOutcome::NotWritten, Error::TooLarge))?;
        if let Err(error) = write_frames(&mut self.io, frames).and_then(|()| {
            if sync {
                synchronize(&mut self.io)
            } else {
                Ok(())
            }
        }) {
            self.stopped = true;
            return Err((AppendOutcome::Unknown, Error::Io(error)));
        }
        self.published_end = end;
        self.next = last_seq.checked_add(1);
        Ok(())
    }
    pub(crate) fn append_prepared(
        &mut self,
        frames: &mut [crate::batch::Prepared],
        sync: bool,
        synchronize: impl FnOnce(&mut W) -> io::Result<()>,
    ) -> Vec<crate::batch::FrameResult> {
        use crate::batch::failures;
        let count = frames.len();
        if count == 0 {
            return Vec::new();
        }
        if self.stopped {
            return failures(AppendOutcome::NotWritten, Error::Stopped, count);
        }
        if self.next.is_none() {
            return failures(AppendOutcome::NotWritten, Error::SequenceExhausted, count);
        }
        let created_at = match crate::write_time(std::time::SystemTime::now()) {
            Ok(time) => time,
            Err(error) => return failures(AppendOutcome::NotWritten, error, count),
        };
        let mut next = self.next;
        let mut records = Vec::new();
        for frame in frames.iter_mut() {
            let Some(seq) = next else {
                break;
            };
            let record = Record {
                seq,
                property_hash: frame.hash,
                created_at,
            };
            fill_frame(&mut frame.frame, &record);
            records.push(record);
            next = seq.checked_add(1);
        }
        let attempted = records.len();
        let slices: Vec<_> = frames[..attempted]
            .iter()
            .map(|frame| frame.frame.as_slice())
            .collect();
        let mut results = match self.write_group(
            &slices,
            records.last().expect("at least one sequence").seq,
            sync,
            synchronize,
        ) {
            Ok(()) => records.into_iter().map(Ok).collect(),
            Err((outcome, error)) => failures(outcome, error, attempted),
        };
        results.extend(failures(
            AppendOutcome::NotWritten,
            Error::SequenceExhausted,
            count - attempted,
        ));
        results
    }
}

fn write_frames(writer: &mut impl Write, frames: &[&[u8]]) -> io::Result<()> {
    if let [frame] = frames {
        return writer.write_all(frame);
    }
    let (mut index, mut offset) = (0, 0);
    while index < frames.len() {
        let slices: Vec<_> = frames[index..]
            .iter()
            .take(64)
            .enumerate()
            .map(|(n, frame)| IoSlice::new(if n == 0 { &frame[offset..] } else { frame }))
            .collect();
        let written = match writer.write_vectored(&slices) {
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if written == 0 {
            return Err(io::Error::from(io::ErrorKind::WriteZero));
        }
        let mut remaining = written;
        while index < frames.len() && remaining >= frames[index].len() - offset {
            remaining -= frames[index].len() - offset;
            index += 1;
            offset = 0;
        }
        offset += remaining;
    }
    Ok(())
}

pub(crate) fn fill_frame(frame: &mut [u8], record: &Record) {
    let length = frame.len();
    frame[..4].copy_from_slice(b"DLZR");
    frame[4..12].copy_from_slice(&((length - HEADER_LEN - TRAILER_LEN) as u64).to_le_bytes());
    frame[12..20].copy_from_slice(&record.seq.to_le_bytes());
    frame[20..28].copy_from_slice(&record.created_at.unix_seconds.to_le_bytes());
    frame[28..32].copy_from_slice(&record.created_at.nanos.to_le_bytes());
    frame[32..64].copy_from_slice(&record.property_hash);
    frame[length - 12..length - 8].copy_from_slice(b"ENDR");
    frame[length - 8..].copy_from_slice(&(length as u64).to_le_bytes());
}

pub(crate) fn open(path: &Path, options: Options) -> Result<(PathBuf, Writer<File>), Error> {
    let result = OpenOptions::new()
        .read(true)
        .append(true)
        .create_new(true)
        .open(path);
    let (mut file, new) = match result {
        Ok(file) => (file, true),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (
            OpenOptions::new().read(true).append(true).open(path)?,
            false,
        ),
        Err(e) => return Err(e.into()),
    };
    match file.try_lock() {
        Ok(()) => (),
        Err(TryLockError::WouldBlock) => return Err(Error::Busy),
        Err(TryLockError::Error(e)) => return Err(e.into()),
    }
    if new {
        file.write_all(FILE_HEADER)?;
    }
    let path = path.canonicalize()?;
    let end = file.metadata()?.len();
    // Scanner always opens a separate file: cloned handles can share seek state.
    let mut scanner = Scanner::open(&path, end, options.max_content_bytes)?;
    let mut last = None;
    while let Some((entry, _)) = scanner.next()? {
        last = Some(entry.record.seq());
    }
    let next = match last {
        Some(seq) => seq.checked_add(1),
        None => Some(1),
    };
    Ok((
        path,
        Writer {
            io: file,
            next,
            published_end: end,
            stopped: false,
        },
    ))
}

pub(crate) struct Scanner {
    file: File,
    end: u64,
    offset: u64,
    expected: Option<u64>,
    capacity: u64,
}
impl Scanner {
    pub(crate) fn open(path: &Path, end: u64, capacity: u64) -> Result<Self, Error> {
        let mut file = File::open(path)?;
        let mut header = [0; 12];
        if end < FILE_HEADER.len() as u64 {
            return Err(bad(0, "incomplete file header"));
        }
        read_exact(&mut file, &mut header, 0)?;
        if &header != FILE_HEADER {
            return Err(bad(0, "unsupported or corrupt file header"));
        }
        Ok(Self {
            file,
            end,
            offset: 12,
            expected: Some(1),
            capacity,
        })
    }
    pub(crate) fn next(&mut self) -> Result<Option<(Entry, u64)>, Error> {
        let offset = self.offset;
        if offset == self.end {
            return Ok(None);
        }
        if self.end - offset < HEADER_LEN as u64 {
            return Err(bad(offset, "incomplete record header"));
        }
        let mut header = [0; HEADER_LEN];
        read_exact(&mut self.file, &mut header, offset)?;
        if &header[..4] != b"DLZR" {
            return Err(bad(offset, "invalid record magic"));
        }
        let length = u64::from_le_bytes(header[4..12].try_into().expect("eight bytes"));
        let seq = u64::from_le_bytes(header[12..20].try_into().expect("eight bytes"));
        let seconds = i64::from_le_bytes(header[20..28].try_into().expect("eight bytes"));
        let nanos = u32::from_le_bytes(header[28..32].try_into().expect("four bytes"));
        if Some(seq) != self.expected {
            return Err(bad(offset, "unexpected sequence"));
        }
        if nanos >= 1_000_000_000 {
            return Err(bad(offset, "invalid timestamp nanoseconds"));
        }
        let frame_length = length
            .checked_add((HEADER_LEN + TRAILER_LEN) as u64)
            .ok_or_else(|| bad(offset, "frame length overflow"))?;
        let frame_end = offset
            .checked_add(frame_length)
            .ok_or_else(|| bad(offset, "file offset overflow"))?;
        if length < 8 || frame_end > self.end {
            return Err(bad(offset, "invalid or incomplete frame length"));
        }
        if length > self.capacity {
            return Err(Error::ResourceLimit {
                seq,
                required_encoded_property_bytes: length,
                max_bytes: self.capacity,
            });
        }
        let size = usize::try_from(length).map_err(|_| Error::ResourceLimit {
            seq,
            required_encoded_property_bytes: length,
            max_bytes: usize::MAX as u64,
        })?;
        let mut payload = Vec::new();
        payload.try_reserve(size).map_err(|_| Error::Allocation)?;
        payload.resize(size, 0);
        read_exact(&mut self.file, &mut payload, offset + HEADER_LEN as u64)?;
        let hash: [u8; 32] = header[32..64].try_into().expect("32 bytes");
        if blake3::hash(&payload).as_bytes() != &hash {
            return Err(bad(offset, "property hash mismatch"));
        }
        let properties = codec::decode(&payload, offset + HEADER_LEN as u64)?;
        let mut trailer = [0; TRAILER_LEN];
        read_exact(
            &mut self.file,
            &mut trailer,
            offset + HEADER_LEN as u64 + length,
        )?;
        if &trailer[..4] != b"ENDR"
            || u64::from_le_bytes(trailer[4..].try_into().expect("eight bytes")) != frame_length
        {
            return Err(bad(offset, "invalid frame trailer"));
        }
        self.offset = frame_end;
        self.expected = seq.checked_add(1);
        Ok(Some((
            Entry {
                record: Record {
                    seq,
                    property_hash: hash,
                    created_at: WriteTime {
                        unix_seconds: seconds,
                        nanos,
                    },
                },
                properties,
            },
            length,
        )))
    }
}
fn bad(offset: u64, reason: &'static str) -> Error {
    Error::Corruption { offset, reason }
}
fn read_exact(file: &mut File, bytes: &mut [u8], offset: u64) -> Result<(), Error> {
    match file.read_exact(bytes) {
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
            Err(bad(offset, "unexpected end of file"))
        }
        result => result.map_err(Into::into),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct ShortWriter {
        bytes: Vec<u8>,
        fail_after: Option<usize>,
        interrupt: bool,
    }
    impl Write for ShortWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.interrupt {
                self.interrupt = false;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            let remaining = self
                .fail_after
                .unwrap_or(usize::MAX)
                .saturating_sub(self.bytes.len());
            if remaining == 0 {
                return Err(io::Error::other("scripted write failure"));
            }
            let n = bytes.len().min(3).min(remaining);
            self.bytes.extend_from_slice(&bytes[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    fn writer(fail_after: Option<usize>) -> Writer<ShortWriter> {
        Writer {
            io: ShortWriter {
                bytes: Vec::new(),
                fail_after,
                interrupt: true,
            },
            next: Some(1),
            published_end: 12,
            stopped: false,
        }
    }
    #[test]
    fn short_interrupted_writes_complete_without_default_sync() {
        let mut writer = writer(None);
        writer
            .append_frame(b"abcdefgh", 1, false, |_| panic!("default must not sync"))
            .unwrap();
        assert_eq!(writer.io.bytes, b"abcdefgh");
        assert_eq!(
            (writer.next, writer.published_end, writer.stopped),
            (Some(2), 20, false)
        );
    }
    #[test]
    fn errors_at_each_short_write_boundary_stop_and_leave_publication_unchanged() {
        for cut in 0..8 {
            let mut writer = writer(Some(cut));
            let error = writer
                .append_frame(b"abcdefgh", 1, false, |_| Ok(()))
                .unwrap_err();
            assert_eq!(error.0, AppendOutcome::Unknown);
            assert_eq!(writer.io.bytes, &b"abcdefgh"[..cut]);
            assert_eq!(
                (writer.next, writer.published_end, writer.stopped),
                (Some(1), 12, true)
            );
            assert!(matches!(
                writer.append_frame(b"ignored", 1, false, |_| Ok(())),
                Err((AppendOutcome::NotWritten, Error::Stopped))
            ));
            assert_eq!(writer.io.bytes.len(), cut);
        }
    }
    #[test]
    fn final_sequence_does_not_wrap() {
        let mut writer = writer(None);
        writer.next = Some(u64::MAX);
        writer
            .append_frame(b"last", u64::MAX, false, |_| Ok(()))
            .unwrap();
        assert_eq!(writer.next, None);
    }

    #[test]
    fn sync_error_can_follow_full_write_but_does_not_publish_it() {
        let mut writer = writer(None);
        let error = writer
            .append_frame(b"abcdefgh", 1, true, |_| {
                Err(io::Error::other("scripted sync failure"))
            })
            .unwrap_err();
        assert_eq!(error.0, AppendOutcome::Unknown);
        assert_eq!(writer.io.bytes, b"abcdefgh");
        assert_eq!(
            (writer.next, writer.published_end, writer.stopped),
            (Some(1), 12, true)
        );
    }
}

#[cfg(test)]
mod batch_tests {
    use super::*;
    use crate::{Content, batch};
    use std::cell::Cell;
    struct Vectored {
        bytes: Vec<u8>,
        interrupt: bool,
        zero_at: Option<usize>,
    }
    impl Write for Vectored {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.write_vectored(&[IoSlice::new(bytes)])
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn write_vectored(&mut self, slices: &[IoSlice<'_>]) -> io::Result<usize> {
            if self.interrupt {
                self.interrupt = false;
                return Err(io::Error::from(io::ErrorKind::Interrupted));
            }
            let allowed = 4.min(
                self.zero_at
                    .unwrap_or(usize::MAX)
                    .saturating_sub(self.bytes.len()),
            );
            let mut written = 0;
            for slice in slices {
                let take = slice.len().min(allowed - written);
                self.bytes.extend_from_slice(&slice[..take]);
                written += take;
                if written == allowed {
                    break;
                }
            }
            Ok(written)
        }
    }
    #[test]
    fn partial_vectored_writes_cross_frames_and_handle_interrupted_and_zero() {
        let mut output = Vectored {
            bytes: Vec::new(),
            interrupt: true,
            zero_at: None,
        };
        write_frames(&mut output, &[b"ab", b"cdef", b"g"]).unwrap();
        assert_eq!(output.bytes, b"abcdefg");
        let mut output = Vectored {
            bytes: Vec::new(),
            interrupt: true,
            zero_at: Some(4),
        };
        assert_eq!(
            write_frames(&mut output, &[b"ab", b"cdef", b"g"])
                .unwrap_err()
                .kind(),
            io::ErrorKind::WriteZero
        );
        assert_eq!(output.bytes, b"abcd");
    }
    fn prepared(count: u8) -> Vec<batch::Prepared> {
        (0..count)
            .map(|n| batch::prepare(&Content::Bytes(vec![n]), Options::default()).unwrap())
            .collect()
    }
    #[test]
    fn one_barrier_covers_complete_frames_and_publishes_only_on_success() {
        let mut writer = Writer {
            io: Vec::<u8>::new(),
            next: Some(1),
            published_end: 12,
            stopped: false,
        };
        let barriers = Cell::new(0);
        let mut frames = prepared(3);
        let results = writer.append_prepared(&mut frames, true, |bytes| {
            assert_eq!(bytes.len(), 3 * 109);
            barriers.set(barriers.get() + 1);
            Ok(())
        });
        assert_eq!(barriers.get(), 1);
        assert_eq!(writer.published_end, 12 + 3 * 109);
        assert_eq!(
            results
                .iter()
                .map(|r| r.as_ref().unwrap().seq())
                .collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        for (n, frame) in frames.iter().enumerate() {
            assert_eq!(&frame.frame[12..20], &(n as u64 + 1).to_le_bytes());
            assert_eq!(
                &frame.frame[32..64],
                results[n].as_ref().unwrap().property_hash()
            );
        }
        assert_eq!(
            writer.io,
            frames
                .iter()
                .flat_map(|f| f.frame.iter().copied())
                .collect::<Vec<_>>()
        );
        let mut failed = Writer {
            io: Vec::<u8>::new(),
            next: Some(1),
            published_end: 12,
            stopped: false,
        };
        let errors = failed.append_prepared(&mut prepared(3), true, |_| {
            Err(io::Error::other("barrier failed"))
        });
        assert!(
            errors
                .iter()
                .all(|r| matches!(r, Err((AppendOutcome::Unknown, Error::SharedIo(_)))))
        );
        assert_eq!(
            (failed.published_end, failed.next, failed.stopped),
            (12, Some(1), true)
        );
        let untouched = failed.append_prepared(&mut prepared(1), true, |_| {
            panic!("must not synchronize after stop")
        });
        assert!(matches!(
            &untouched[0],
            Err((AppendOutcome::NotWritten, Error::Stopped))
        ));
    }
    #[test]
    fn sequence_exhaustion_writes_valid_prefix_without_wrapping() {
        let mut writer = Writer {
            io: Vec::<u8>::new(),
            next: Some(u64::MAX - 1),
            published_end: 12,
            stopped: false,
        };
        let results =
            writer.append_prepared(&mut prepared(3), false, |_| panic!("written must not sync"));
        assert_eq!(results[0].as_ref().unwrap().seq(), u64::MAX - 1);
        assert_eq!(results[1].as_ref().unwrap().seq(), u64::MAX);
        assert!(matches!(
            &results[2],
            Err((AppendOutcome::NotWritten, Error::SequenceExhausted))
        ));
        assert_eq!(writer.next, None);
        assert_eq!(writer.io.len(), 2 * 109);
    }
}
