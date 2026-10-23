//! Release-mode real-network measurement; see docs/measurement.md.
mod measure_support;

use deleuze::gates::{Gate, GateOptions};
use deleuze::{BatchOptions, Completion, Entry, Log, Options, Query, Value};
use measure_support::{Outcome, Sample};
use std::collections::BTreeMap;
use std::io::{self, BufRead, Read};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}
struct Config {
    protocol: String,
    output: PathBuf,
    count: usize,
    size: usize,
    rate: usize,
    outstanding: usize,
    gate_capacity: usize,
    workers: usize,
    synced: bool,
    batch_records: usize,
    batch_bytes: usize,
    batch_delay_us: u64,
}
impl Config {
    fn parse() -> Result<Self, Box<dyn std::error::Error>> {
        let mut pairs = BTreeMap::new();
        let mut args = std::env::args().skip(1);
        while let Some(key) = args.next() {
            let value = args
                .next()
                .ok_or_else(|| invalid("each option requires a value"))?;
            if pairs.insert(key, value).is_some() {
                return Err(invalid("duplicate option").into());
            }
        }
        let protocol = pairs.remove("--protocol").unwrap_or_else(|| "http".into());
        let output = PathBuf::from(
            pairs
                .remove("--out-dir")
                .ok_or_else(|| invalid("--out-dir required"))?,
        );
        let count = pairs
            .remove("--count")
            .unwrap_or_else(|| "10000".into())
            .parse::<usize>()?;
        let size = pairs
            .remove("--payload-bytes")
            .unwrap_or_else(|| "128".into())
            .parse::<usize>()?;
        let rate = pairs
            .remove("--rate")
            .unwrap_or_else(|| "5000".into())
            .parse::<usize>()?;
        let outstanding = pairs
            .remove("--outstanding")
            .unwrap_or_else(|| "128".into())
            .parse::<usize>()?;
        let gate_capacity = pairs
            .remove("--gate-in-flight")
            .unwrap_or_else(|| "64".into())
            .parse::<usize>()?;
        let workers = pairs
            .remove("--runtime-workers")
            .unwrap_or_else(|| "4".into())
            .parse::<usize>()?;
        let batch_records = pairs
            .remove("--batch-records")
            .unwrap_or_else(|| "256".into())
            .parse::<usize>()?;
        let batch_bytes = pairs
            .remove("--batch-bytes")
            .unwrap_or_else(|| "1048576".into())
            .parse::<usize>()?;
        let batch_delay_us = pairs
            .remove("--batch-delay-us")
            .unwrap_or_else(|| "1000".into())
            .parse::<u64>()?;
        let synced = match pairs.remove("--sync").as_deref().unwrap_or("written") {
            "written" => false,
            "synced" => true,
            _ => return Err(invalid("--sync written|synced").into()),
        };
        if !pairs.is_empty()
            || !matches!(protocol.as_str(), "http" | "grpc")
            || !(1..=1_000_000).contains(&count)
            || !(8..=1_048_571).contains(&size)
            || !(1..=10_000_000).contains(&rate)
            || !(1..=4096).contains(&outstanding)
            || !(1..=4096).contains(&gate_capacity)
            || !(1..=64).contains(&workers)
            || !(1..=65_536).contains(&batch_records)
            || !(1..=64 * 1024 * 1024).contains(&batch_bytes)
            || batch_delay_us > 1_000_000
            || count
                .checked_mul(size + 108)
                .is_none_or(|n| n > 1024 * 1024 * 1024)
        {
            return Err(invalid("invalid or excessive measurement options").into());
        }
        Ok(Self {
            protocol,
            output,
            count,
            size,
            rate,
            outstanding,
            gate_capacity,
            workers,
            synced,
            batch_records,
            batch_bytes,
            batch_delay_us,
        })
    }
}

fn content(entry: &Entry) -> io::Result<(&[u8], bool)> {
    let Some((key, Value::Bytes(bytes))) = entry.properties.first() else {
        return Err(invalid("unexpected content mapping"));
    };
    if key != "content" {
        return Err(invalid("unexpected content property"));
    }
    let partial = match entry.properties.as_slice() {
        [_] => false,
        [_, (name, Value::Bool(true))] if name == "capture_incomplete" => true,
        _ => return Err(invalid("unexpected additional properties")),
    };
    Ok((bytes, partial))
}

fn latency_fields(summary: &mut BTreeMap<String, String>, name: &str, samples: &[&Sample]) {
    summary.insert(format!("{name}.count"), samples.len().to_string());
    for (metric, values) in [
        (
            "actual",
            samples
                .iter()
                .map(|s| s.end_ns - s.start_ns)
                .collect::<Vec<_>>(),
        ),
        (
            "scheduled",
            samples.iter().map(|s| s.end_ns - s.due_ns).collect(),
        ),
        (
            "generator_lag",
            samples.iter().map(|s| s.start_ns - s.due_ns).collect(),
        ),
    ] {
        for percentile in [50, 95, 99, 100] {
            let value = measure_support::nearest_rank(&values, percentile)
                .map(|n| n.to_string())
                .unwrap_or_default();
            summary.insert(format!("{name}.{metric}.p{percentile}_ns"), value);
        }
    }
}

async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(parent) = config.output.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::create_dir(&config.output)?;
    let output = config.output.canonicalize()?;
    let log = Arc::new(Log::open(
        output.join("append.dlz"),
        Options {
            completion: if config.synced {
                Completion::Synced
            } else {
                Completion::Written
            },
            batch: BatchOptions {
                max_records: config.batch_records,
                max_bytes: config.batch_bytes,
                max_delay: std::time::Duration::from_micros(config.batch_delay_us),
                ..BatchOptions::default()
            },
            max_query_records: config.count as u32 + 1,
            max_query_bytes: (config.count as u64 + 1) * (config.size as u64 + 200),
            ..Options::default()
        },
    )?);
    let gate = Gate::new(
        Arc::clone(&log),
        GateOptions {
            max_in_flight: config.gate_capacity,
            ..GateOptions::default()
        },
    )?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let address = format!("http://{}", listener.local_addr()?);
    let (stop, stopping) = tokio::sync::oneshot::channel();
    let serving_gate = gate.clone();
    let server = tokio::spawn(async move {
        serving_gate
            .serve(listener, async {
                let _ = stopping.await;
            })
            .await
    });
    let script = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("scripts/measure-network.mjs");
    let args = vec![
        config.protocol.clone(),
        address,
        config.count.to_string(),
        config.size.to_string(),
        config.rate.to_string(),
        config.outstanding.to_string(),
        output.join("samples.tsv").to_string_lossy().into_owned(),
    ];
    let wall_start = Instant::now();
    let (phase, completed) = tokio::sync::oneshot::channel();
    let client = tokio::task::spawn_blocking(move || -> io::Result<std::process::Output> {
        let mut child = std::process::Command::new("node")
            .arg(script)
            .args(args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;
        let mut stdout = io::BufReader::new(child.stdout.take().expect("piped stdout"));
        let mut marker = String::new();
        let read = stdout.read_line(&mut marker);
        let phase_result = read.and_then(|_| {
            if marker == "WORKLOAD_COMPLETE\n" {
                Ok(())
            } else {
                Err(invalid("missing client completion marker"))
            }
        });
        let _ = phase.send(phase_result);
        let mut remainder = Vec::new();
        stdout.read_to_end(&mut remainder)?;
        let mut result = child.wait_with_output()?;
        result.stdout = remainder;
        Ok(result)
    });
    let phase_result = completed.await;
    let drain_start = Instant::now();
    let _ = stop.send(());
    server.await??;
    let drain_ns = drain_start.elapsed().as_nanos();
    let wall_ns = wall_start.elapsed().as_nanos();
    // Joining the client and reading exported artifacts are outside capture timing.
    let client = client.await??;
    phase_result??;
    std::fs::write(output.join("client-summary.tsv"), &client.stdout)?;
    std::fs::write(output.join("client-stderr.txt"), &client.stderr)?;
    if !client.status.success() {
        return Err(invalid(format!(
            "load client failed: {}",
            String::from_utf8_lossy(&client.stderr)
        ))
        .into());
    }
    let mut counts = BTreeMap::new();
    for line in std::str::from_utf8(&client.stdout)?.lines() {
        let (key, value) = line
            .split_once('\t')
            .ok_or_else(|| invalid("bad client summary"))?;
        if counts
            .insert(key.to_owned(), value.parse::<u64>()?)
            .is_some()
        {
            return Err(invalid("duplicate client summary field").into());
        }
    }
    let number = |key: &str| -> io::Result<u64> {
        counts
            .get(key)
            .copied()
            .ok_or_else(|| invalid(format!("missing client field {key}")))
    };
    let reconcile_start = Instant::now();
    let samples = measure_support::read_samples(output.join("samples.tsv"))?;
    if samples.len() != config.count {
        return Err(invalid("sample count differs from offered").into());
    }
    let mut seen = vec![false; config.count];
    let mut outcomes = [0_u64; 4];
    for sample in &samples {
        let id = usize::try_from(sample.id)?;
        if id >= config.count || std::mem::replace(&mut seen[id], true) {
            return Err(invalid("invalid or duplicate sample ID").into());
        }
        outcomes[match sample.outcome {
            Outcome::Retained => 0,
            Outcome::Failed => 1,
            Outcome::Unknown => 2,
            Outcome::NotSent => 3,
        }] += 1;
    }
    let issued = number("issued")?;
    if number("offered")? != config.count as u64
        || issued + number("not_sent")? != config.count as u64
        || number("completed")? != issued
        || number("pending")? != 0
        || number("protocol_errors")? != 0
        || outcomes
            != [
                number("client_retained")?,
                number("client_failed")?,
                number("client_unknown")?,
                number("not_sent")?,
            ]
    {
        return Err(invalid("client population contradiction").into());
    }
    let stats = gate.stats();
    if stats.active != 0
        || !stats.closed
        || stats.arrived != stats.retained + stats.failed + stats.unknown
        || stats.arrived > issued
    {
        return Err(invalid("server population contradiction").into());
    }
    let entries = log
        .query(Query {
            limit: config.count as u32 + 1,
            ..Query::default()
        })?
        .entries;
    if entries.len() as u64 != stats.retained_records {
        return Err(invalid("retained record count contradiction").into());
    }
    let mut retained = vec![false; config.count];
    let mut complete = vec![false; config.count];
    let mut partial_count = 0_u64;
    let mut unidentified_partial = 0_u64;
    let by_id: BTreeMap<_, _> = samples.iter().map(|sample| (sample.id, sample)).collect();
    for entry in &entries {
        let (bytes, partial) = content(entry)?;
        if partial {
            partial_count += 1;
            if bytes.len() > config.size
                || bytes
                    .get(8..)
                    .is_some_and(|tail| tail.iter().any(|b| *b != 0xa5))
            {
                return Err(invalid("invalid retained partial pattern").into());
            }
            if bytes.len() < 8 {
                unidentified_partial += 1;
                continue;
            }
        }
        let id = if partial {
            u64::from_le_bytes(bytes[..8].try_into()?)
        } else {
            measure_support::verify_payload(bytes, config.size)?
        };
        let index = usize::try_from(id)?;
        if index >= config.count
            || std::mem::replace(&mut retained[index], true)
            || by_id[&id].outcome == Outcome::NotSent
        {
            return Err(invalid("unexpected or duplicate retained workload ID").into());
        }
        complete[index] = !partial;
    }
    for sample in &samples {
        if sample.outcome == Outcome::Retained
            && (!complete[sample.id as usize] || sample.seq.is_none() || sample.hash.is_none())
        {
            return Err(invalid("retained receipt lacks complete native record").into());
        }
        if let Some(seq) = sample.seq {
            let entry = seq
                .checked_sub(1)
                .and_then(|index| entries.get(index as usize))
                .ok_or_else(|| invalid("receipt sequence missing"))?;
            let (bytes, _) = content(entry)?;
            if bytes.len() < 8
                || u64::from_le_bytes(bytes[..8].try_into()?) != sample.id
                || sample.hash.as_ref() != Some(entry.record.property_hash())
            {
                return Err(invalid("receipt identity/hash mismatch").into());
            }
        }
    }
    let complete_count = complete.iter().filter(|v| **v).count() as u64;
    let server_only = samples
        .iter()
        .filter(|s| complete[s.id as usize] && s.outcome != Outcome::Retained)
        .count();
    let reconcile_ns = reconcile_start.elapsed().as_nanos();
    let mut summary = BTreeMap::new();
    for (key, value) in &counts {
        summary.insert(key.clone(), value.to_string());
    }
    for (key, value) in [
        ("mode", config.protocol.clone()),
        (
            "boundary",
            if config.synced { "synced" } else { "written" }.into(),
        ),
        ("payload_bytes", config.size.to_string()),
        ("offered_rate_s", config.rate.to_string()),
        ("client_outstanding", config.outstanding.to_string()),
        ("gate_in_flight", config.gate_capacity.to_string()),
        ("runtime_workers", config.workers.to_string()),
        ("batch_max_records", config.batch_records.to_string()),
        ("batch_max_bytes", config.batch_bytes.to_string()),
        ("batch_max_delay_us", config.batch_delay_us.to_string()),
        ("server_arrived", stats.arrived.to_string()),
        ("server_retained", stats.retained.to_string()),
        ("server_failed", stats.failed.to_string()),
        ("server_unknown", stats.unknown.to_string()),
        ("server_active", stats.active.to_string()),
        ("server_records", stats.retained_records.to_string()),
        ("complete_records", complete_count.to_string()),
        ("partial_records", partial_count.to_string()),
        ("unidentified_partial", unidentified_partial.to_string()),
        ("server_only_complete", server_only.to_string()),
        (
            "issued_without_server_notification",
            (issued - stats.arrived).to_string(),
        ),
        ("server_window_ns", wall_ns.to_string()),
        (
            "server_window",
            "client_launch_through_server_drain_excludes_artifact_export".into(),
        ),
        (
            "connection_model",
            if config.protocol == "grpc" {
                "http2_sessions_rollover_for_future_requests"
            } else {
                "http1_keepalive_pool"
            }
            .into(),
        ),
        (
            "arrival_model",
            "absolute_due_times_bounded_client_outstanding".into(),
        ),
        ("drain_ns", drain_ns.to_string()),
        ("reconcile_ns", reconcile_ns.to_string()),
        (
            "server_retained_rate_s",
            format!("{:.3}", stats.retained as f64 * 1e9 / wall_ns as f64),
        ),
        (
            "logical_complete_bytes",
            (complete_count * config.size as u64).to_string(),
        ),
        (
            "file_bytes",
            std::fs::metadata(output.join("append.dlz"))?
                .len()
                .to_string(),
        ),
    ] {
        summary.insert(key.into(), value);
    }
    for (name, cohort) in [
        (
            "all_completed",
            samples
                .iter()
                .filter(|s| s.outcome != Outcome::NotSent)
                .collect::<Vec<_>>(),
        ),
        (
            "retained",
            samples
                .iter()
                .filter(|s| s.outcome == Outcome::Retained)
                .collect(),
        ),
        (
            "failed",
            samples
                .iter()
                .filter(|s| s.outcome == Outcome::Failed)
                .collect(),
        ),
        (
            "unknown",
            samples
                .iter()
                .filter(|s| s.outcome == Outcome::Unknown)
                .collect(),
        ),
        (
            "not_sent",
            samples
                .iter()
                .filter(|s| s.outcome == Outcome::NotSent)
                .collect(),
        ),
    ] {
        latency_fields(&mut summary, name, &cohort);
    }
    let fields: Vec<_> = summary
        .iter()
        .map(|(key, value)| (key.as_str(), value.clone()))
        .collect();
    measure_support::write_summary(output.join("summary.tsv"), &fields)?;
    for (key, value) in summary {
        println!("{key}\t{value}");
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse()?;
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(config.workers)
        .enable_all()
        .build()?;
    runtime.block_on(run(config))
}
