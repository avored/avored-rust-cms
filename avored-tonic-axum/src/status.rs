use axum::response::{IntoResponse, Response};
use std::ops::{Deref, DerefMut};

/// A simple wrapper around a `tonic::Status` usable in axum middleware.
///
/// ## Example
/// ```
/// use axum::{middleware::{Next, from_fn}, response::Response, Router, extract::Request};
/// use axum_tonic::GrpcStatus;
///
/// async fn tonic_middleware(
///     req: Request,
///     next: Next
/// ) -> Result<Response, GrpcStatus> {
///     if is_auth(&req) {
///         Ok(next.run(req).await)
///     } else {
///         Err(
///             tonic::Status::permission_denied("Not authenticated").into()
///         )
///     }
/// }
///
/// fn is_auth<B>(req: &Request<B>) -> bool {
///     true // or other logic
/// }
///
/// let router: Router<()> = Router::new()
///     .layer(from_fn(tonic_middleware));
/// ```
#[derive(Debug)]
pub struct GrpcStatus(pub tonic::Status);

impl From<tonic::Status> for GrpcStatus {
    fn from(s: tonic::Status) -> Self {
        Self(s)
    }
}

impl Deref for GrpcStatus {
    type Target = tonic::Status;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GrpcStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoResponse for GrpcStatus {
    fn into_response(self) -> Response {
        self.0.into_http::<axum::body::Body>().into_response()
    }
}