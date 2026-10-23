use crate::{AppendOutcome, BatchOptions, Content, Error, Options, Record, codec, storage};
use std::fs::File;
use std::io;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};
use std::time::Instant;

pub(crate) type FrameResult = Result<Record, (AppendOutcome, Error)>;
pub(crate) struct Prepared {
    pub(crate) frame: Vec<u8>,
    pub(crate) hash: [u8; 32],
}
pub(crate) fn prepare(content: &Content, options: Options) -> Result<Prepared, Error> {
    let frame = codec::pack(content, options)?;
    let hash =
        *blake3::hash(&frame[storage::HEADER_LEN..frame.len() - storage::TRAILER_LEN]).as_bytes();
    Ok(Prepared { frame, hash })
}
pub(crate) fn failures(outcome: AppendOutcome, error: Error, count: usize) -> Vec<FrameResult> {
    let error = match error {
        Error::Io(e) => Error::SharedIo(Arc::new(e)),
        other => other,
    };
    (0..count)
        .map(|_| {
            Err((
                outcome,
                match &error {
                    Error::SharedIo(e) => Error::SharedIo(Arc::clone(e)),
                    Error::Stopped => Error::Stopped,
                    Error::QueueFull => Error::QueueFull,
                    Error::SequenceExhausted => Error::SequenceExhausted,
                    Error::ClockRange => Error::ClockRange,
                    Error::TooLarge => Error::TooLarge,
                    _ => unreachable!("private storage failure kind"),
                },
            ))
        })
        .collect()
}

struct Budget {
    state: Mutex<(usize, usize)>,
    options: BatchOptions,
}
impl Budget {
    fn acquire(self: &Arc<Self>, bytes: usize) -> Result<Permit, Error> {
        let mut state = self.state.lock().map_err(|_| Error::Stopped)?;
        if state.0 >= self.options.max_pending_batches
            || bytes > self.options.max_pending_bytes.saturating_sub(state.1)
        {
            return Err(Error::QueueFull);
        }
        state.0 += 1;
        state.1 += bytes;
        Ok(Permit {
            budget: Arc::clone(self),
            bytes,
        })
    }
}
struct Permit {
    budget: Arc<Budget>,
    bytes: usize,
}
impl Drop for Permit {
    fn drop(&mut self) {
        let mut state = self.budget.state.lock().unwrap_or_else(|e| e.into_inner());
        state.0 -= 1;
        state.1 -= self.bytes;
    }
}
struct Work {
    frames: Vec<Prepared>,
    bytes: usize,
    enqueued: Instant,
    reply: mpsc::Sender<Vec<FrameResult>>,
    permit: Permit,
}
pub(crate) struct Batcher {
    sender: Option<mpsc::SyncSender<Work>>,
    worker: Option<JoinHandle<()>>,
    budget: Arc<Budget>,
}
impl Batcher {
    pub(crate) fn new(
        writer: Arc<Mutex<storage::Writer<File>>>,
        options: BatchOptions,
    ) -> Result<Self, Error> {
        let budget = Arc::new(Budget {
            state: Mutex::new((0, 0)),
            options,
        });
        let (sender, receiver) = mpsc::sync_channel(options.max_pending_batches);
        let worker = thread::Builder::new()
            .name("deleuze-sync".into())
            .spawn(move || run(receiver, writer, options, |file| file.sync_all()))?;
        Ok(Self {
            sender: Some(sender),
            worker: Some(worker),
            budget,
        })
    }
    pub(crate) fn submit(&self, frames: Vec<Prepared>) -> Vec<FrameResult> {
        let count = frames.len();
        let Some(bytes) = frames
            .iter()
            .try_fold(0_usize, |sum, frame| sum.checked_add(frame.frame.len()))
        else {
            return failures(AppendOutcome::NotWritten, Error::QueueFull, count);
        };
        let permit = match self.budget.acquire(bytes) {
            Ok(permit) => permit,
            Err(error) => return failures(AppendOutcome::NotWritten, error, count),
        };
        let (reply, receiver) = mpsc::channel();
        let work = Work {
            frames,
            bytes,
            enqueued: Instant::now(),
            reply,
            permit,
        };
        match self
            .sender
            .as_ref()
            .expect("live batcher sender")
            .try_send(work)
        {
            Ok(()) => receive(receiver, count),
            Err(mpsc::TrySendError::Full(_)) => {
                failures(AppendOutcome::NotWritten, Error::QueueFull, count)
            }
            Err(mpsc::TrySendError::Disconnected(_)) => {
                failures(AppendOutcome::NotWritten, Error::Stopped, count)
            }
        }
    }
}
fn receive(receiver: mpsc::Receiver<Vec<FrameResult>>, count: usize) -> Vec<FrameResult> {
    receiver
        .recv()
        .unwrap_or_else(|_| failures(AppendOutcome::Unknown, Error::Stopped, count))
}
impl Drop for Batcher {
    fn drop(&mut self) {
        drop(self.sender.take());
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

fn collect(
    first: Work,
    receiver: &mpsc::Receiver<Work>,
    options: BatchOptions,
) -> (Vec<Work>, Option<Work>) {
    let deadline = first.enqueued + options.max_delay;
    let mut records = first.frames.len();
    let mut bytes = first.bytes;
    let mut groups = vec![first];
    let mut carry = None;
    while records < options.max_records && bytes < options.max_bytes {
        // Expired age forbids intentional waiting, not draining ready work.
        let next = match receiver.try_recv() {
            Ok(next) => next,
            Err(mpsc::TryRecvError::Disconnected) => break,
            Err(mpsc::TryRecvError::Empty) => {
                if Instant::now() >= deadline {
                    break;
                }
                match receiver.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
                    Ok(next) => next,
                    Err(_) => break,
                }
            }
        };
        if next.frames.len() > options.max_records - records
            || next.bytes > options.max_bytes - bytes
        {
            carry = Some(next);
            break;
        }
        records += next.frames.len();
        bytes += next.bytes;
        groups.push(next);
    }
    (groups, carry)
}
fn run<W: io::Write>(
    receiver: mpsc::Receiver<Work>,
    writer: Arc<Mutex<storage::Writer<W>>>,
    options: BatchOptions,
    synchronize: impl Fn(&mut W) -> io::Result<()>,
) {
    let mut carry = None;
    while let Some(first) = carry.take().or_else(|| receiver.recv().ok()) {
        let (mut groups, next) = collect(first, &receiver, options);
        carry = next;
        let lengths: Vec<_> = groups.iter().map(|g| g.frames.len()).collect();
        let mut frames = Vec::new();
        for group in &mut groups {
            frames.append(&mut group.frames);
        }
        let results = match writer.lock() {
            Ok(mut writer) => writer.append_prepared(&mut frames, true, &synchronize),
            Err(_) => failures(AppendOutcome::NotWritten, Error::Stopped, frames.len()),
        };
        drop(frames);
        let mut results = results.into_iter();
        for (group, length) in groups.into_iter().zip(lengths) {
            let result = results.by_ref().take(length).collect();
            drop(group.permit);
            let _ = group.reply.send(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    fn budget(options: BatchOptions) -> Arc<Budget> {
        Arc::new(Budget {
            state: Mutex::new((0, 0)),
            options,
        })
    }
    fn work(budget: &Arc<Budget>, age: Duration) -> (Work, mpsc::Receiver<Vec<FrameResult>>) {
        let prepared = prepare(&Content::Bytes(vec![42]), Options::default()).unwrap();
        let bytes = prepared.frame.len();
        let (reply, receiver) = mpsc::channel();
        (
            Work {
                frames: vec![prepared],
                bytes,
                enqueued: Instant::now() - age,
                reply,
                permit: budget.acquire(bytes).unwrap(),
            },
            receiver,
        )
    }
    #[test]
    fn aged_ready_groups_still_coalesce_without_waiting() {
        let options = BatchOptions {
            max_records: 2,
            ..BatchOptions::default()
        };
        let budget = budget(options);
        let (first, _) = work(&budget, Duration::from_secs(1));
        let (second, _) = work(&budget, Duration::from_secs(1));
        let (sender, receiver) = mpsc::sync_channel(2);
        sender.send(second).unwrap();
        let (groups, carry) = collect(first, &receiver, options);
        assert_eq!(
            groups.len(),
            2,
            "old age must stop waiting, not prevent draining ready work"
        );
        assert!(carry.is_none());
    }
    #[test]
    fn zero_delay_is_valid_no_intentional_wait() {
        assert!(
            Options {
                batch: BatchOptions {
                    max_delay: Duration::ZERO,
                    ..BatchOptions::default()
                },
                ..Options::default()
            }
            .validate()
            .is_ok()
        );
    }
}
