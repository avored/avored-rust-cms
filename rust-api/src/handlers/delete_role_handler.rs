use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::routes::{AppState, establish_connection};

pub async fn delete_role_handler(
    app_state: State<Arc<AppState>>,
    Path(role_id): Path<String>,
) -> impl IntoResponse {
    let role_uuid: Uuid = Uuid::parse_str(&role_id).unwrap();
    let connection = establish_connection().await;
    app_state
        .role_repository
        .delete_by_uuid(connection, role_uuid)
        .await;

    Json(StatusCode::NO_CONTENT).into_response()
}
