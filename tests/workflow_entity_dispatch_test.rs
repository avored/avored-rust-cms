#![allow(clippy::unwrap_used, clippy::expect_used)]
#![recursion_limit = "512"]

use avored_rust_cms::{
    avored_state::{test_avored_state, AppState},
    core::{
        application::dtos::{CreateWorkflowCommand, WorkflowPaginationQuery},
        domain::entities::{
            entity::{StorableEntity, StorableEntityAttribute},
            WorkflowAction, WorkflowEvent, WorkflowExecutionStatus, WorkflowTrigger,
        },
    },
    interfaces::api::api_manager::{
        create_entity_record_handler::create_entity_record_handler,
        update_entity_record_handler::update_entity_record_handler,
    },
};
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    routing::{patch, post},
    Router,
};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use tower::ServiceExt;

fn auth_token(state: &AppState) -> String {
    let now = Utc::now().timestamp() as usize;
    encode(
        &Header::default(),
        &avored_rust_cms::core::domain::entities::user::TokenClaims {
            sub: "users:workflow-test".to_string(),
            name: "Workflow Test".to_string(),
            email: "workflow@example.test".to_string(),
            iat: now,
            exp: now + 3600,
        },
        &EncodingKey::from_secret(state.config.jwt_secret_key.as_bytes()),
    )
    .expect("test JWT should be encoded")
}

fn request(method: &str, uri: &str, body: &str, token: &str) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .expect("request should be valid")
}

async fn wait_for_execution(state: &AppState, workflow_id: &str) {
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(3);
    loop {
        let history = state
            .workflow_use_case
            .find_executions(workflow_id, WorkflowPaginationQuery::default())
            .await
            .expect("execution history query should succeed");
        if history.total > 0 {
            assert_eq!(history.data[0].status, WorkflowExecutionStatus::Failed);
            return;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "background workflow execution should finish within the timeout"
        );
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn entity_create_and_update_dispatch_matching_workflows_without_changing_responses() {
    let state = test_avored_state().await;
    let entity = state
        .entity_use_case
        .create(StorableEntity {
            name: "Workflow dispatch customers".to_string(),
            identifier: "workflow_dispatch_customers".to_string(),
            logged_in_user_email: "workflow@example.test".to_string(),
            attributes: vec![StorableEntityAttribute {
                name: "Name".to_string(),
                identifier: "name".to_string(),
                data_type: "string".to_string(),
                field_type: "text".to_string(),
            }],
        })
        .await
        .expect("dynamic entity should be created");

    let email_action = || WorkflowAction::Email {
        template_id: "welcome-template".to_string(),
        recipient: "person@example.test".to_string(),
    };
    let created_workflow = state
        .workflow_use_case
        .create(CreateWorkflowCommand {
            name: "Customer created".to_string(),
            trigger: WorkflowTrigger {
                entity_type: entity.identifier.clone(),
                event: WorkflowEvent::Created,
            },
            is_active: Some(true),
            actions: vec![email_action()],
        })
        .await
        .expect("create-trigger workflow should be configured");
    let updated_workflow = state
        .workflow_use_case
        .create(CreateWorkflowCommand {
            name: "Customer updated".to_string(),
            trigger: WorkflowTrigger {
                entity_type: entity.identifier.clone(),
                event: WorkflowEvent::Updated,
            },
            is_active: Some(true),
            actions: vec![email_action()],
        })
        .await
        .expect("update-trigger workflow should be configured");

    let token = auth_token(&state);
    let app: Router = Router::new()
        .route("/api/v1/entities", post(create_entity_record_handler))
        .route("/api/v1/entities/{id}", patch(update_entity_record_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            avored_rust_cms::infrastructure::middleware::auth_middleware::check_auth,
        ))
        .with_state(state.clone());

    let create_response = app
        .clone()
        .oneshot(request(
            "POST",
            "/api/v1/entities",
            r#"{"entity_type":"workflow_dispatch_customers","attributes":{"name":"Ada"}}"#,
            &token,
        ))
        .await
        .expect("create request should complete");
    assert_eq!(create_response.status(), StatusCode::CREATED);
    let created_json: serde_json::Value = serde_json::from_slice(
        &to_bytes(create_response.into_body(), usize::MAX)
            .await
            .expect("create response body should be readable"),
    )
    .expect("create response should be JSON");
    assert_eq!(created_json["entity_type"], entity.identifier);
    assert_eq!(created_json["attributes"]["name"], "Ada");
    let record_id = created_json["id"]
        .as_str()
        .expect("create response should include record ID")
        .to_string();
    wait_for_execution(&state, &created_workflow.id).await;
    assert_eq!(
        state
            .workflow_use_case
            .find_executions(&updated_workflow.id, WorkflowPaginationQuery::default())
            .await
            .expect("update workflow history should be queryable")
            .total,
        0
    );

    let update_response = app
        .oneshot(request(
            "PATCH",
            &format!(
                "/api/v1/entities/{record_id}?entity_type={}",
                entity.identifier
            ),
            r#"{"attributes":{"name":"Grace"}}"#,
            &token,
        ))
        .await
        .expect("update request should complete");
    assert_eq!(update_response.status(), StatusCode::OK);
    let updated_json: serde_json::Value = serde_json::from_slice(
        &to_bytes(update_response.into_body(), usize::MAX)
            .await
            .expect("update response body should be readable"),
    )
    .expect("update response should be JSON");
    assert_eq!(updated_json["entity_type"], entity.identifier);
    assert_eq!(updated_json["attributes"]["name"], "Grace");
    wait_for_execution(&state, &updated_workflow.id).await;
}
