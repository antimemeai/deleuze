//! Release baseline: --mode log|file --out-dir NEW --count N --payload-bytes N
//! --producers N --sync written|synced. Closed-loop call latency, not open-loop
//! response latency. File mode is a frame-sized I/O floor without log semantics.
mod measure_support;

use deleuze::{AppendOutcome, Log, Options, Query, Value};
use measure_support::{
    Outcome, Sample, make_payload, nearest_rank, verify_payload, write_samples, write_summary,
};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::{Arc, Barrier, Mutex, OnceLock};
use std::time::{Duration, Instant};

const OVERHEAD: usize = 108; // Bytes property encoding (32) + physical frame (76).
const MAX_FILE_BYTES: u64 = 512 * 1024 * 1024;
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
}
impl Args {
    fn parse(arguments: impl Iterator<Item = String>) -> io::Result<Self> {
        let mut mode = None;
        let mut out = None;
        let (mut count, mut payload, mut producers, mut synced) = (10_000, 128, 1, false);
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
        };
        result.validate()?;
        Ok(result)
    }
    fn validate(&self) -> io::Result<()> {
        if !(1..=1_000_000).contains(&self.count)
            || !(8..=1024 * 1024).contains(&self.payload)
            || !(1..=128).contains(&self.producers)
            || self.producers as u64 > self.count
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
    fn issue(
        &self,
        id: u64,
        payload: usize,
        synced: bool,
    ) -> (Outcome, Option<u64>, Option<[u8; 32]>) {
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
        let (payload, producers, synced) = (args.payload, args.producers as u64, args.synced);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            let start = *start.get().expect("origin set before barrier");
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
            samples
        }));
    }
    start.set(Instant::now()).expect("single start origin");
    barrier.wait();
    let mut samples = Vec::new();
    samples
        .try_reserve_exact(args.count as usize)
        .map_err(io::Error::other)?;
    for thread in threads {
        samples.extend(
            thread
                .join()
                .map_err(|_| io::Error::other("producer panicked"))?,
        );
    }
    let elapsed = samples.iter().map(|s| s.end_ns).max().unwrap_or(0);
    samples.sort_unstable_by_key(|s| s.id);
    if samples.len() != args.count as usize
        || samples.iter().enumerate().any(|(i, s)| s.id != i as u64)
    {
        return Err(mismatch("offered sample IDs are missing or duplicated"));
    }
    Ok((samples, elapsed))
}

fn verify_receipts(
    samples: &[Sample],
    retained: &[(u64, Option<u64>, Option<[u8; 32]>)],
) -> io::Result<(u64, u64)> {
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
                    limit: args.count as u32,
                })
                .map_err(io::Error::other)?;
            // Exactly-full pages may conservatively carry a continuation. This
            // workload has at most count records; no second query is needed.
            for entry in page.entries {
                let [(_, Value::Bytes(bytes))] = entry.properties.as_slice() else {
                    return Err(mismatch(
                        "native retained properties differ from byte-content mapping",
                    ));
                };
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
                sync_each_append: args.synced,
                max_query_records: args.count as u32,
                max_query_bytes: args.count * (args.payload as u64 + 32),
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
    let mut summary: Vec<(String, String)> = vec![
        ("mode".into(), mode.into()),
        ("arrival_model".into(), "closed_loop".into()),
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
        ("incomplete_records".into(), "0".into()),
        ("workload_ns".into(), elapsed_ns.to_string()),
        ("drain_ns".into(), "0".into()),
        ("reconciliation_ns".into(), reconciliation_ns.to_string()),
        ("offered_per_s".into(), rate(args.count)),
        ("verified_retained_per_s".into(), rate(retained)),
        (
            "logical_bytes".into(),
            (retained * args.payload as u64).to_string(),
        ),
        ("file_bytes".into(), data_path.metadata()?.len().to_string()),
        ("runtime_workers".into(), "0".into()),
        ("connection_model".into(), "native_threads".into()),
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
                format!("{label}_service_p{percentile}_ns"),
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
