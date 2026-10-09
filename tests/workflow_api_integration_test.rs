#![recursion_limit = "512"]

use avored_rust_cms::{
    avored_state::test_avored_state,
    core::domain::entities::user::TokenClaims,
    interfaces::api::workflow::{
        create_workflow_handler, delete_workflow_handler, fetch_workflow_handler,
        list_workflows_handler, set_workflow_active_handler, update_workflow_handler,
    },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::{delete, get, patch, post, put},
    Router,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use tower::ServiceExt;

fn token(secret: &str) -> String {
    let now = Utc::now().timestamp() as usize;
    encode(
        &Header::default(),
        &TokenClaims {
            sub: "users:workflow-test".to_string(),
            name: "Workflow Test".to_string(),
            email: "workflow@example.test".to_string(),
            iat: now,
            exp: now + 3600,
        },
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("test JWT should be encoded")
}

fn workflow_api(state: avored_rust_cms::avored_state::AppState) -> Router {
    Router::new()
        .route("/api/workflows", get(list_workflows_handler))
        .route("/api/workflows", post(create_workflow_handler))
        .route("/api/workflows/{id}", get(fetch_workflow_handler))
        .route("/api/workflows/{id}", put(update_workflow_handler))
        .route("/api/workflows/{id}", delete(delete_workflow_handler))
        .route(
            "/api/workflows/{id}/active",
            patch(set_workflow_active_handler),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            avored_rust_cms::infrastructure::middleware::auth_middleware::check_auth,
        ))
        .with_state(state)
}

fn request(method: &str, uri: &str, body: Option<&str>, jwt: Option<&str>) -> Request<Body> {
    let mut builder = Request::builder().method(method).uri(uri);
    if let Some(jwt) = jwt {
        builder = builder.header("authorization", format!("Bearer {jwt}"));
    }
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    builder
        .body(
            body.map(str::to_string)
                .map(Body::from)
                .unwrap_or_else(Body::empty),
        )
        .expect("request should be valid")
}

async fn response_json(response: axum::response::Response) -> serde_json::Value {
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    serde_json::from_slice(&body).expect("response should contain JSON")
}

#[tokio::test]
async fn workflow_api_requires_authentication() {
    let state = test_avored_state().await;
    let app = workflow_api(state);
    let response = app
        .oneshot(request("GET", "/api/workflows", None, None))
        .await
        .expect("request should complete");

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn workflow_api_supports_authenticated_crud_and_activation() {
    let state = test_avored_state().await;
    let jwt = token(&state.config.jwt_secret_key);
    let app = workflow_api(state);
    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/workflows",
            Some(
                r#"{
                    "name":"  Customer created  ",
                    "trigger":{"entity_type":" customers ","event":"created"},
                    "actions":[{"type":"webhook","url":"https://example.test/hook"}]
                }"#,
            ),
            Some(&jwt),
        ))
        .await
        .expect("create request should complete");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created = response_json(create_response).await;
    let id = created["id"]
        .as_str()
        .expect("created workflow should contain an id")
        .to_string();
    assert_eq!(created["name"], "Customer created");
    assert_eq!(created["trigger"]["entity_type"], "customers");
    assert_eq!(created["is_active"], true);

    let list_response = app
        .clone()
        .oneshot(request(
            "GET",
            "/api/workflows?page=1&page_size=10",
            None,
            Some(&jwt),
        ))
        .await
        .expect("list request should complete");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list = response_json(list_response).await;
    assert_eq!(list["total"], 1);
    assert_eq!(list["data"][0]["id"], id);

    let fetch_response = app
        .clone()
        .oneshot(request(
            "GET",
            &format!("/api/workflows/{id}"),
            None,
            Some(&jwt),
        ))
        .await
        .expect("fetch request should complete");
    assert_eq!(fetch_response.status(), StatusCode::OK);

    let update_response = app
        .clone()
        .oneshot(request(
            "PUT",
            &format!("/api/workflows/{id}"),
            Some(
                r#"{
                    "name":"Customer updated",
                    "trigger":{"entity_type":"customers","event":"updated"},
                    "is_active":true,
                    "actions":[{"type":"email","template_id":"welcome","recipient":"user@example.test"}]
                }"#,
            ),
            Some(&jwt),
        ))
        .await
        .expect("update request should complete");
    assert_eq!(update_response.status(), StatusCode::OK);
    let updated = response_json(update_response).await;
    assert_eq!(updated["name"], "Customer updated");
    assert_eq!(updated["actions"][0]["type"], "email");

    let deactivate_response = app
        .clone()
        .oneshot(request(
            "PATCH",
            &format!("/api/workflows/{id}/active"),
            Some(r#"{"is_active":false}"#),
            Some(&jwt),
        ))
        .await
        .expect("activation request should complete");
    assert_eq!(deactivate_response.status(), StatusCode::OK);
    assert_eq!(response_json(deactivate_response).await["is_active"], false);

    let delete_response = app
        .clone()
        .oneshot(request(
            "DELETE",
            &format!("/api/workflows/{id}"),
            None,
            Some(&jwt),
        ))
        .await
        .expect("delete request should complete");
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

    let missing_response = app
        .oneshot(request(
            "GET",
            &format!("/api/workflows/{id}"),
            None,
            Some(&jwt),
        ))
        .await
        .expect("missing workflow request should complete");
    assert_eq!(missing_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn workflow_api_returns_structured_bad_request_for_invalid_definitions() {
    let state = test_avored_state().await;
    let jwt = token(&state.config.jwt_secret_key);
    let app = workflow_api(state);

    let blank_fields = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/workflows",
            Some(
                r#"{
                    "name":" ",
                    "trigger":{"entity_type":" ","event":"created"},
                    "actions":[]
                }"#,
            ),
            Some(&jwt),
        ))
        .await
        .expect("invalid request should complete");
    assert_eq!(blank_fields.status(), StatusCode::BAD_REQUEST);
    let error = response_json(blank_fields).await;
    assert_eq!(error["status"], false);
    assert_eq!(error["errors"].as_array().unwrap().len(), 2);

    let invalid_action = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/workflows",
            Some(
                r#"{
                    "name":"Invalid action",
                    "trigger":{"entity_type":"customers","event":"created"},
                    "actions":[{"type":"unknown"}]
                }"#,
            ),
            Some(&jwt),
        ))
        .await
        .expect("invalid action request should complete");
    assert_eq!(invalid_action.status(), StatusCode::BAD_REQUEST);
    let error = response_json(invalid_action).await;
    assert_eq!(error["status"], false);

    let invalid_webhook = app
        .oneshot(request(
            "POST",
            "/api/workflows",
            Some(
                r#"{
                    "name":"Invalid webhook",
                    "trigger":{"entity_type":"customers","event":"created"},
                    "actions":[{"type":"webhook","url":"file:///etc/passwd"}]
                }"#,
            ),
            Some(&jwt),
        ))
        .await
        .expect("invalid webhook request should complete");
    assert_eq!(invalid_webhook.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn workflow_api_converts_malformed_json_to_structured_bad_request() {
    let state = test_avored_state().await;
    let jwt = token(&state.config.jwt_secret_key);
    let app = workflow_api(state);
    let response = app
        .oneshot(request(
            "POST",
            "/api/workflows",
            Some(r#"{"name":"broken""#),
            Some(&jwt),
        ))
        .await
        .expect("malformed request should complete");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error = response_json(response).await;
    assert_eq!(error["status"], false);
    assert_eq!(error["errors"][0]["key"], "body");
}
