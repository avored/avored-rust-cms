use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use crate::prelude::AvoRedResult;

pub async fn log_request<T> (request: Request<T>, next: Next<T> ) -> AvoRedResult<Response> {
    println!("Hello log middleware");
    Ok(next.run(request).await)
}