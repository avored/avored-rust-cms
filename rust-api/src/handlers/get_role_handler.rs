use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use std::sync::Arc;
use uuid::Uuid;

use crate::routes::{AppState, establish_connection};

pub async fn get_role_handler(
    app_state: State<Arc<AppState>>,
    Path(role_id): Path<String>,
) -> impl IntoResponse {
    let role_uuid: Uuid = Uuid::parse_str(&role_id).unwrap();
    let connection = establish_connection().await;
    let admin_user = app_state
        .role_repository
        .find_by_uuid(connection, role_uuid)
        .await;

    Json(admin_user).into_response()
}
