use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::application::dtos::{
        CreateWorkflowCommand, SetWorkflowActiveCommand, UpdateWorkflowCommand,
        WorkflowPaginationQuery,
    },
    core::domain::entities::user::TokenClaims,
};

pub async fn list_workflows_handler(
    State(_state): State<AppState>,
    Query(_query): Query<WorkflowPaginationQuery>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn create_workflow_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Json(_payload): Json<CreateWorkflowCommand>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn fetch_workflow_handler(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn update_workflow_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(_id): Path<String>,
    Json(_payload): Json<UpdateWorkflowCommand>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn delete_workflow_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(_id): Path<String>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn set_workflow_active_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(_id): Path<String>,
    Json(_payload): Json<SetWorkflowActiveCommand>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

pub async fn list_workflow_executions_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(_id): Path<String>,
    Query(_query): Query<WorkflowPaginationQuery>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}
