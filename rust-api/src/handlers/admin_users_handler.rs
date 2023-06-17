use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::State};

use crate::{routes::AppState, responses::admin_user_response::AdminUserResponse};

pub async fn admin_users_handler(app_state : State<Arc<AppState>>) -> impl IntoResponse {

    let admin_users = app_state.admin_user_repository.find_by_email("admin@admin.com".to_string());
    let admin_users_response: AdminUserResponse = admin_users.into();

    Json(admin_users_response).into_response()
}
