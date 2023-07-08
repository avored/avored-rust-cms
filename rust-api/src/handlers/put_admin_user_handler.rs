use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{requests::update_admin_user_request::UpdateAdminUsersRequest, routes::{AppState, establish_connection}};

pub async fn put_admin_user_handler(
    app_state: State<Arc<AppState>>,
    Path(admin_user_id): Path<String>,
    Json(payload): Json<UpdateAdminUsersRequest>,
) -> impl IntoResponse {
    let admin_user_uuid: Uuid = Uuid::parse_str(&admin_user_id).unwrap();
    let connection = establish_connection().await;
    let admin_user = app_state
        .admin_user_repository
        .update_by_uuid(connection, admin_user_uuid, payload.email)
        .await;

    Json(admin_user).into_response()
}
