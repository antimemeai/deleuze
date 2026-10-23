//! Optional HTTP and raw-content unary gRPC gates through the native append path.
//!
//! [`tower_service::Service::call`] observes arrivals synchronously and transfers
//! the body to an owned task. Dropping its response future does not cancel capture.
//! Hosts must close admission and [`Gate::drain`] before dropping their runtime.
//! Transport syntax rejected before `call` is outside these request counters.

mod protocol;

use crate::{Log, Record};
use axum::body::{Body, Bytes};
use axum::http::{HeaderMap, HeaderValue, Request, Response, StatusCode};
use http_body::Body as HttpBody;
use std::convert::Infallible;
use std::fmt;
use std::future::{Future, Ready, ready};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::{Notify, oneshot};
use tokio::time::Instant;
use tower_service::Service;

/// Per-request acquisition and concurrent-operation limits.
#[derive(Debug, Clone, Copy)]
pub struct GateOptions {
    pub max_in_flight: usize,
    /// Maximum acquired HTTP body or gRPC wire bytes, including gRPC prefixes.
    pub max_body_bytes: usize,
    /// Acquisition budget only; storage completion is not cancelled by this timer.
    pub body_timeout: Duration,
}
impl Default for GateOptions {
    fn default() -> Self {
        Self {
            max_in_flight: 64,
            max_body_bytes: 1024 * 1024,
            body_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug)]
pub struct GateError(&'static str);
impl fmt::Display for GateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}
impl std::error::Error for GateError {}

/// Request outcomes and record count are separate populations. These are not
/// persisted request events. `arrived = active + retained + failed + unknown`.
#[derive(Debug, Clone, Copy, Default)]
pub struct GateStats {
    pub arrived: u64,
    pub active: usize,
    pub retained: u64,
    pub failed: u64,
    pub unknown: u64,
    pub retained_records: u64,
    pub closed: bool,
}
struct Shared {
    stats: Mutex<GateStats>,
    changed: Notify,
}

/// Cloneable dedicated ingestion service. No routing/extractor precedes `call`.
#[derive(Clone)]
pub struct Gate {
    log: Arc<Log>,
    options: GateOptions,
    shared: Arc<Shared>,
}
impl Gate {
    pub fn new(log: Arc<Log>, options: GateOptions) -> Result<Self, GateError> {
        if options.max_in_flight == 0
            || options.max_body_bytes == 0
            || options.max_body_bytes > isize::MAX as usize
            || options.body_timeout.is_zero()
            || Instant::now().checked_add(options.body_timeout).is_none()
        {
            return Err(GateError(
                "gate limits must be nonzero and fit allocation/clock bounds",
            ));
        }
        Ok(Self {
            log,
            options,
            shared: Arc::new(Shared {
                stats: Mutex::new(GateStats::default()),
                changed: Notify::new(),
            }),
        })
    }

    pub fn stats(&self) -> GateStats {
        *self.shared.stats.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Stop admission. Calls racing after this still count as failed arrivals.
    pub fn close(&self) {
        self.shared
            .stats
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .closed = true;
    }

    /// Wait for owned operations, including storage, to finish. Close first when
    /// using a custom host so new requests cannot continually extend the wait.
    pub async fn drain(&self) {
        loop {
            let notified = self.shared.changed.notified();
            tokio::pin!(notified);
            notified.as_mut().enable();
            if self.stats().active == 0 {
                return;
            }
            notified.await;
        }
    }

    /// Serve HTTP/1+HTTP/2, close admission at shutdown, then drain owned capture.
    /// Call with a live Tokio runtime and an explicit shutdown future.
    pub async fn serve(
        &self,
        listener: tokio::net::TcpListener,
        shutdown: impl Future<Output = ()> + Send + 'static,
    ) -> std::io::Result<()> {
        let closing = self.clone();
        let result = axum::serve(listener, MakeGate(self.clone()))
            .with_graceful_shutdown(async move {
                shutdown.await;
                closing.close();
            })
            .await;
        self.close();
        self.drain().await;
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Retained,
    Failed,
    Unknown,
}
impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Self::Retained => "retained",
            Self::Failed => "failed",
            Self::Unknown => "unknown",
        }
    }
    pub(super) fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Unknown, _) | (_, Self::Unknown) => Self::Unknown,
            (Self::Failed, _) | (_, Self::Failed) => Self::Failed,
            _ => Self::Retained,
        }
    }
}
struct Active {
    shared: Arc<Shared>,
    finished: bool,
}
impl Active {
    fn finish(&mut self, outcome: Outcome, records: u64) {
        let mut stats = self.shared.stats.lock().unwrap_or_else(|e| e.into_inner());
        stats.active -= 1;
        match outcome {
            Outcome::Retained => stats.retained += 1,
            Outcome::Failed => stats.failed += 1,
            Outcome::Unknown => stats.unknown += 1,
        }
        stats.retained_records += records;
        self.finished = true;
        drop(stats);
        self.shared.changed.notify_waiters();
    }
}
impl Drop for Active {
    fn drop(&mut self) {
        if !self.finished {
            self.finish(Outcome::Unknown, 0);
        }
    }
}

impl Service<Request<Body>> for Gate {
    type Response = Response<Body>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Response<Body>, Infallible>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let profile = protocol::Profile::inspect(&request, self.options.body_timeout);
        let runtime = tokio::runtime::Handle::try_current();
        {
            let mut stats = self.shared.stats.lock().unwrap_or_else(|e| e.into_inner());
            stats.arrived += 1;
            if stats.closed || stats.active >= self.options.max_in_flight || runtime.is_err() {
                stats.failed += 1;
                let rejected = protocol::Rejected {
                    capacity: !stats.closed && runtime.is_ok(),
                };
                let response = profile.rejected(rejected);
                return Box::pin(async move { Ok(response) });
            }
            stats.active += 1;
        }
        let mut active = Active {
            shared: Arc::clone(&self.shared),
            finished: false,
        };
        let log = Arc::clone(&self.log);
        let max_bytes = self.options.max_body_bytes;
        let deadline = Instant::now() + profile.budget;
        let (sender, receiver) = oneshot::channel();
        runtime
            .expect("runtime checked before admission")
            .spawn(async move {
                let acquired = acquire(request.into_body(), max_bytes, deadline).await;
                // One blocking task owns parsing + all appends for this request. The
                // response observer cannot cancel it, and no task is spawned per message.
                let capture_profile = profile.clone();
                let result =
                    tokio::task::spawn_blocking(move || capture_profile.capture(&log, acquired))
                        .await;
                let captured = match result {
                    Ok(result) => result,
                    Err(_) => protocol::Captured::unknown(),
                };
                active.finish(captured.outcome, captured.records);
                let _ = sender.send(profile.response(captured));
            });
        Box::pin(async move {
            Ok(receiver.await.unwrap_or_else(|_| {
                let mut response = Response::new(Body::empty());
                *response.status_mut() = StatusCode::SERVICE_UNAVAILABLE;
                response.headers_mut().insert(
                    "deleuze-capture-outcome",
                    HeaderValue::from_static("unknown"),
                );
                response
            }))
        })
    }
}

struct MakeGate(Gate);
impl<'a> Service<axum::serve::IncomingStream<'a, tokio::net::TcpListener>> for MakeGate {
    type Response = Gate;
    type Error = Infallible;
    type Future = Ready<Result<Gate, Infallible>>;
    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Infallible>> {
        Poll::Ready(Ok(()))
    }
    fn call(
        &mut self,
        _: axum::serve::IncomingStream<'a, tokio::net::TcpListener>,
    ) -> Self::Future {
        ready(Ok(self.0.clone()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum AcquireFailure {
    Limit,
    Timeout,
    Read,
    Allocation,
}
pub(super) struct Acquired {
    bytes: Vec<u8>,
    failure: Option<AcquireFailure>,
}
async fn acquire(mut body: Body, max: usize, deadline: Instant) -> Acquired {
    let mut bytes = Vec::new();
    let failure = loop {
        if Instant::now() >= deadline {
            break Some(AcquireFailure::Timeout);
        }
        let frame = tokio::time::timeout_at(
            deadline,
            std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)),
        )
        .await;
        match frame {
            Err(_) => break Some(AcquireFailure::Timeout),
            Ok(Some(Err(_))) => break Some(AcquireFailure::Read),
            Ok(None) => break None,
            Ok(Some(Ok(frame))) => {
                if let Ok(data) = frame.into_data() {
                    let take = data.len().min(max - bytes.len());
                    if bytes.try_reserve(take).is_err() {
                        break Some(AcquireFailure::Allocation);
                    }
                    bytes.extend_from_slice(&data[..take]);
                    if take < data.len() {
                        break Some(AcquireFailure::Limit);
                    }
                }
            }
        }
    };
    Acquired { bytes, failure }
}

fn metadata(headers: &mut HeaderMap, outcome: Outcome, record: Option<&Record>) {
    headers.insert(
        "deleuze-capture-outcome",
        HeaderValue::from_static(outcome.name()),
    );
    if let Some(record) = record {
        for (name, value) in [
            ("deleuze-seq", record.seq().to_string()),
            (
                "deleuze-property-hash",
                record
                    .property_hash()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect(),
            ),
            (
                "deleuze-created-at-seconds",
                record.created_at().unix_seconds().to_string(),
            ),
            (
                "deleuze-created-at-nanos",
                record.created_at().nanos().to_string(),
            ),
        ] {
            headers.insert(
                name,
                HeaderValue::from_str(&value).expect("numeric/hex metadata"),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Outcome;
    #[test]
    fn unknown_dominates_failed_and_retained_independently_of_order() {
        for first in [Outcome::Retained, Outcome::Failed, Outcome::Unknown] {
            assert_eq!(first.combine(Outcome::Unknown), Outcome::Unknown);
            assert_eq!(Outcome::Unknown.combine(first), Outcome::Unknown);
        }
        assert_eq!(Outcome::Retained.combine(Outcome::Failed), Outcome::Failed);
        assert_eq!(Outcome::Failed.combine(Outcome::Retained), Outcome::Failed);
    }
}
