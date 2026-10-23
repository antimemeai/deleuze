use super::{AcquireFailure, Acquired, Body, Bytes, HeaderMap, HeaderValue, Outcome, Response, StatusCode, metadata};
use crate::{AppendOutcome, Content, Log, Record, Value};
use axum::http::{Method, Request, Version};
use http_body::{Body as HttpBody, Frame};
use std::convert::Infallible;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

const GRPC_PATH: &str = "/deleuze.v1.Capture/Append";

#[derive(Clone)]
pub(super) struct Profile {
    grpc: bool,
    http_rejection: Option<StatusCode>,
    grpc_rejection: Option<u8>,
    opaque: Option<&'static str>,
    pub(super) budget: Duration,
}
pub(super) struct Rejected { pub(super) capacity: bool }
pub(super) struct Captured {
    pub(super) outcome: Outcome,
    pub(super) records: u64,
    last: Option<Record>,
    acquisition: Option<AcquireFailure>,
    grpc_error: Option<u8>,
    storage_failed: bool,
}
impl Captured {
    pub(super) fn unknown() -> Self {
        Self { outcome: Outcome::Unknown, records: 0, last: None, acquisition: None, grpc_error: None, storage_failed: true }
    }
    fn append(&mut self, log: &Log, content: Content) {
        match log.append(content) {
            Ok(record) => { self.records += 1; self.last = Some(record); }
            Err(error) => {
                self.outcome = self.outcome.combine(match error.outcome { AppendOutcome::Unknown => Outcome::Unknown, AppendOutcome::NotWritten => Outcome::Failed });
                self.storage_failed = true;
            }
        }
    }
}
impl Profile {
    pub(super) fn inspect(request: &Request<Body>, budget: Duration) -> Self {
        let media = request.headers().get("content-type").and_then(|h| h.to_str().ok()).unwrap_or("").split(';').next().unwrap_or("").trim();
        let grpc = request.uri().path() == GRPC_PATH || media.starts_with("application/grpc");
        let mut profile = Self { grpc, http_rejection: None, grpc_rejection: None, opaque: None, budget };
        if grpc {
            if request.uri().path() != GRPC_PATH || request.method() != Method::POST { profile.grpc_rejection = Some(12); }
            if request.version() != Version::HTTP_2 { profile.grpc_rejection.get_or_insert(3); }
            if media != "application/grpc" && media != "application/grpc+deleuze" { profile.grpc_rejection.get_or_insert(3); }
            if !identity(request.headers(), "grpc-encoding") {
                profile.grpc_rejection.get_or_insert(12);
                profile.opaque = Some("grpc-unsupported-content-coding");
            }
            if let Some(timeout) = request.headers().get("grpc-timeout") {
                match timeout.to_str().ok().and_then(grpc_timeout) {
                    Some(duration) => profile.budget = profile.budget.min(duration),
                    None => { profile.grpc_rejection.get_or_insert(3); }
                }
            }
        } else {
            if request.uri().path() != "/v1/capture" { profile.http_rejection = Some(StatusCode::NOT_FOUND); }
            else if request.method() != Method::POST { profile.http_rejection = Some(StatusCode::METHOD_NOT_ALLOWED); }
            if !identity(request.headers(), "content-encoding") {
                profile.http_rejection.get_or_insert(StatusCode::UNSUPPORTED_MEDIA_TYPE);
                profile.opaque = Some("http-unsupported-content-coding");
            }
        }
        profile
    }

    pub(super) fn rejected(&self, rejected: Rejected) -> Response<Body> {
        if self.grpc { self.grpc_response(Outcome::Failed, None, if rejected.capacity { 8 } else { 14 }) }
        else { self.http_response(Outcome::Failed, None, StatusCode::SERVICE_UNAVAILABLE) }
    }

    pub(super) fn capture(&self, log: &Log, acquired: Acquired) -> Captured {
        let mut result = Captured { outcome: if acquired.failure.is_some() { Outcome::Failed } else { Outcome::Retained }, records: 0, last: None, acquisition: acquired.failure, grpc_error: self.grpc_rejection, storage_failed: false };
        if !self.grpc {
            if acquired.failure.is_none() || !acquired.bytes.is_empty() {
                result.append(log, content(acquired.bytes, acquired.failure.is_some(), self.opaque));
            }
            return result;
        }
        let mut remaining = acquired.bytes.as_slice();
        let mut messages = 0_u64;
        while !remaining.is_empty() {
            if remaining.len() < 5 {
                result.outcome = result.outcome.combine(Outcome::Failed);
                result.grpc_error.get_or_insert(3);
                break;
            }
            let flag = remaining[0];
            let length = u32::from_be_bytes(remaining[1..5].try_into().expect("four-byte prefix length")) as usize;
            remaining = &remaining[5..];
            let partial = remaining.len() < length;
            let payload = &remaining[..remaining.len().min(length)];
            let mut opaque = self.opaque;
            if flag != 0 {
                result.grpc_error.get_or_insert(if flag == 1 { 12 } else { 3 });
                opaque = Some(if flag == 1 { "grpc-compressed-unsupported" } else { "grpc-invalid-compression-flag" });
            }
            if !partial || !payload.is_empty() {
                let mut bytes = Vec::new();
                if bytes.try_reserve(payload.len()).is_err() {
                    result.outcome = result.outcome.combine(Outcome::Failed);
                    result.storage_failed = true;
                    break;
                }
                bytes.extend_from_slice(payload);
                result.append(log, content(bytes, partial, opaque));
            }
            if partial {
                result.outcome = result.outcome.combine(Outcome::Failed);
                result.grpc_error.get_or_insert(3);
                break;
            }
            messages += 1;
            remaining = &remaining[length..];
        }
        if messages == 0 {
            result.outcome = result.outcome.combine(Outcome::Failed);
            result.grpc_error.get_or_insert(3);
        } else if messages > 1 { result.grpc_error.get_or_insert(3); }
        result
    }

    pub(super) fn response(&self, captured: Captured) -> Response<Body> {
        if self.grpc {
            let status = if captured.outcome == Outcome::Unknown || captured.storage_failed { 14 }
                else if let Some(failure) = captured.acquisition { grpc_failure(failure) }
                else { captured.grpc_error.unwrap_or(0) };
            self.grpc_response(captured.outcome, captured.last.as_ref(), status)
        } else {
            let status = if captured.outcome == Outcome::Unknown || captured.storage_failed { StatusCode::SERVICE_UNAVAILABLE }
                else if let Some(failure) = captured.acquisition { http_failure(failure) }
                else { self.http_rejection.unwrap_or(StatusCode::CREATED) };
            self.http_response(captured.outcome, captured.last.as_ref(), status)
        }
    }

    fn http_response(&self, outcome: Outcome, record: Option<&Record>, status: StatusCode) -> Response<Body> {
        let mut response = Response::new(Body::empty());
        *response.status_mut() = status;
        metadata(response.headers_mut(), outcome, record);
        response
    }
    fn grpc_response(&self, outcome: Outcome, record: Option<&Record>, status: u8) -> Response<Body> {
        let mut trailers = HeaderMap::new();
        trailers.insert("grpc-status", HeaderValue::from_str(&status.to_string()).expect("numeric status"));
        let mut response = Response::new(Body::new(GrpcResponse { data: (status == 0).then(|| Bytes::from_static(&[0; 5])), trailers: Some(trailers) }));
        response.headers_mut().insert("content-type", HeaderValue::from_static("application/grpc+deleuze"));
        metadata(response.headers_mut(), outcome, record);
        response
    }
}

fn content(bytes: Vec<u8>, incomplete: bool, opaque: Option<&str>) -> Content {
    if incomplete {
        let value = if let Some(representation) = opaque { Value::Encoded { representation: representation.into(), bytes } } else { Value::Bytes(bytes) };
        Content::Properties(vec![("content".into(), value), ("capture_incomplete".into(), Value::Bool(true))])
    } else if let Some(representation) = opaque { Content::Encoded { representation: representation.into(), bytes } }
    else { Content::Bytes(bytes) }
}
fn identity(headers: &HeaderMap, name: &'static str) -> bool {
    headers.get(name).is_none_or(|h| h.to_str().is_ok_and(|v| v.eq_ignore_ascii_case("identity")))
}
fn http_failure(failure: AcquireFailure) -> StatusCode {
    match failure { AcquireFailure::Limit => StatusCode::PAYLOAD_TOO_LARGE,
        AcquireFailure::Timeout => StatusCode::REQUEST_TIMEOUT, _ => StatusCode::SERVICE_UNAVAILABLE }
}
fn grpc_failure(failure: AcquireFailure) -> u8 {
    match failure { AcquireFailure::Limit | AcquireFailure::Allocation => 8, AcquireFailure::Timeout => 4, AcquireFailure::Read => 14 }
}
fn grpc_timeout(value: &str) -> Option<Duration> {
    let (digits, unit) = value.split_at_checked(value.len().checked_sub(1)?)?;
    if digits.is_empty() || digits.len() > 8 || !digits.bytes().all(|v| v.is_ascii_digit()) { return None; }
    let number = digits.parse::<u64>().ok()?;
    if number == 0 { return None; }
    match unit { "H" => Some(Duration::from_secs(number.checked_mul(3600)?)), "M" => Some(Duration::from_secs(number.checked_mul(60)?)),
        "S" => Some(Duration::from_secs(number)), "m" => Some(Duration::from_millis(number)),
        "u" => Some(Duration::from_micros(number)), "n" => Some(Duration::from_nanos(number)), _ => None }
}

struct GrpcResponse { data: Option<Bytes>, trailers: Option<HeaderMap> }
impl HttpBody for GrpcResponse {
    type Data = Bytes;
    type Error = Infallible;
    fn poll_frame(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
        Poll::Ready(self.data.take().map(Frame::data).or_else(|| self.trailers.take().map(Frame::trailers)).map(Ok))
    }
    fn is_end_stream(&self) -> bool { self.data.is_none() && self.trailers.is_none() }
}
