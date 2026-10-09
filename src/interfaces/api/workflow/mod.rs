use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    http::StatusCode,
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::{
        application::dtos::{
            CreateWorkflowCommand, SetWorkflowActiveCommand, UpdateWorkflowCommand,
            WorkflowPaginationQuery, WorkflowPaginationResponse,
        },
        domain::entities::{user::TokenClaims, ErrorMessageResponse, ErrorResponse, WorkflowModel},
    },
    error::{Error, Result},
};

pub async fn list_workflows_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Query(query): Query<WorkflowPaginationQuery>,
) -> Result<Json<WorkflowPaginationResponse>> {
    let workflows = state.workflow_use_case.find_all(query).await?;
    Ok(Json(workflows))
}

pub async fn create_workflow_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    payload: std::result::Result<Json<CreateWorkflowCommand>, JsonRejection>,
) -> Result<(StatusCode, Json<WorkflowModel>)> {
    let Json(payload) = parse_json_payload(payload)?;
    let workflow = state.workflow_use_case.create(payload).await?;
    Ok((StatusCode::CREATED, Json(workflow)))
}

pub async fn fetch_workflow_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
) -> Result<Json<WorkflowModel>> {
    let workflow = state.workflow_use_case.find_by_id(&id).await?;
    Ok(Json(workflow))
}

pub async fn update_workflow_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    payload: std::result::Result<Json<UpdateWorkflowCommand>, JsonRejection>,
) -> Result<Json<WorkflowModel>> {
    let Json(payload) = parse_json_payload(payload)?;
    let workflow = state.workflow_use_case.update(&id, payload).await?;
    Ok(Json(workflow))
}

pub async fn delete_workflow_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
) -> Result<StatusCode> {
    state.workflow_use_case.delete(&id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn set_workflow_active_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    payload: std::result::Result<Json<SetWorkflowActiveCommand>, JsonRejection>,
) -> Result<Json<WorkflowModel>> {
    let Json(payload) = parse_json_payload(payload)?;
    let workflow = state.workflow_use_case.set_active(&id, payload).await?;
    Ok(Json(workflow))
}

pub async fn list_workflow_executions_handler(
    State(_state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(_id): Path<String>,
    Query(_query): Query<WorkflowPaginationQuery>,
) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

fn parse_json_payload<T>(payload: std::result::Result<Json<T>, JsonRejection>) -> Result<Json<T>> {
    payload.map_err(|rejection| {
        Error::BadRequest(ErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "body".to_string(),
                message: format!("Invalid JSON request body: {}", rejection.body_text()),
            }],
        })
    })
}
