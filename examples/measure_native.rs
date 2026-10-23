//! Release baseline: --mode log|file --out-dir NEW --count N --payload-bytes N
//! --producers N --sync written|synced. Closed-loop call latency, not open-loop
//! response latency. File mode is a frame-sized I/O floor without log semantics.
mod measure_support;

use deleuze::{AppendOutcome, BatchOptions, Completion, Content, Log, Options, Query, Value};
use measure_support::{
    Outcome, Sample, make_payload, nearest_rank, verify_payload, write_samples, write_summary,
};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{self, IoSlice, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Barrier, Mutex, OnceLock};
use std::time::{Duration, Instant};

const OVERHEAD: usize = 108; // Bytes property encoding (32) + physical frame (76).
const MAX_FILE_BYTES: u64 = 512 * 1024 * 1024;
type CallResult = (Outcome, Option<u64>, Option<[u8; 32]>);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Log,
    File,
}
struct Args {
    mode: Mode,
    out: PathBuf,
    count: u64,
    payload: usize,
    producers: usize,
    synced: bool,
    batch_size: usize,
    batch_records: usize,
    batch_bytes: usize,
    batch_delay_us: u64,
}
impl Args {
    fn parse(arguments: impl Iterator<Item = String>) -> io::Result<Self> {
        let mut mode = None;
        let mut out = None;
        let (mut count, mut payload, mut producers, mut synced) = (10_000, 128, 1, false);
        let (mut batch_size, mut batch_records, mut batch_bytes, mut batch_delay_us) =
            (1, 256, 1024 * 1024, 1000);
        let mut args = arguments;
        while let Some(flag) = args.next() {
            let value = args
                .next()
                .ok_or_else(|| invalid("every option requires a value"))?;
            match flag.as_str() {
                "--mode" => {
                    mode = Some(match value.as_str() {
                        "log" => Mode::Log,
                        "file" => Mode::File,
                        _ => return Err(invalid("mode must be log or file")),
                    })
                }
                "--out-dir" => out = Some(PathBuf::from(value)),
                "--count" => count = value.parse().map_err(|_| invalid("invalid count"))?,
                "--payload-bytes" => {
                    payload = value.parse().map_err(|_| invalid("invalid payload size"))?
                }
                "--producers" => {
                    producers = value
                        .parse()
                        .map_err(|_| invalid("invalid producer count"))?
                }
                "--batch-size" => {
                    batch_size = value.parse().map_err(|_| invalid("invalid batch size"))?
                }
                "--batch-records" => {
                    batch_records = value
                        .parse()
                        .map_err(|_| invalid("invalid batch record bound"))?
                }
                "--batch-bytes" => {
                    batch_bytes = value
                        .parse()
                        .map_err(|_| invalid("invalid batch byte bound"))?
                }
                "--batch-delay-us" => {
                    batch_delay_us = value.parse().map_err(|_| invalid("invalid batch delay"))?
                }
                "--sync" => {
                    synced = match value.as_str() {
                        "written" => false,
                        "synced" => true,
                        _ => return Err(invalid("sync must be written or synced")),
                    }
                }
                _ => return Err(invalid("unknown option")),
            }
        }
        let result = Self {
            mode: mode.ok_or_else(|| invalid("--mode is required"))?,
            out: out.ok_or_else(|| invalid("--out-dir is required"))?,
            count,
            payload,
            producers,
            synced,
            batch_size,
            batch_records,
            batch_bytes,
            batch_delay_us,
        };
        result.validate()?;
        Ok(result)
    }
    fn validate(&self) -> io::Result<()> {
        if !(1..=1_000_000).contains(&self.count)
            || !(8..=1024 * 1024).contains(&self.payload)
            || !(1..=128).contains(&self.producers)
            || self.producers as u64 > self.count
            || !(1..=65_536).contains(&self.batch_size)
            || !(1..=65_536).contains(&self.batch_records)
            || self.batch_bytes == 0
            || self.batch_bytes > 64 * 1024 * 1024
            || self.batch_delay_us > 1_000_000
        {
            return Err(invalid(
                "require count1..1M, payload8..1MiB, producers1..128 and producers<=count",
            ));
        }
        if self
            .count
            .checked_mul((self.payload + OVERHEAD) as u64)
            .and_then(|v| v.checked_add(12))
            .is_none_or(|v| v > MAX_FILE_BYTES)
        {
            return Err(invalid("run exceeds the 512MiB file budget"));
        }
        Ok(())
    }
}
fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}
fn mismatch(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn ns(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos()).expect("bounded measurement duration fits u64 nanoseconds")
}
struct Floor {
    file: File,
    stopped: bool,
}
#[derive(Clone)]
enum Sink {
    Log(Arc<Log>),
    File(Arc<Mutex<Floor>>),
}
impl Sink {
    fn issue(&self, id: u64, payload: usize, synced: bool) -> CallResult {
        let size = payload
            + if matches!(self, Self::File(_)) {
                OVERHEAD
            } else {
                0
            };
        let bytes = match make_payload(id, size) {
            Ok(v) => v,
            Err(_) => return (Outcome::NotSent, None, None),
        };
        match self {
            Self::Log(log) => match log.append(bytes) {
                Ok(record) => (
                    Outcome::Retained,
                    Some(record.seq()),
                    Some(*record.property_hash()),
                ),
                Err(error) => (
                    if error.outcome == AppendOutcome::Unknown {
                        Outcome::Unknown
                    } else {
                        Outcome::Failed
                    },
                    None,
                    None,
                ),
            },
            Self::File(floor) => {
                let mut floor = match floor.lock() {
                    Ok(v) => v,
                    Err(_) => return (Outcome::Failed, None, None),
                };
                if floor.stopped {
                    return (Outcome::Failed, None, None);
                }
                if floor
                    .file
                    .write_all(&bytes)
                    .and_then(|()| {
                        if synced {
                            floor.file.sync_all()
                        } else {
                            Ok(())
                        }
                    })
                    .is_err()
                {
                    floor.stopped = true;
                    (Outcome::Unknown, None, None)
                } else {
                    (Outcome::Retained, None, None)
                }
            }
        }
    }
}

fn write_floor_batch(file: &mut File, frames: &[Vec<u8>]) -> io::Result<()> {
    let mut slices: Vec<_> = frames.iter().map(|bytes| IoSlice::new(bytes)).collect();
    let mut remaining = slices.as_mut_slice();
    while !remaining.is_empty() {
        match file.write_vectored(remaining) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "zero-length batch write",
                ));
            }
            Ok(written) => IoSlice::advance_slices(&mut remaining, written),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => (),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}
impl Sink {
    fn issue_batch(&self, ids: &[u64], payload: usize, synced: bool) -> Vec<CallResult> {
        let width = payload
            + if matches!(self, Self::File(_)) {
                OVERHEAD
            } else {
                0
            };
        let frames = match ids
            .iter()
            .map(|id| make_payload(*id, width))
            .collect::<io::Result<Vec<_>>>()
        {
            Ok(frames) => frames,
            Err(_) => return vec![(Outcome::NotSent, None, None); ids.len()],
        };
        match self {
            Self::Log(log) => log
                .append_batch(frames.into_iter().map(Content::Bytes).collect())
                .into_iter()
                .map(|result| match result {
                    Ok(record) => (
                        Outcome::Retained,
                        Some(record.seq()),
                        Some(*record.property_hash()),
                    ),
                    Err(error) => (
                        if error.outcome == AppendOutcome::Unknown {
                            Outcome::Unknown
                        } else {
                            Outcome::Failed
                        },
                        None,
                        None,
                    ),
                })
                .collect(),
            Self::File(floor) => {
                let mut floor = match floor.lock() {
                    Ok(floor) => floor,
                    Err(_) => return vec![(Outcome::Failed, None, None); ids.len()],
                };
                if floor.stopped {
                    return vec![(Outcome::Failed, None, None); ids.len()];
                }
                let result = write_floor_batch(&mut floor.file, &frames).and_then(|()| {
                    if synced {
                        floor.file.sync_all()
                    } else {
                        Ok(())
                    }
                });
                let outcome = if result.is_ok() {
                    Outcome::Retained
                } else {
                    floor.stopped = true;
                    Outcome::Unknown
                };
                vec![(outcome, None, None); ids.len()]
            }
        }
    }
}

fn measure(args: &Args, sink: &Sink) -> io::Result<(Vec<Sample>, u64)> {
    // Allocate sample storage before releasing producers. Payload allocation and
    // ID stamping happen in issue(), inside each individual operation timer.
    let mut buffers = Vec::new();
    for worker in 0..args.producers {
        let quota =
            (args.count + args.producers as u64 - 1 - worker as u64) / args.producers as u64;
        let mut samples = Vec::new();
        samples
            .try_reserve_exact(quota as usize)
            .map_err(io::Error::other)?;
        buffers.push((worker as u64, quota, samples));
    }
    let start = Arc::new(OnceLock::<Instant>::new());
    let barrier = Arc::new(Barrier::new(args.producers + 1));
    let mut threads = Vec::new();
    for (worker, quota, mut samples) in buffers {
        let (start, barrier, sink) = (Arc::clone(&start), Arc::clone(&barrier), sink.clone());
        let (payload, producers, synced, batch_size) = (
            args.payload,
            args.producers as u64,
            args.synced,
            args.batch_size,
        );
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            let start = *start.get().expect("origin set before barrier");
            if batch_size == 1 {
                for ordinal in 0..quota {
                    let id = worker + ordinal * producers;
                    let start_ns = ns(start.elapsed());
                    let (outcome, seq, hash) = sink.issue(id, payload, synced);
                    let end_ns = ns(start.elapsed());
                    samples.push(Sample {
                        id,
                        due_ns: start_ns,
                        start_ns,
                        end_ns,
                        outcome,
                        seq,
                        hash,
                    });
                }
            } else {
                let mut ordinal = 0;
                while ordinal < quota {
                    let start_ns = ns(start.elapsed());
                    let count = (quota - ordinal).min(batch_size as u64);
                    let ids: Vec<_> = (ordinal..ordinal + count)
                        .map(|n| worker + n * producers)
                        .collect();
                    let results = sink.issue_batch(&ids, payload, synced);
                    let end_ns = ns(start.elapsed());
                    assert_eq!(results.len(), ids.len(), "batch result count");
                    for (id, (outcome, seq, hash)) in ids.into_iter().zip(results) {
                        samples.push(Sample {
                            id,
                            due_ns: start_ns,
                            start_ns,
                            end_ns,
                            outcome,
                            seq,
                            hash,
                        });
                    }
                    ordinal += count;
                }
            }
            samples
        }));
    }
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(args.count as usize)
        .map_err(io::Error::other)?;
    let mut completed = Vec::new();
    completed
        .try_reserve_exact(args.producers)
        .map_err(io::Error::other)?;
    start.set(Instant::now()).expect("single start origin");
    barrier.wait();
    for thread in threads {
        completed.push(
            thread
                .join()
                .map_err(|_| io::Error::other("producer panicked"))?,
        );
    }
    // No aggregate allocation, bulk sample copies, sorting or artifact I/O
    // competes with producers before their final completion timestamp.
    let elapsed = completed
        .iter()
        .flatten()
        .map(|s| s.end_ns)
        .max()
        .unwrap_or(0);
    for worker_samples in completed {
        samples.extend(worker_samples);
    }
    samples.sort_unstable_by_key(|s| s.id);
    if samples.len() != args.count as usize
        || samples.iter().enumerate().any(|(i, s)| s.id != i as u64)
    {
        return Err(mismatch("offered sample IDs are missing or duplicated"));
    }
    Ok((samples, elapsed))
}

type RetainedReceipt = (u64, Option<u64>, Option<[u8; 32]>);
fn verify_receipts(samples: &[Sample], retained: &[RetainedReceipt]) -> io::Result<(u64, u64)> {
    let mut seen = BTreeSet::new();
    let mut server_only = 0;
    for &(id, seq, hash) in retained {
        if !seen.insert(id) {
            return Err(mismatch("duplicate retained workload ID"));
        }
        let sample = samples
            .get(id as usize)
            .filter(|s| s.id == id)
            .ok_or_else(|| mismatch("retained workload ID outside offered set"))?;
        if matches!(sample.outcome, Outcome::Failed | Outcome::NotSent) {
            return Err(mismatch("known failed/not-sent work appears retained"));
        }
        if sample.outcome == Outcome::Retained && (sample.seq != seq || sample.hash != hash) {
            return Err(mismatch(
                "receipt sequence/hash differs from retained record",
            ));
        }
        if sample.outcome == Outcome::Unknown {
            server_only += 1;
        }
    }
    if samples
        .iter()
        .any(|s| s.outcome == Outcome::Retained && !seen.contains(&s.id))
    {
        return Err(mismatch("successful sample has no retained content"));
    }
    Ok((seen.len() as u64, server_only))
}
fn reconcile(
    args: &Args,
    sink: &Sink,
    data_path: &std::path::Path,
    samples: &[Sample],
) -> io::Result<(u64, u64)> {
    let mut retained = Vec::new();
    match sink {
        Sink::Log(log) => {
            let page = log
                .query(Query {
                    after: None,
                    through: None,
                    limit: (args.count + 1) as u32,
                })
                .map_err(io::Error::other)?;
            // One extra slot exposes an unexpected suffix without a second scan.
            if page.entries.len() > args.count as usize {
                return Err(mismatch("more retained records than offered IDs"));
            }
            for entry in page.entries {
                let [(key, Value::Bytes(bytes))] = entry.properties.as_slice() else {
                    return Err(mismatch(
                        "native retained properties differ from byte-content mapping",
                    ));
                };
                if key != "content" {
                    return Err(mismatch("unexpected native property key"));
                }
                let id = verify_payload(bytes, args.payload)?;
                retained.push((
                    id,
                    Some(entry.record.seq()),
                    Some(*entry.record.property_hash()),
                ));
            }
        }
        Sink::File(_) => {
            let mut file = File::open(data_path)?;
            let width = args.payload + OVERHEAD;
            let length = file.metadata()?.len();
            if length % width as u64 != 0 && !samples.iter().any(|s| s.outcome == Outcome::Unknown)
            {
                return Err(mismatch("file floor has unexplained partial record bytes"));
            }
            let mut bytes = vec![0; width];
            for _ in 0..length / width as u64 {
                file.read_exact(&mut bytes)?;
                retained.push((verify_payload(&bytes, width)?, None, None));
            }
        }
    }
    verify_receipts(samples, &retained)
}
fn command(command: &str, arguments: &[&str]) -> String {
    std::process::Command::new(command)
        .args(arguments)
        .output()
        .ok()
        .filter(|v| v.status.success())
        .map(|v| String::from_utf8_lossy(&v.stdout).trim().to_owned())
        .unwrap_or_else(|| "unavailable".into())
}
fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = args.out.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::create_dir(&args.out)?; // Never reuse or overwrite a run directory.
    let data_path = args.out.join(if args.mode == Mode::Log {
        "append.dlz"
    } else {
        "floor.bin"
    });
    let sink = match args.mode {
        Mode::Log => Sink::Log(Arc::new(Log::open(
            &data_path,
            Options {
                completion: if args.synced {
                    Completion::Synced
                } else {
                    Completion::Written
                },
                batch: BatchOptions {
                    max_records: args.batch_records,
                    max_bytes: args.batch_bytes,
                    max_delay: Duration::from_micros(args.batch_delay_us),
                    ..BatchOptions::default()
                },
                max_query_records: (args.count + 1) as u32,
                max_query_bytes: (args.count + 1) * (args.payload as u64 + 32),
                ..Options::default()
            },
        )?)),
        Mode::File => Sink::File(Arc::new(Mutex::new(Floor {
            file: OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&data_path)?,
            stopped: false,
        }))),
    };
    let (samples, elapsed_ns) = measure(&args, &sink)?;
    write_samples(args.out.join("samples.tsv"), &samples)?;
    let reconciliation_start = Instant::now();
    let (retained, server_only) = reconcile(&args, &sink, &data_path, &samples)?;
    let reconciliation_ns = ns(reconciliation_start.elapsed());
    let count = |outcome| samples.iter().filter(|s| s.outcome == outcome).count() as u64;
    let (succeeded, failed, unknown, not_sent) = (
        count(Outcome::Retained),
        count(Outcome::Failed),
        count(Outcome::Unknown),
        count(Outcome::NotSent),
    );
    if succeeded + failed + unknown + not_sent != args.count {
        return Err(mismatch("sample population differs from offered count").into());
    }
    let mode = if args.mode == Mode::Log {
        "log"
    } else {
        "file_floor"
    };
    let rate = |number: u64| format!("{:.3}", number as f64 * 1e9 / elapsed_ns.max(1) as f64);
    let file_bytes = data_path.metadata()?.len();
    let floor_tail_bytes = if args.mode == Mode::File {
        file_bytes % (args.payload + OVERHEAD) as u64
    } else {
        0
    };
    let mut summary: Vec<(String, String)> = vec![
        ("mode".into(), mode.into()),
        ("arrival_model".into(), "closed_loop".into()),
        ("clock".into(), "std::time::Instant".into()),
        ("warmup".into(), "none".into()),
        (
            "operation_timer".into(),
            if args.batch_size == 1 {
                "input_allocation_id_stamp_and_complete_call"
            } else {
                "bulk_input_preparation_through_batch_return_per_member"
            }
            .into(),
        ),
        (
            "throughput_window".into(),
            "pre_barrier_origin_through_last_completion_including_intercall_sample_storage".into(),
        ),
        (
            "boundary".into(),
            if args.synced {
                "file_synced"
            } else {
                "written"
            }
            .into(),
        ),
        ("payload_bytes".into(), args.payload.to_string()),
        ("producers".into(), args.producers.to_string()),
        ("producer_batch_size".into(), args.batch_size.to_string()),
        (
            "producer_calls".into(),
            (0..args.producers as u64)
                .map(|worker| {
                    let quota =
                        (args.count + args.producers as u64 - 1 - worker) / args.producers as u64;
                    quota.div_ceil(args.batch_size as u64)
                })
                .sum::<u64>()
                .to_string(),
        ),
        ("batch_max_records".into(), args.batch_records.to_string()),
        ("batch_max_bytes".into(), args.batch_bytes.to_string()),
        ("batch_max_delay_us".into(), args.batch_delay_us.to_string()),
        ("offered".into(), args.count.to_string()),
        ("issued".into(), (args.count - not_sent).to_string()),
        ("not_sent".into(), not_sent.to_string()),
        ("completed".into(), (args.count - not_sent).to_string()),
        ("pending".into(), "0".into()),
        ("client_retained".into(), succeeded.to_string()),
        ("client_failed".into(), failed.to_string()),
        ("client_unknown".into(), unknown.to_string()),
        ("verified_complete_records".into(), retained.to_string()),
        ("server_only_retained".into(), server_only.to_string()),
        (
            "incomplete_records".into(),
            u64::from(floor_tail_bytes != 0).to_string(),
        ),
        (
            "incomplete_file_floor_bytes".into(),
            floor_tail_bytes.to_string(),
        ),
        ("workload_ns".into(), elapsed_ns.to_string()),
        ("drain_ns".into(), "0".into()),
        ("reconciliation_ns".into(), reconciliation_ns.to_string()),
        ("offered_per_s".into(), rate(args.count)),
        ("verified_retained_per_s".into(), rate(retained)),
        (
            "logical_bytes".into(),
            (retained * args.payload as u64).to_string(),
        ),
        ("file_bytes".into(), file_bytes.to_string()),
        ("runtime_workers".into(), "0".into()),
        ("connection_model".into(), "native_threads".into()),
        (
            "grouping_model".into(),
            if args.mode == Mode::File {
                "explicit_producer_groups_only"
            } else if args.synced {
                "bounded_sync_group_commit"
            } else {
                "caller_single_or_explicit_bulk_write"
            }
            .into(),
        ),
        ("os".into(), std::env::consts::OS.into()),
        ("arch".into(), std::env::consts::ARCH.into()),
        (
            "build".into(),
            if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
            .into(),
        ),
        ("rustc".into(), command("rustc", &["--version"])),
        (
            "source_revision".into(),
            command("git", &["rev-parse", "HEAD"]),
        ),
        ("verification".into(), "passed".into()),
    ];
    if args.mode == Mode::Log {
        summary.push((
            "server_counter_source".into(),
            "observed_native_public_call_results_and_posttiming_query".into(),
        ));
        for (key, value) in [
            ("server_arrived", args.count - not_sent),
            ("server_active", 0),
            ("server_retained", succeeded),
            ("server_failed", failed),
            ("server_unknown", unknown),
            ("server_retained_records", retained),
        ] {
            summary.push((key.into(), value.to_string()));
        }
    }
    for (label, outcome) in [
        ("all_completed", None),
        ("retained", Some(Outcome::Retained)),
        ("failed", Some(Outcome::Failed)),
        ("unknown", Some(Outcome::Unknown)),
    ] {
        let values: Vec<_> = samples
            .iter()
            .filter(|s| outcome.map_or(s.outcome != Outcome::NotSent, |o| s.outcome == o))
            .map(Sample::service_ns)
            .collect();
        summary.push((format!("{label}_count"), values.len().to_string()));
        for percentile in [50, 95, 99, 100] {
            summary.push((
                if percentile == 100 {
                    format!("{label}_service_max_ns")
                } else {
                    format!("{label}_service_p{percentile}_ns")
                },
                nearest_rank(&values, percentile)
                    .map(|v| v.to_string())
                    .unwrap_or_default(),
            ));
        }
    }
    write_summary(
        args.out.join("summary.tsv"),
        &summary
            .iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect::<Vec<_>>(),
    )?;
    println!(
        "{mode}: {} offered, {retained} verified; {} ns workload",
        args.count, elapsed_ns
    );
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(Args::parse(std::env::args().skip(1))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample(outcome: Outcome) -> Sample {
        Sample {
            id: 0,
            due_ns: 0,
            start_ns: 0,
            end_ns: 10,
            outcome,
            seq: Some(7),
            hash: Some([3; 32]),
        }
    }
    #[test]
    fn reconciliation_rejects_duplicate_missing_unknown_ids_and_wrong_receipts() {
        let samples = [sample(Outcome::Retained)];
        let good = (0, Some(7), Some([3; 32]));
        assert_eq!(verify_receipts(&samples, &[good]).unwrap(), (1, 0));
        assert!(verify_receipts(&samples, &[]).is_err());
        assert!(verify_receipts(&samples, &[good, good]).is_err());
        assert!(verify_receipts(&samples, &[(1, Some(7), Some([3; 32]))]).is_err());
        assert!(verify_receipts(&samples, &[(0, Some(8), Some([3; 32]))]).is_err());
        assert!(verify_receipts(&samples, &[(0, Some(7), Some([4; 32]))]).is_err());
        assert!(verify_receipts(&[sample(Outcome::Failed)], &[good]).is_err());
        assert!(verify_receipts(&[sample(Outcome::NotSent)], &[good]).is_err());
        assert_eq!(
            verify_receipts(&[sample(Outcome::Unknown)], &[good]).unwrap(),
            (1, 1)
        );
    }
    #[test]
    fn cli_rejects_zero_overflow_and_excessive_run_budget() {
        let base = |count, payload, producers| Args {
            mode: Mode::Log,
            out: PathBuf::from("unused"),
            count,
            payload,
            producers,
            synced: false,
            batch_size: 1,
            batch_records: 256,
            batch_bytes: 1024 * 1024,
            batch_delay_us: 1000,
        };
        for args in [
            base(0, 128, 1),
            base(10, 7, 1),
            base(10, 128, 0),
            base(10, 128, 11),
            base(u64::MAX, 128, 1),
            base(1_000_000, 1_048_576, 1),
        ] {
            assert!(args.validate().is_err());
        }
        assert!(base(50_000, 128, 4).validate().is_ok());
        assert!(base(20_000, 4096, 4).validate().is_ok());
    }
    #[test]
    fn tiny_log_and_file_runs_reconcile_and_refuse_overwrite() {
        let root = PathBuf::from("target/measurements").join(format!(
            "native-functional-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        for (name, mode, synced, batch_size) in [
            ("log", Mode::Log, false, 1),
            ("floor", Mode::File, true, 1),
            ("log-batch", Mode::Log, true, 3),
            ("floor-batch", Mode::File, true, 3),
        ] {
            let out = root.join(name);
            let args = Args {
                mode,
                out: out.clone(),
                count: 8,
                payload: 128,
                producers: 3,
                synced,
                batch_size,
                batch_records: 256,
                batch_bytes: 1024 * 1024,
                batch_delay_us: 1000,
            };
            run(args).unwrap();
            let samples = measure_support::read_samples(out.join("samples.tsv")).unwrap();
            assert_eq!(samples.len(), 8);
            assert!(
                samples
                    .iter()
                    .all(|s| s.outcome == Outcome::Retained && s.start_ns == s.due_ns)
            );
            let summary = std::fs::read_to_string(out.join("summary.tsv")).unwrap();
            assert!(summary.contains("verified_complete_records\t8\n"));
            assert!(summary.contains("pending\t0\n"));
            let data = out.join(if mode == Mode::Log {
                "append.dlz"
            } else {
                "floor.bin"
            });
            assert_eq!(
                data.metadata().unwrap().len(),
                8 * (128 + OVERHEAD) as u64 + if mode == Mode::Log { 12 } else { 0 }
            );
            let before = std::fs::read(&data).unwrap();
            assert!(
                run(Args {
                    mode,
                    out,
                    count: 8,
                    payload: 128,
                    producers: 3,
                    synced,
                    batch_size,
                    batch_records: 256,
                    batch_bytes: 1024 * 1024,
                    batch_delay_us: 1000,
                })
                .is_err()
            );
            assert_eq!(std::fs::read(data).unwrap(), before);
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[cfg(test)]
mod extra_record_regression {
    use super::*;
    #[test]
    fn regression_extra_native_record_is_not_censored_by_query_limit() {
        let out = PathBuf::from("target/measurements").join(format!(
            "extra-native-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&out).unwrap();
        let path = out.join("append.dlz");
        let log = Arc::new(
            Log::open(
                &path,
                Options {
                    max_query_records: 2,
                    max_query_bytes: 320,
                    ..Options::default()
                },
            )
            .unwrap(),
        );
        let receipt = log.append(make_payload(0, 128).unwrap()).unwrap();
        log.append(make_payload(1, 128).unwrap()).unwrap();
        let samples = vec![Sample {
            id: 0,
            due_ns: 0,
            start_ns: 0,
            end_ns: 1,
            outcome: Outcome::Retained,
            seq: Some(receipt.seq()),
            hash: Some(*receipt.property_hash()),
        }];
        let args = Args {
            mode: Mode::Log,
            out: out.clone(),
            count: 1,
            payload: 128,
            producers: 1,
            synced: false,
            batch_size: 1,
            batch_records: 256,
            batch_bytes: 1024 * 1024,
            batch_delay_us: 1000,
        };
        let sink = Sink::Log(log);
        let result = reconcile(&args, &sink, &path, &samples);
        drop(sink);
        std::fs::remove_dir_all(out).unwrap();
        assert!(
            result.is_err(),
            "an extra retained record must not be hidden by the result limit"
        );
    }
}
