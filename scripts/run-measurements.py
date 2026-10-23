#!/usr/bin/env python3
"""Run the first modest, sequential release matrix; artifacts are create-new."""
import argparse
import csv
import hashlib
import os
from pathlib import Path
import platform
import plistlib
import random
import re
import shlex
import subprocess
import sys


def capture(*command):
    result = subprocess.run(command, capture_output=True, text=True)
    return " | ".join(result.stdout.strip().splitlines()) if result.returncode == 0 else "unavailable"


def summary(path):
    with path.open() as source:
        rows = list(csv.reader(source, delimiter="\t"))
    if not rows or rows[0] != ["metric", "value"]:
        raise RuntimeError(f"invalid summary: {path}")
    result = dict(rows[1:])
    if len(result) != len(rows) - 1:
        raise RuntimeError("duplicate summary metrics")
    return result


def normalized(name, rep, kind, metrics, resources):
    def value(*keys):
        return next((metrics[key] for key in keys if key in metrics), "")
    row = {key: value(key) for key in [
        "mode", "boundary", "payload_bytes", "producers", "runtime_workers", "gate_in_flight",
        "client_outstanding", "producer_batch_size", "producer_calls", "batch_max_records", "batch_max_bytes", "batch_max_delay_us", "grouping_model", "offered_rate_s", "offered", "issued", "not_sent", "completed", "pending",
        "client_retained", "client_failed", "client_unknown", "server_arrived", "server_retained",
        "server_failed", "server_unknown", "server_active", "partial_records", "server_only_complete",
        "issued_without_server_notification", "file_bytes", "drain_ns",
    ]}
    row.update(run=name, repetition=rep, kind=kind,
        boundary="synced" if value("boundary") == "file_synced" else value("boundary"),
        complete_records=value("verified_complete_records", "complete_records"),
        incomplete_records=value("incomplete_records", "partial_records"),
        retained_rate_s=value("verified_retained_per_s", "server_retained_rate_s"),
        window_ns=value("workload_ns", "server_window_ns"),
        window=value("throughput_window", "server_window"),
        reconciliation_ns=value("reconciliation_ns", "reconcile_ns"),
        retained_p50_ns=value("retained_service_p50_ns", "retained.actual.p50_ns"),
        retained_p95_ns=value("retained_service_p95_ns", "retained.actual.p95_ns"),
        retained_p99_ns=value("retained_service_p99_ns", "retained.actual.p99_ns"),
        retained_max_ns=value("retained_service_max_ns", "retained.actual.p100_ns"),
        retained_scheduled_p99_ns=value("retained.scheduled.p99_ns"),
        generator_lag_p99_ns=value("all_completed.generator_lag.p99_ns"),
        logical_bytes=value("logical_bytes", "logical_complete_bytes"),
        counter_source=value("server_counter_source") or ("file_floor_no_server" if kind == "native" else "gate_stats_after_drain"),
    )
    cpu = re.search(r"([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys", resources)
    if cpu:
        row.update(process_real_s=cpu[1], process_user_s=cpu[2], process_system_s=cpu[3])
    rss = re.search(r"(\d+)\s+maximum resident set size", resources)
    row["time_reported_maxrss"] = rss[1] if rss else ""
    return row


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--repetitions", type=int, default=3)
    parser.add_argument("--suite", choices=["baseline", "gate-capacity", "batching"], default="baseline")
    args = parser.parse_args()
    if not 1 <= args.repetitions <= 10:
        parser.error("repetitions must be1..10")
    if subprocess.check_output(["git", "status", "--porcelain"]):
        parser.error("commit the measurement source before running the matrix")
    args.out_dir.parent.mkdir(parents=True, exist_ok=True)
    args.out_dir.mkdir()
    metadata = {
        "source_revision": capture("git", "rev-parse", "HEAD"),
        "cargo_lock_sha256": hashlib.sha256(Path("Cargo.lock").read_bytes()).hexdigest(),
        "rustc": capture("rustc", "-Vv"), "node": capture("node", "--version"),
        "os": f"{platform.system()} {platform.release()}", "arch": platform.machine(),
        "free_bytes_before": str(os.statvfs(".").f_bavail * os.statvfs(".").f_frsize),
        "native_build": "cargo build --release --example measure_native",
        "network_build": "cargo build --release --features gates --example measure_gates",
        "warmup": "none; fresh process per run", "order_seed": "108", "suite": args.suite,
        "environment_control": "same host; no affinity, cache flush or background-load isolation",
        "resource_scope": "time process totals include artifact export and reconciliation; not capture-only CPU",
    }
    if sys.platform == "darwin":
        for key in ["hw.model", "hw.memsize", "hw.physicalcpu", "hw.logicalcpu", "machdep.cpu.brand_string"]:
            metadata[key] = capture("sysctl", "-n", key)
        metadata["macos"] = capture("sw_vers", "-productVersion")
        disk = subprocess.run(["diskutil", "info", "-plist", "/System/Volumes/Data"], capture_output=True)
        if disk.returncode == 0:
            values = plistlib.loads(disk.stdout)
            for key in ["FilesystemType", "Internal", "SolidState", "TotalSize", "APFSContainerFree"]:
                if key in values:
                    metadata[f"storage.{key}"] = str(values[key])
    subprocess.run(shlex.split(metadata["native_build"]), check=True)
    subprocess.run(shlex.split(metadata["network_build"]), check=True)
    cases = []
    for rep in range(1, args.repetitions + 1):
        for size, count in [(128, 50_000), (4096, 10_000)]:
            for producers in [1, 4]:
                for mode in ["log", "file"]:
                    cases.append((f"native-{mode}-{size}-p{producers}-written-r{rep}", rep, "native", [
                        "target/release/examples/measure_native", "--mode", mode, "--count", str(count),
                        "--payload-bytes", str(size), "--producers", str(producers), "--sync", "written"]))
        for producers in [1, 4]:
            for mode in ["log", "file"]:
                cases.append((f"native-{mode}-128-p{producers}-synced-r{rep}", rep, "native", [
                    "target/release/examples/measure_native", "--mode", mode, "--count", "1000",
                    "--payload-bytes", "128", "--producers", str(producers), "--sync", "synced"]))
        for protocol in ["http", "grpc"]:
            profiles = [("normal", rate * 2, rate, 128, 64, "written") for rate in [1000, 10_000, 40_000]]
            profiles += [("client-cap", 3000, 100_000, 1, 64, "written"),
                         ("gate-cap", 5000, 10_000, 128, 1, "written"),
                         ("synced", 1000, 1000, 128, 64, "synced")]
            for label, count, rate, cap, gate, sync in profiles:
                cases.append((f"{protocol}-{label}-{rate}-r{rep}", rep, label, [
                    "target/release/examples/measure_gates", "--protocol", protocol, "--count", str(count),
                    "--payload-bytes", "128", "--rate", str(rate), "--outstanding", str(cap),
                    "--gate-in-flight", str(gate), "--sync", sync, "--runtime-workers", "4"]))
    if args.suite == "gate-capacity":
        cases = []
        for rep in range(1, args.repetitions + 1):
            for protocol in ["http", "grpc"]:
                for rate in [10_000, 40_000]:
                    for gate in [64, 256]:
                        cases.append((f"{protocol}-capacity-{gate}-{rate}-r{rep}", rep, "capacity", [
                            "target/release/examples/measure_gates", "--protocol", protocol,
                            "--count", str(rate * 2), "--payload-bytes", "128", "--rate", str(rate),
                            "--outstanding", "128", "--gate-in-flight", str(gate), "--sync", "written",
                            "--runtime-workers", "4"]))
    if args.suite == "batching":
        cases = []
        for rep in range(1, args.repetitions + 1):
            for mode in ["log", "file"]:
                for producers, batch_size in [(1, 1), (4, 1), (1, 16), (1, 64), (1, 256), (1, 1024)]:
                    cases.append((f"written-{mode}-p{producers}-batch{batch_size}-r{rep}", rep, "native", [
                        "target/release/examples/measure_native", "--mode", mode, "--count", "262144" if batch_size == 1024 else "65536", "--payload-bytes", "128",
                        "--producers", str(producers), "--sync", "written", "--batch-size", str(batch_size), "--batch-records", str(max(256, batch_size)), "--batch-delay-us", "0"]))
                for batch_size in [1, 16, 64, 256, 1024]:
                    cases.append((f"synced-{mode}-explicit{batch_size}-r{rep}", rep, "native", [
                        "target/release/examples/measure_native", "--mode", mode, "--count", "1000" if batch_size == 1 else str(batch_size * 256),
                        "--payload-bytes", "128", "--producers", "1", "--sync", "synced", "--batch-size", str(batch_size),
                        "--batch-records", "1" if batch_size == 1 else str(max(256, batch_size)), "--batch-delay-us", "0"]))
            for producers in [4, 32, 64]:
                cases.append((f"synced-log-auto-p{producers}-r{rep}", rep, "native", [
                    "target/release/examples/measure_native", "--mode", "log", "--count", "4096", "--payload-bytes", "128",
                    "--producers", str(producers), "--sync", "synced", "--batch-size", "1", "--batch-delay-us", "1000"]))
            for protocol in ["http", "grpc"]:
                for sync, rate, cap, gate in [("written", 1000, 128, 64), ("written", 40000, 128, 64),
                                               ("synced", 1000, 128, 64), ("synced", 10000, 128, 64), ("synced", 40000, 512, 256)]:
                    cases.append((f"{protocol}-{sync}-{rate}-gate{gate}-r{rep}", rep, "batch-network", [
                        "target/release/examples/measure_gates", "--protocol", protocol, "--count", str(rate * 2),
                        "--payload-bytes", "128", "--rate", str(rate), "--outstanding", str(cap), "--gate-in-flight", str(gate),
                        "--sync", sync, "--runtime-workers", "4", "--batch-records", "256", "--batch-delay-us", "1000"]))
    random.Random(108).shuffle(cases)
    rows = []
    with (args.out_dir / "commands.tsv").open("x") as manifest:
        manifest.write("run\trepetition\tkind\tcommand\n")
        for index, (name, rep, kind, command) in enumerate(cases, 1):
            out = args.out_dir / name
            command += ["--out-dir", str(out)]
            manifest.write(f"{name}\t{rep}\t{kind}\t{shlex.join(command)}\n")
            manifest.flush()
            wrapper = ["/usr/bin/time", "-l"] if sys.platform == "darwin" else []
            result = subprocess.run(wrapper + command, capture_output=True, text=True)
            # Runner creates its directory; preserve failure output too.
            out.mkdir(exist_ok=True)
            (out / "stdout.txt").write_text(result.stdout)
            (out / "resources.txt").write_text(result.stderr)
            if result.returncode:
                raise RuntimeError(f"run failed: {name}\n{result.stderr}")
            metrics = summary(out / "summary.tsv")
            if metrics["pending"] != "0":
                raise RuntimeError("pending work after run")
            if kind == "client-cap" and int(metrics["not_sent"]) == 0:
                raise RuntimeError("client-cap fixture did not exercise not-sent work")
            if kind == "gate-cap" and int(metrics["server_failed"]) == 0:
                raise RuntimeError("gate-cap fixture did not exercise rejection")
            row = normalized(name, rep, kind, metrics, result.stderr)
            rows.append(row)
            print(f"{index}/{len(cases)} {name}: {row['complete_records']} records, {row['retained_rate_s']}/s", flush=True)
    metadata["free_bytes_after"] = str(os.statvfs(".").f_bavail * os.statvfs(".").f_frsize)
    with (args.out_dir / "environment.tsv").open("x") as output:
        output.write("metric\tvalue\n")
        for key, value in metadata.items():
            output.write(f"{key}\t{value}\n")
    with (args.out_dir / "results.csv").open("x", newline="") as output:
        writer = csv.DictWriter(output, fieldnames=sorted(set().union(*(row.keys() for row in rows))), lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)
    print(f"results: {args.out_dir / 'results.csv'}", flush=True)


if __name__ == "__main__":
    main()
