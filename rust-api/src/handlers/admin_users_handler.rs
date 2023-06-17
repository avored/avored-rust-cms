use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::State};

use crate::{routes::AppState, responses::admin_user_response::AdminUserResponse};

pub async fn admin_users_handler(app_state : State<Arc<AppState>>) -> impl IntoResponse {

    let admin_users = app_state.admin_user_repository.all();
    let mut admin_users_response: Vec<AdminUserResponse> = Vec::new();

    for admin_user_model in admin_users {
        let admin_user_response: AdminUserResponse = admin_user_model.into();
        admin_users_response.push(admin_user_response);
    }

    Json(admin_users_response).into_response()
}
