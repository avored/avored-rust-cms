#![allow(clippy::unwrap_used, clippy::expect_used)]

use avored_api::infrastructure::middleware::request_id_middleware;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
    response::IntoResponse,
    routing::get,
    Router,
};
use tower::ServiceExt;

/// A trivial handler that always returns 200 OK.
async fn ok_handler() -> impl IntoResponse {
    (StatusCode::OK, "ok")
}

/// A trivial handler that always returns a 404 error response.
async fn not_found_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
}

/// Builds a minimal router with the request_id_middleware applied.
fn app() -> Router {
    Router::new()
        .route("/ok", get(ok_handler))
        .route("/not-found", get(not_found_handler))
        .layer(middleware::from_fn(
            request_id_middleware::request_id_middleware,
        ))
}

#[tokio::test]
async fn response_contains_x_request_id_header() {
    let response = app()
        .oneshot(Request::builder().uri("/ok").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let req_id = response
        .headers()
        .get("x-request-id")
        .expect("x-request-id header should be present");

    // Should be a valid non-empty UUID-shaped string
    let id_str = req_id.to_str().unwrap();
    assert!(!id_str.is_empty(), "request id must not be empty");
}

#[tokio::test]
async fn passes_through_existing_x_request_id() {
    let custom_id = "my-trace-id-abc123";

    let response = app()
        .oneshot(
            Request::builder()
                .uri("/ok")
                .header("x-request-id", custom_id)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let returned_id = response
        .headers()
        .get("x-request-id")
        .expect("x-request-id header should be present")
        .to_str()
        .unwrap();

    assert_eq!(
        returned_id, custom_id,
        "existing request id should be echoed back"
    );
}

#[tokio::test]
async fn error_responses_still_include_x_request_id() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/not-found")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let req_id = response
        .headers()
        .get("x-request-id")
        .expect("x-request-id header should be present even on error responses");

    assert!(!req_id.to_str().unwrap().is_empty());
}

#[tokio::test]
async fn each_request_gets_a_unique_id() {
    let response1 = app()
        .oneshot(Request::builder().uri("/ok").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let response2 = app()
        .oneshot(Request::builder().uri("/ok").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let id1 = response1
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let id2 = response2
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    assert_ne!(
        id1, id2,
        "each request should receive a distinct correlation id"
    );
}
