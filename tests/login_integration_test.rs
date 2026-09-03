use std::sync::Arc;

use avored_rust_cms::{
    avored_state::AppState,
    core::{
        application::use_cases::{AuthUseCase, MiscUseCase},
        domain::repositories::AuthRepository,
    },
    infrastructure::persistence::{
        auth_repository::AuthRepositoryImpl, misc_repository::MiscRepositoryImpl,
    },
    interfaces::api::auth::login_handler,
    providers::{
        avored_config_provider::AvoRedConfigProvider,
        avored_database_provider::AvoRedDatabaseProvider,
    },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::post,
    Router,
};
use tower::ServiceExt;

async fn test_repository() -> AuthRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    let (datastore, session) = &provider.db;
    datastore
        .execute(
            "CREATE users:test_user SET name = 'Test User', email = 'test@example.com', password = 'secret';",
            session,
            None,
        )
        .await
        .expect("test user should be created");

    AuthRepositoryImpl::new(Arc::new(provider))
}

async fn test_state() -> AppState {
    let auth_repository = test_repository().await;
    let database_provider = auth_repository.database_provider.clone();
    let misc_repository = MiscRepositoryImpl::new(database_provider.clone());

    AppState {
        leptos_options: leptos::config::LeptosOptions::builder()
            .output_name("avored-rust-cms-test")
            .build(),
        auth_use_case: AuthUseCase::new(auth_repository),
        misc_use_case: MiscUseCase::new(misc_repository),
        database_provider,
        config: Arc::new(AvoRedConfigProvider {
            database_folder: "mem://".to_string(),
            database_name: "auth".to_string(),
            database_namespace: "test".to_string(),
            password_salt: String::new(),
            jwt_secret_key: String::new(),
            cors_allowed_app_url: vec![],
        }),
    }
}

#[tokio::test]
async fn authenticates_a_user_from_an_in_memory_database() {
    let repository = test_repository().await;

    let user = repository
        .authenticate("test@example.com", "secret")
        .await
        .expect("valid credentials should return a user");

    assert_eq!(user.id, "users:test_user");
    assert_eq!(user.name, "Test User");
}

#[tokio::test]
async fn rejects_invalid_credentials_in_an_in_memory_database() {
    let repository = test_repository().await;

    assert!(repository
        .authenticate("test@example.com", "wrong-password")
        .await
        .is_none());
}

#[tokio::test]
async fn login_handler_authenticates_against_an_in_memory_database() {
    let app = Router::new()
        .route("/login", post(login_handler))
        .with_state(test_state().await);
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
        .with_state(test_state().await);
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
