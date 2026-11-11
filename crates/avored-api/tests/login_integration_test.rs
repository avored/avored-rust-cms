#![allow(clippy::unwrap_used, clippy::expect_used)]
#![recursion_limit = "256"]

mod common;

use avored_api::{
    avored_state::test_avored_state, core::domain::repositories::AuthRepository,
    infrastructure::persistence::auth_repository::test_auth_repository,
    interfaces::api::auth::login_handler,
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
    common::seed_test_user(&repository.database_provider).await;

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
    common::seed_test_user(&repository.database_provider).await;

    assert!(repository
        .authenticate("unknown@example.com")
        .await
        .is_err());
}

#[tokio::test]
async fn login_handler_authenticates_against_an_in_memory_database() {
    let state = test_avored_state().await;
    common::seed_test_user(&state.database_provider).await;
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(state);
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
    assert!(response_json["token"]
        .as_str()
        .is_some_and(|token| !token.is_empty()));
}

#[tokio::test]
async fn login_handler_rejects_missing_required_fields() {
    let state = test_avored_state().await;
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(state);
    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"email":"test@example.com"}"#))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn login_handler_rejects_invalid_credentials() {
    let state = test_avored_state().await;
    common::seed_test_user(&state.database_provider).await;
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(state);
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

#[tokio::test]
async fn login_handler_rejects_unknown_users_with_unauthorized() {
    let state = test_avored_state().await;
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(state);
    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"email":"unknown@example.com","password":"secret"}"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
