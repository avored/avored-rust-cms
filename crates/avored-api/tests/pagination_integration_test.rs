#![allow(clippy::unwrap_used, clippy::expect_used)]

use avored_api::core::domain::entities::user::TokenClaims;
use avored_api::{
    avored_state::test_avored_state,
    core::domain::entities::entity::StorableEntity,
    interfaces::api::{
        api_key::paginate_api_keys_handler::paginate_api_keys_handler,
        api_manager::paginate_entities_handler::paginate_entities_handler as paginate_dynamic_entities_handler,
        collection::paginate_collections_handler::paginate_collections_handler,
        email_template::paginate_email_templates_handler::paginate_email_templates_handler,
        entity::paginate_entities_handler::paginate_entities_handler,
        workflow::list_workflows_handler,
    },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::get,
    Extension, Router,
};
use chrono::Utc;
use tower::ServiceExt;

fn pagination_api(state: avored_api::avored_state::AppState) -> Router {
    let claims = TokenClaims {
        sub: "users:pagination-test".to_string(),
        name: "Pagination Test".to_string(),
        email: "pagination@example.test".to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: (Utc::now().timestamp() + 3600) as usize,
    };

    Router::new()
        .route("/api/v1/admin/entities", get(paginate_entities_handler))
        .route(
            "/api/v1/admin/email-templates",
            get(paginate_email_templates_handler),
        )
        .route("/api/v1/admin/api-keys", get(paginate_api_keys_handler))
        .route("/api/v1/admin/workflows", get(list_workflows_handler))
        .route(
            "/api/v1/admin/collection",
            get(paginate_collections_handler),
        )
        .route("/api/v1/entities", get(paginate_dynamic_entities_handler))
        .layer(Extension(claims))
        .with_state(state)
}

async fn get_json(app: Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let response = app
        .oneshot(
            Request::builder()
                .uri(uri)
                .body(Body::empty())
                .expect("request should be valid"),
        )
        .await
        .expect("request should complete");
    let status = response.status();
    let body = to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("response body should be readable");
    (
        status,
        serde_json::from_slice(&body).expect("response should contain JSON"),
    )
}

#[tokio::test]
async fn list_endpoints_share_the_pagination_envelope_and_defaults() {
    let state = test_avored_state().await;
    let entity = state
        .entity_use_case
        .create(StorableEntity {
            name: "Pagination integration".to_string(),
            identifier: "pagination_integration".to_string(),
            logged_in_user_email: "pagination@example.test".to_string(),
            attributes: Vec::new(),
        })
        .await
        .expect("test entity should be created");
    let app = pagination_api(state);
    let collection_uri = format!("/api/v1/admin/collection?entity_id={}", entity.id);
    let endpoints = [
        ("/api/v1/admin/entities", 1),
        ("/api/v1/admin/email-templates", 0),
        ("/api/v1/admin/api-keys", 0),
        ("/api/v1/admin/workflows", 0),
        (collection_uri.as_str(), 0),
        ("/api/v1/entities?entity_type=pagination_integration", 0),
    ];

    for (uri, total) in endpoints {
        let (status, response) = get_json(app.clone(), uri).await;
        assert_eq!(status, StatusCode::OK, "request to {uri} should succeed");
        assert!(response["data"].is_array(), "{uri} should return data");
        assert_eq!(response["pagination"]["total"], total, "{uri}");
        assert_eq!(response["pagination"]["per_page"], 25, "{uri}");
        assert_eq!(response["pagination"]["current_page"], 1, "{uri}");
        assert_eq!(response["pagination"]["last_page"], 1, "{uri}");
    }

    let (status, response) = get_json(
        app,
        "/api/v1/entities?entity_type=pagination_integration&page=2&per_page=10",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(response["pagination"]["per_page"], 10);
    assert_eq!(response["pagination"]["current_page"], 2);
    assert_eq!(response["pagination"]["last_page"], 1);
}
