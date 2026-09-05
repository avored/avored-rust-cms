#![recursion_limit = "256"]

use avored_rust_cms::{
    avored_state::test_avored_state,
    infrastructure::persistence::auth_repository::test_auth_repository,
    interfaces::api::auth::login_handler,
    core::domain::repositories::AuthRepository,
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;

#[tokio::test]
async fn authenticates_a_user_from_an_in_memory_database() {
    let repository = test_auth_repository().await;

    let user = repository
        .authenticate("test@example.com")
        .await
        .expect("valid credentials should return a user");

    assert_eq!(user.id, "users:test_user");
    assert_eq!(user.name, "Test User");
}

#[tokio::test]
async fn rejects_invalid_credentials_in_an_in_memory_database() {
    let repository = test_auth_repository().await;

    assert!(repository
        .authenticate("test@example.com")
        .await
        .is_err());
}

#[tokio::test]
async fn login_handler_authenticates_against_an_in_memory_database() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(test_avored_state().await);
    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"email":"test@example.com","password":"secret"}"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_json["authenticated"], true);
    assert_eq!(response_json["user"]["email"], "test@example.com");
    assert_eq!(response_json["token"], "demo-token-for-users:test_user");
}

#[tokio::test]
async fn login_handler_rejects_invalid_credentials() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(test_avored_state().await);
    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"email":"test@example.com","password":"wrong-password"}"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
