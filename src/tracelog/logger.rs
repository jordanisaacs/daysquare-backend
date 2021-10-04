use axum::http::{Request, Response};
use std::time::Duration;
use tower_http::trace::{MakeSpan, OnBodyChunk, OnEos, OnFailure, OnRequest, OnResponse};
use tracing::Level;
use tracing::Span;

#[derive(Debug, Clone)]
pub struct TracingLogger {
    pub(crate) req_level: Option<Level>,
    pub(crate) _chunk_level: Option<Level>,
    pub(crate) _eos_level: Option<Level>,
    pub(crate) resp_level: Option<Level>,
    pub(crate) fail_level: Option<Level>,
}

impl<B> MakeSpan<B> for TracingLogger {
    fn make_span(&mut self, req: &Request<B>) -> Span {
        let root_span = crate::root_span!(req,);
        root_span
    }
}

macro_rules! log_pattern_req {
    (
        $log:expr, $message:expr, [$($level:ident),*]
    ) => {
        match ($log) {
            $(
                Level::$level => {
                    tracing::event!(
                        Level::$level,
                        $message
                    );
                }
            )*
        }
    }
}

impl<B> OnRequest<B> for TracingLogger {
    fn on_request(&mut self, _req: &Request<B>, _span: &Span) {
        if let Some(level) = self.req_level {
            log_pattern_req!(
                level,
                "finished processing request",
                [ERROR, WARN, INFO, DEBUG, TRACE]
            );
        };
    }
}

impl OnEos for TracingLogger {
    fn on_eos(
        self,
        _trailers: Option<&hyper::HeaderMap>,
        _stream_duration: Duration,
        _span: &Span,
    ) {
    }
}

impl<B> OnBodyChunk<B> for TracingLogger {
    fn on_body_chunk(&mut self, _chunk: &B, _latency: Duration, _span: &Span) {}
}

macro_rules! log_pattern_resp {
    (
        $log:expr, $latency:expr, $message:expr, [$($level:ident),*]
    ) => {
        match ($log) {
            $(
                Level::$level => {
                    tracing::event!(
                        Level::$level,
                        latency = format_args!("{} ms", $latency.as_millis()),
                        $message
                    );
                }
            )*
        }
    }
}

impl<B> OnResponse<B> for TracingLogger {
    fn on_response(self, resp: &Response<B>, latency: Duration, span: &Span) {
        span.record("http.status_code", &tracing::field::display(resp.status()));

        if let Some(level) = self.resp_level {
            log_pattern_resp!(
                level,
                latency,
                "finished processing request",
                [ERROR, WARN, INFO, DEBUG, TRACE]
            );
        }
    }
}

impl<FailureClass> OnFailure<FailureClass> for TracingLogger
where
    FailureClass: std::fmt::Display,
{
    fn on_failure(&mut self, failure_classification: FailureClass, latency: Duration, span: &Span) {
        span.record(
            "failure_class",
            &tracing::field::display(&failure_classification),
        );

        if let Some(level) = self.fail_level {
            log_pattern_resp!(
                level,
                latency,
                "response failed",
                [ERROR, WARN, INFO, DEBUG, TRACE]
            );
        }
    }
}
