#![recursion_limit = "256"]

use avored_rust_cms::{
    avored_state::test_avored_state, core::domain::{entities::user::StorableUser, repositories::MiscRepository}, infrastructure::persistence::misc_repository::test_misc_repository, interfaces::api::misc::setup_handler::setup_handler,
};
use axum::{Router, body::{Body, to_bytes}, http::Request, routing::post};
use tower::ServiceExt;

#[tokio::test]
async fn creates_a_user_from_an_in_memory_database() {
    let repository = test_misc_repository().await;

    let storable_user = StorableUser {
        name: String::from("Test user"),
        email: String::from("test@local.com"),
        password: String::from("123456"),
    };

    let user = repository
        .create_user(storable_user)
        .await
        .expect("create user issue");

    assert_eq!(user.name, "Test user");
}

#[tokio::test]
async fn misc_setup_handler_create_user_an_in_memory_database() {
    let app = Router::new()
        .route("/login", post(setup_handler))
        .with_state(test_avored_state().await);
    let request = Request::builder()
        .method("POST")
        .uri("/login")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"name":"test","email":"test@example.com","password":"secret"}"#,
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(response_json["success"], true);
}
