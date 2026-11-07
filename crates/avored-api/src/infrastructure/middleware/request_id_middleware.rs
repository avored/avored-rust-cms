use axum::{
    extract::Request,
    http::{HeaderName, HeaderValue},
    middleware::Next,
    response::Response,
};

pub static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

#[derive(Clone, Debug)]
pub struct RequestId(pub String);

/// Middleware that inspects or creates a unique Request ID for each HTTP request,
/// stores it in request extensions, records it on the current tracing span,
/// and attaches it to the outgoing response header.
pub async fn request_id_middleware(mut request: Request, next: Next) -> Response {
    let req_id = request
        .headers()
        .get(&X_REQUEST_ID)
        .and_then(|val| val.to_str().ok())
        .filter(|val| !val.trim().is_empty())
        .map(|val| val.to_string())
        .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

    tracing::Span::current().record("request_id", tracing::field::display(&req_id));

    request.extensions_mut().insert(RequestId(req_id.clone()));

    let mut response = next.run(request).await;

    if let Ok(header_val) = HeaderValue::from_str(&req_id) {
        response
            .headers_mut()
            .insert(X_REQUEST_ID.clone(), header_val);
    }

    response
}
