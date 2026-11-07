#![allow(clippy::unwrap_used, clippy::expect_used)]

use avored_api::{api::rest_api_routes::rest_api_routes, avored_state::test_avored_state};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn health_check_returns_database_status_without_authentication() {
    let app =
        rest_api_routes(test_avored_state().await).expect("application routes should be created");

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("health request should complete");

    assert_eq!(response.status(), StatusCode::OK);

    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("response body should be valid JSON");

    assert_eq!(response_json["status"], "healthy");
    assert_eq!(response_json["database"], "connected");
    assert_eq!(response_json["version"], env!("CARGO_PKG_VERSION"));
}
