use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::update_role_request::UpdateRoleRequest,
    routes::{establish_connection, AppState},
};

pub async fn put_role_handler(
    Extension(current_user): Extension<AdminUser>,
    app_state: State<Arc<AppState>>,
    Path(role_id): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> impl IntoResponse {
    let role_uuid: Uuid = Uuid::parse_str(&role_id).unwrap();
    let connection = establish_connection().await;
    let updated_role_model = app_state
        .role_repository
        .update_by_uuid(connection, role_uuid, payload, current_user.email)
        .await;

    Json(updated_role_model).into_response()
}
