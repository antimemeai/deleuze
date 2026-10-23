#![cfg(feature = "gates")]

use deleuze::gates::{Gate, GateOptions};
use deleuze::{Log, Options, Query, Value};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use tokio::net::TcpListener;

struct Temp(PathBuf);
impl Drop for Temp {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn client(language: &str, mode: &str, address: &str, payload: &[u8]) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let (program, script) = match language {
        "python" => ("python3", "examples/clients/capture.py"),
        "node" => ("node", "examples/clients/capture.mjs"),
        _ => unreachable!(),
    };
    let mut child = Command::new(program)
        .arg(root.join(script))
        .args([mode, address])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("install Node and Python with grpcio before running this test");
    child.stdin.take().unwrap().write_all(payload).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{language} {mode} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Opt-in because these are real external language runtimes, not Cargo deps.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "requires Node and Python grpcio; run cargo test --features gates --test polyglot -- --ignored"]
async fn python_and_node_http_and_grpc_retain_exact_native_content() {
    let temp = Temp(std::env::temp_dir().join(format!("deleuze-polyglot-{}", std::process::id())));
    std::fs::create_dir(&temp.0).unwrap();
    let log = Arc::new(Log::open(temp.0.join("events.dlz"), Options::default()).unwrap());
    let gate = Gate::new(Arc::clone(&log), GateOptions::default()).unwrap();
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let (stop, stopping) = tokio::sync::oneshot::channel();
    let server_gate = gate.clone();
    let server = tokio::spawn(async move {
        server_gate
            .serve(listener, async {
                let _ = stopping.await;
            })
            .await
            .unwrap();
    });
    let cases = [
        (
            "python",
            "http",
            b"{\"seq\":999,\"created_at\":-1,\"n\":9007199254740993}".to_vec(),
        ),
        ("python", "grpc", vec![0xff, 0, 0xfe, b'\n']),
        ("node", "http", b"{malformed JSON\0\xff".to_vec()),
        ("node", "grpc", Vec::new()),
    ];
    for (index, (language, mode, payload)) in cases.into_iter().enumerate() {
        let expected = payload.clone();
        let target = address.clone();
        let output = tokio::task::spawn_blocking(move || client(language, mode, &target, &payload))
            .await
            .unwrap();
        let seq = (index + 1) as u64;
        let entry = log.read(seq).unwrap().unwrap();
        assert_eq!(
            entry.properties,
            vec![("content".into(), Value::Bytes(expected))]
        );
        assert_eq!(entry.record.seq(), seq);
        assert!(entry.record.created_at().unix_seconds() > 0);
        let digest: String = entry
            .record
            .property_hash()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(output.trim(), format!("seq={seq} property_hash={digest}"));
    }
    stop.send(()).unwrap();
    server.await.unwrap();
    assert_eq!(log.query(Query::default()).unwrap().entries.len(), 4);
    let stats = gate.stats();
    assert_eq!(
        (stats.arrived, stats.retained, stats.failed, stats.unknown),
        (4, 4, 0, 0)
    );
    assert_eq!(
        (stats.active, stats.retained_records, stats.closed),
        (0, 4, true)
    );
}
