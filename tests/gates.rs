#![cfg(feature = "gates")]

use axum::body::{Body, Bytes};
use axum::http::{HeaderMap, Method, Request, Response, StatusCode, Version};
use deleuze::gates::{Gate, GateOptions};
use deleuze::{Entry, Log, Options, Query, Value};
use http_body::{Body as HttpBody, Frame};
use std::collections::VecDeque;
use std::io;
use std::path::PathBuf;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::task::{Context, Poll};
use std::time::Duration;
use tower_service::Service;

static ID: AtomicU64 = AtomicU64::new(0);
struct Fixture {
    log: Arc<Log>,
    path: PathBuf,
}
impl Fixture {
    fn new(options: Options) -> Self {
        let path = std::env::temp_dir().join(format!(
            "deleuze-gates-{}-{}.dlz",
            std::process::id(),
            ID.fetch_add(1, Ordering::Relaxed)
        ));
        Self {
            log: Arc::new(Log::open(&path, options).unwrap()),
            path,
        }
    }
    fn gate(&self, options: GateOptions) -> Gate {
        Gate::new(Arc::clone(&self.log), options).unwrap()
    }
    fn entries(&self) -> Vec<Entry> {
        self.log.query(Query::default()).unwrap().entries
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}
struct ScriptBody {
    chunks: VecDeque<Result<Bytes, io::Error>>,
    stall: bool,
    polls: Arc<AtomicUsize>,
}
impl HttpBody for ScriptBody {
    type Data = Bytes;
    type Error = io::Error;
    fn poll_frame(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
    ) -> Poll<Option<Result<Frame<Bytes>, io::Error>>> {
        self.polls.fetch_add(1, Ordering::Relaxed);
        if let Some(chunk) = self.chunks.pop_front() {
            Poll::Ready(Some(chunk.map(Frame::data)))
        } else if self.stall {
            Poll::Pending
        } else {
            Poll::Ready(None)
        }
    }
}
fn script(chunks: Vec<Result<Bytes, io::Error>>, stall: bool) -> (Body, Arc<AtomicUsize>) {
    let polls = Arc::new(AtomicUsize::new(0));
    (
        Body::new(ScriptBody {
            chunks: chunks.into(),
            stall,
            polls: Arc::clone(&polls),
        }),
        polls,
    )
}
fn http(body: Body) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/v1/capture")
        .body(body)
        .unwrap()
}
fn grpc(body: Body) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/deleuze.v1.Capture/Append")
        .version(Version::HTTP_2)
        .header("content-type", "application/grpc+deleuze")
        .body(body)
        .unwrap()
}
fn message(flag: u8, payload: &[u8]) -> Vec<u8> {
    let mut wire = vec![flag];
    wire.extend((payload.len() as u32).to_be_bytes());
    wire.extend(payload);
    wire
}
async fn unpack(response: Response<Body>) -> (HeaderMap, Vec<u8>, HeaderMap) {
    let (parts, mut body) = response.into_parts();
    let mut bytes = Vec::new();
    let mut trailers = HeaderMap::new();
    while let Some(frame) = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await {
        let frame = frame.unwrap();
        match frame.into_data() {
            Ok(data) => bytes.extend(data),
            Err(frame) => trailers = frame.into_trailers().unwrap(),
        }
    }
    (parts.headers, bytes, trailers)
}
fn payload(entry: &Entry) -> &[u8] {
    match &entry.properties[0].1 {
        Value::Bytes(bytes) | Value::Encoded { bytes, .. } => bytes,
        other => panic!("unexpected content {other:?}"),
    }
}
fn incomplete(entry: &Entry) -> bool {
    entry
        .properties
        .iter()
        .any(|(k, v)| k == "capture_incomplete" && v == &Value::Bool(true))
}
fn population(gate: &Gate) {
    let s = gate.stats();
    assert_eq!(
        s.arrived,
        s.active as u64 + s.retained + s.failed + s.unknown
    );
}

#[tokio::test]
async fn unpolled_dropped_response_keeps_owned_body_and_capture_work() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let future = gate.call(http(Body::from(vec![0, 255, 123])));
    assert_eq!((gate.stats().arrived, gate.stats().active), (1, 1));
    drop(future);
    gate.close();
    gate.drain().await;
    assert_eq!(payload(&fixture.entries()[0]), &[0, 255, 123]);
    assert_eq!(
        (
            gate.stats().retained,
            gate.stats().active,
            gate.stats().retained_records
        ),
        (1, 0, 1)
    );
    population(&gate);
}

#[tokio::test]
async fn http_exact_content_metadata_and_complete_empty() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let bytes = b"{\"seq\":999,\"created_at\":-1,\"broken\":\xff}".to_vec();
    let response = gate.call(http(Body::from(bytes.clone()))).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let record = &fixture.entries()[0].record;
    assert_eq!(response.headers()["deleuze-capture-outcome"], "retained");
    assert_eq!(response.headers()["deleuze-seq"], record.seq().to_string());
    assert_eq!(
        response.headers()["deleuze-created-at-seconds"],
        record.created_at().unix_seconds().to_string()
    );
    assert_eq!(
        response.headers()["deleuze-property-hash"],
        record
            .property_hash()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect::<String>()
    );
    assert_eq!(payload(&fixture.entries()[0]), bytes);
    assert_eq!(
        gate.call(http(Body::empty())).await.unwrap().status(),
        StatusCode::CREATED
    );
    assert!(payload(&fixture.entries()[1]).is_empty());
    assert_eq!(gate.stats().retained, 2);
    population(&gate);
}

#[tokio::test]
async fn http_protocol_rejection_still_retains_content_once() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    for (path, method, coding, expected) in [
        ("/elsewhere", Method::POST, None, StatusCode::NOT_FOUND),
        (
            "/v1/capture",
            Method::GET,
            None,
            StatusCode::METHOD_NOT_ALLOWED,
        ),
        (
            "/v1/capture",
            Method::POST,
            Some("gzip"),
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
        ),
    ] {
        let mut request = Request::builder().uri(path).method(method);
        if let Some(coding) = coding {
            request = request.header("content-encoding", coding);
        }
        let response = gate
            .call(request.body(Body::from("unchanged")).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), expected);
        assert_eq!(response.headers()["deleuze-capture-outcome"], "retained");
    }
    assert_eq!(fixture.entries().len(), 3);
    assert!(fixture.entries().iter().all(|e| payload(e) == b"unchanged"));
    assert!(matches!(
        &fixture.entries()[2].properties[0].1,
        Value::Encoded { .. }
    ));
    assert_eq!((gate.stats().arrived, gate.stats().retained), (3, 3));
    population(&gate);
}

#[tokio::test]
async fn limit_read_error_and_timeout_retain_only_the_observed_prefix() {
    for failure in 0..3 {
        let fixture = Fixture::new(Options::default());
        let mut gate = fixture.gate(GateOptions {
            max_body_bytes: 3,
            body_timeout: Duration::from_millis(15),
            ..GateOptions::default()
        });
        let (body, _) = match failure {
            0 => script(vec![Ok(Bytes::from_static(b"abcdef"))], false),
            1 => script(
                vec![
                    Ok(Bytes::from_static(b"abc")),
                    Err(io::Error::other("reset")),
                ],
                false,
            ),
            _ => script(vec![Ok(Bytes::from_static(b"abc"))], true),
        };
        let response = gate.call(http(body)).await.unwrap();
        assert_eq!(
            response.status(),
            match failure {
                0 => StatusCode::PAYLOAD_TOO_LARGE,
                1 => StatusCode::SERVICE_UNAVAILABLE,
                _ => StatusCode::REQUEST_TIMEOUT,
            }
        );
        assert_eq!(response.headers()["deleuze-capture-outcome"], "failed");
        let entries = fixture.entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(payload(&entries[0]), b"abc");
        assert!(incomplete(&entries[0]));
        assert_eq!((gate.stats().failed, gate.stats().retained_records), (1, 1));
        population(&gate);
    }
}

#[tokio::test]
async fn unavailable_input_does_not_fabricate_empty_record() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let (body, _) = script(vec![Err(io::Error::other("no content"))], false);
    assert_eq!(
        gate.call(http(body)).await.unwrap().status(),
        StatusCode::SERVICE_UNAVAILABLE
    );
    for wire in [vec![], vec![0, 0, 0, 0], vec![0, 0, 0, 0, 5]] {
        let response = gate.call(grpc(Body::from(wire))).await.unwrap();
        let (headers, _, trailers) = unpack(response).await;
        assert_eq!(headers["deleuze-capture-outcome"], "failed");
        assert_ne!(trailers["grpc-status"], "0");
    }
    assert!(fixture.entries().is_empty());
    assert_eq!((gate.stats().failed, gate.stats().retained_records), (4, 0));
    population(&gate);
}

#[tokio::test]
async fn every_grpc_prefix_split_preserves_only_payload_and_standard_success_trailers() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let wire = message(0, &[0, 255, 2, 3]);
    for split in 0..=wire.len() {
        let (body, _) = script(
            vec![
                Ok(Bytes::copy_from_slice(&wire[..split])),
                Ok(Bytes::copy_from_slice(&wire[split..])),
            ],
            false,
        );
        let response = gate.call(grpc(body)).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        let (headers, bytes, trailers) = unpack(response).await;
        assert_eq!(headers["deleuze-capture-outcome"], "retained");
        assert_eq!(bytes, vec![0; 5]);
        assert_eq!(trailers["grpc-status"], "0");
    }
    let entries = fixture.entries();
    assert_eq!(entries.len(), wire.len() + 1);
    assert!(entries.iter().all(|e| payload(e) == [0, 255, 2, 3]));
    assert!(entries.iter().all(|e| !incomplete(e)));
    population(&gate);
}

#[tokio::test]
async fn grpc_zero_length_is_content_and_extra_messages_are_protocol_error_not_capture_loss() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let (_, _, trailers) =
        unpack(gate.call(grpc(Body::from(message(0, &[])))).await.unwrap()).await;
    assert_eq!(trailers["grpc-status"], "0");
    assert!(payload(&fixture.entries()[0]).is_empty());
    let mut wire = message(0, b"first");
    wire.extend(message(0, b"second"));
    let (headers, bytes, trailers) = unpack(gate.call(grpc(Body::from(wire))).await.unwrap()).await;
    assert_eq!(headers["deleuze-capture-outcome"], "retained");
    assert_eq!(trailers["grpc-status"], "3");
    assert!(bytes.is_empty());
    let entries = fixture.entries();
    assert_eq!(entries.len(), 3);
    assert_eq!(payload(&entries[1]), b"first");
    assert_eq!(payload(&entries[2]), b"second");
    assert_eq!(
        (
            gate.stats().arrived,
            gate.stats().retained,
            gate.stats().retained_records
        ),
        (2, 2, 3)
    );
    population(&gate);
}

#[tokio::test]
async fn grpc_invalid_flags_compression_and_partial_payloads_remain_content() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    for flag in [1, 2, 255] {
        let mut request = grpc(Body::from(message(flag, b"opaque")));
        if flag == 1 {
            request
                .headers_mut()
                .insert("grpc-encoding", "gzip".parse().unwrap());
        }
        let (headers, _, trailers) = unpack(gate.call(request).await.unwrap()).await;
        assert_eq!(headers["deleuze-capture-outcome"], "retained");
        assert_ne!(trailers["grpc-status"], "0");
    }
    let mut partial = vec![0];
    partial.extend(9_u32.to_be_bytes());
    partial.extend(b"prefix");
    let (headers, _, trailers) = unpack(gate.call(grpc(Body::from(partial))).await.unwrap()).await;
    assert_eq!(headers["deleuze-capture-outcome"], "failed");
    assert_eq!(trailers["grpc-status"], "3");
    let entries = fixture.entries();
    assert_eq!(entries.len(), 4);
    for entry in &entries[..3] {
        assert_eq!(payload(entry), b"opaque");
        assert!(matches!(&entry.properties[0].1, Value::Encoded { .. }));
    }
    assert_eq!(payload(&entries[3]), b"prefix");
    assert!(incomplete(&entries[3]));
    assert_eq!(
        (
            gate.stats().retained,
            gate.stats().failed,
            gate.stats().retained_records
        ),
        (3, 1, 4)
    );
    population(&gate);
}

#[tokio::test]
async fn grpc_budget_counts_wire_bytes_and_deadline_tightens_acquisition_only() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions {
        max_body_bytes: 7,
        ..GateOptions::default()
    });
    let (headers, _, trailers) = unpack(
        gate.call(grpc(Body::from(message(0, b"abcdef"))))
            .await
            .unwrap(),
    )
    .await;
    assert_eq!(headers["deleuze-capture-outcome"], "failed");
    assert_eq!(trailers["grpc-status"], "8");
    assert_eq!(payload(&fixture.entries()[0]), b"ab");
    assert!(incomplete(&fixture.entries()[0]));
    let (body, _) = script(vec![], true);
    let mut request = grpc(body);
    request
        .headers_mut()
        .insert("grpc-timeout", "1m".parse().unwrap());
    let (headers, _, trailers) = unpack(gate.call(request).await.unwrap()).await;
    assert_eq!(headers["deleuze-capture-outcome"], "failed");
    assert_eq!(trailers["grpc-status"], "4");
    assert_eq!(fixture.entries().len(), 1);
    population(&gate);
}

#[tokio::test]
async fn overload_closed_and_multiple_drainers_keep_every_arrival_visible() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions {
        max_in_flight: 1,
        body_timeout: Duration::from_millis(15),
        ..GateOptions::default()
    });
    let (body, _) = script(vec![], true);
    let first = gate.call(http(body));
    let (body, polls) = script(vec![Ok(Bytes::from_static(b"not acquired"))], false);
    let response = gate.call(http(body)).await.unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(polls.load(Ordering::Relaxed), 0);
    gate.close();
    let response = gate
        .call(grpc(Body::from(message(0, b"closed"))))
        .await
        .unwrap();
    let (_, _, trailers) = unpack(response).await;
    assert_eq!(trailers["grpc-status"], "14");
    drop(first);
    tokio::join!(gate.drain(), gate.drain());
    assert_eq!(
        (
            gate.stats().arrived,
            gate.stats().active,
            gate.stats().failed
        ),
        (3, 0, 3)
    );
    assert!(fixture.entries().is_empty());
    population(&gate);
}

#[tokio::test]
async fn core_capacity_failure_remains_failed_capture() {
    let fixture = Fixture::new(Options {
        max_content_bytes: 8,
        ..Options::default()
    });
    let mut gate = fixture.gate(GateOptions::default());
    let response = gate.call(http(Body::from("cannot fit"))).await.unwrap();
    assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(response.headers()["deleuze-capture-outcome"], "failed");
    assert!(!response.headers().contains_key("deleuze-seq"));
    assert_eq!(gate.stats().failed, 1);
    assert!(fixture.entries().is_empty());
    population(&gate);
}

#[test]
fn gate_options_reject_zero_and_clock_overflow() {
    let fixture = Fixture::new(Options::default());
    for options in [
        GateOptions {
            max_in_flight: 0,
            ..GateOptions::default()
        },
        GateOptions {
            max_body_bytes: 0,
            ..GateOptions::default()
        },
        GateOptions {
            body_timeout: Duration::ZERO,
            ..GateOptions::default()
        },
        GateOptions {
            body_timeout: Duration::MAX,
            ..GateOptions::default()
        },
    ] {
        assert!(Gate::new(Arc::clone(&fixture.log), options).is_err());
    }
}

#[tokio::test]
async fn regression_explicit_http_route_ignores_grpc_media_hint() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let mut request = http(Body::from("hello"));
    request
        .headers_mut()
        .insert("content-type", "application/grpc".parse().unwrap());
    let response = gate.call(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    assert_eq!(response.headers()["deleuze-capture-outcome"], "retained");
    assert_eq!(payload(&fixture.entries()[0]), b"hello");
    assert_eq!(
        (
            gate.stats().arrived,
            gate.stats().retained,
            gate.stats().retained_records
        ),
        (1, 1, 1)
    );
}

#[tokio::test]
async fn regression_grpc_media_type_is_case_insensitive() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    for media in ["Application/GRPC", "Application/GRPC+Deleuze"] {
        let mut request = grpc(Body::from(message(0, b"hello")));
        request
            .headers_mut()
            .insert("content-type", media.parse().unwrap());
        let (_, _, trailers) = unpack(gate.call(request).await.unwrap()).await;
        assert_eq!(trailers["grpc-status"], "0");
    }
    assert_eq!(gate.stats().retained_records, 2);
}

#[tokio::test]
async fn regression_http_coding_hint_identifies_the_retained_representation() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    for coding in ["gzip", "deflate"] {
        let mut request = http(Body::from("encoded"));
        request
            .headers_mut()
            .insert("content-encoding", coding.parse().unwrap());
        assert_eq!(
            gate.call(request).await.unwrap().status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
    }
    let entries = fixture.entries();
    for (entry, coding) in entries.iter().zip(["gzip", "deflate"]) {
        assert!(
            matches!(&entry.properties[0].1, Value::Encoded { representation, bytes } if representation == &format!("http-content-coding:{coding}") && bytes == b"encoded")
        );
    }
}

#[tokio::test]
async fn regression_grpc_coding_hint_identifies_the_retained_representation() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    for coding in ["gzip", "deflate"] {
        let mut request = grpc(Body::from(message(1, b"encoded")));
        request
            .headers_mut()
            .insert("grpc-encoding", coding.parse().unwrap());
        let (headers, _, trailers) = unpack(gate.call(request).await.unwrap()).await;
        assert_eq!(headers["deleuze-capture-outcome"], "retained");
        assert_eq!(trailers["grpc-status"], "12");
    }
    let entries = fixture.entries();
    for (entry, coding) in entries.iter().zip(["gzip", "deflate"]) {
        assert!(
            matches!(&entry.properties[0].1, Value::Encoded { representation, bytes } if representation == &format!("grpc-content-coding:{coding}") && bytes == b"encoded")
        );
    }
}

#[tokio::test]
async fn regression_outer_http_coding_on_grpc_retains_entity_without_framing_it() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let bytes = b"\x1f\x8b\x08\x00\x00compressed-entity";
    let mut request = grpc(Body::from(bytes.to_vec()));
    request
        .headers_mut()
        .insert("content-encoding", "gzip".parse().unwrap());
    let (headers, _, trailers) = unpack(gate.call(request).await.unwrap()).await;
    assert_eq!(headers["deleuze-capture-outcome"], "retained");
    assert_ne!(trailers["grpc-status"], "0");
    let entries = fixture.entries();
    assert_eq!(entries.len(), 1);
    assert!(
        matches!(&entries[0].properties[0].1, Value::Encoded { representation, bytes: stored } if representation == "http-content-coding:gzip" && stored == bytes)
    );
    assert!(!incomplete(&entries[0]));
}

#[tokio::test]
async fn regression_uncompressed_grpc_message_is_not_mislabeled_by_encoding_header() {
    let fixture = Fixture::new(Options::default());
    let mut gate = fixture.gate(GateOptions::default());
    let mut request = grpc(Body::from(message(0, b"raw")));
    request
        .headers_mut()
        .insert("grpc-encoding", "gzip".parse().unwrap());
    let (headers, _, _) = unpack(gate.call(request).await.unwrap()).await;
    assert_eq!(headers["deleuze-capture-outcome"], "retained");
    assert!(
        matches!(&fixture.entries()[0].properties[0].1, Value::Bytes(bytes) if bytes == b"raw")
    );
}
