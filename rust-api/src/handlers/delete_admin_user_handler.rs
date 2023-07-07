use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::routes::AppState;

pub async fn delete_admin_user_handler(
    app_state: State<Arc<AppState>>,
    Path(admin_user_id): Path<String>,
) -> impl IntoResponse {
    let admin_user_uuid: Uuid = Uuid::parse_str(&admin_user_id).unwrap();
    app_state
        .admin_user_repository
        .delete_by_uuid(admin_user_uuid)
        .await;

    Json(StatusCode::NO_CONTENT).into_response()
}
