use axum::{async_trait, body::Body, extract::RequestParts, http::{Response, StatusCode}, response::IntoResponse};
use axum::extract::FromRequest;
use thiserror::Error;
use tracing::Span;


/// A unique identifier generated for each incoming request.
///
/// Extracting a `RequestId` when the `TracingLogger` service is not
/// registered will result in an internal server error.
///
/// # Usage
///
/// todo
#[derive(Clone)]
pub struct RootSpan(Span);

impl RootSpan {
    pub(crate) fn new(span: Span) -> Self {
        Self(span)
    }
}

impl std::ops::Deref for RootSpan {
    type Target = Span;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RootSpan> for Span {
    fn from(r: RootSpan) -> Self {
        r.0
    }
}

#[async_trait]
impl<B> FromRequest<B> for RootSpan
where
    B: Send,
{
    type Rejection = RootSpanExtractionError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        match req.extensions() {
            Some(e) => e.get::<RootSpan>().cloned().ok_or(RootSpanExtractionError { _priv: () }),
            None => Err(RootSpanExtractionError { _priv: () }),
        }
    }
}

/// Error returned by the [`RootSpan`] extractor when it fails to retrieve
/// the current root span from request-local storage.
///
/// It only occcurs when extracting the current root span without having
/// registered [`TracingLogger`] as a Tower Service for your application.
///
/// [`TracingLogger`]: crate::TracingLogger
#[derive(Error, Debug)]
pub struct RootSpanExtractionError {
    // Unit struct has a public constructor.
    // Thus, adding fields to it (public or private) later on is an API
    // breaking change.
    // Add a dummy private field that the compiler will optimise away
    // to make sure users cannot construct this error manually in their
    // own code.
    _priv: (),
}

impl IntoResponse for RootSpanExtractionError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    
    fn into_response(self) -> Response<Self::Body> {
        Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap()
    }
}

impl std::fmt::Display for RootSpanExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to retrieve root span from request-local storage."
        )
    }
}
