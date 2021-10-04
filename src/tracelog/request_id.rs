use axum::{async_trait, body::Body, extract::{FromRequest, RequestParts}, http::{Response, StatusCode}, response::IntoResponse};
use thiserror::Error;
use uuid::Uuid;

/// A unique identifier generated for each incoming request.
///
/// Extracting a `RequestId` when the `TracingLogger` service is not
/// registered will result in an internal server error.
///
/// # Usage
///
/// todo
#[derive(Clone, Copy, Debug)]
pub struct RequestId(Uuid);

impl RequestId {
    pub(crate) fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

impl std::ops::Deref for RequestId {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<RequestId> for Uuid {
    fn from(r: RequestId) -> Self {
        r.0
    }
}

impl std::fmt::Display for RequestId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[async_trait]
impl<B> FromRequest<B> for RequestId
where
    B: Send,
{
    type Rejection = RequestIdExtractionError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection>{
        match req.extensions() {
            Some(e) => e.get::<RequestId>().cloned().ok_or(RequestIdExtractionError { _priv: () }),
            None => Err(RequestIdExtractionError { _priv: () }),
        }
    }
}

/// Error returned by the [`RequestId`] extractor when it fails to retrieve
/// the current request id from request-local storage.
///
/// It only occcurs when extracting the current request id without having
/// registered [`TracingLogger`] as a Tower Service for your application.
///
/// [`TracingLogger`]: crate::TracingLogger
#[derive(Error, Debug)]
pub struct RequestIdExtractionError {
    // Unit struct has a public constructor.
    // Thus, adding fields to it (public or private) later on is an API
    // breaking change.
    // Add a dummy private field that the compiler will optimise away
    // to make sure users cannot construct this error manually in their
    // own code.
    _priv: (),
}

impl IntoResponse for RequestIdExtractionError {
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;
    
    fn into_response(self) -> Response<Self::Body> {
        Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR).body(Body::empty()).unwrap()
    }
}

impl std::fmt::Display for RequestIdExtractionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to retrieve request id from request-local storage."
        )
    }
}

