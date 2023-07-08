use axum::{extract::State, response::IntoResponse, Extension, Json};
use std::sync::Arc;

use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::create_role_request::CreateRoleRequest,
    responses::role_response::RoleResponse,
    routes::{establish_connection, AppState},
};

pub async fn create_role_handler(
    Extension(current_user): Extension<AdminUser>,
    app_state: State<Arc<AppState>>,
    Json(payload): Json<CreateRoleRequest>,
) -> impl IntoResponse {
    let connection = establish_connection().await;

    let created_role_model = app_state
        .role_repository
        .create(connection, payload, current_user.email)
        .await;

    Json(RoleResponse::transform_into_response(created_role_model)).into_response()
}
