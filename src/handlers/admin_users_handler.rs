use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    requests::admin_user_list_request::AdminUsersRequest,
    routes::{establish_connection, AppState},
};

pub async fn admin_users_handler(
    app_state: State<Arc<AppState>>,
    Query(payload): Query<AdminUsersRequest>,
) -> impl IntoResponse {
    let current_page: u64 = match payload.current_page {
        Some(current_page) => current_page,
        None => 1,
    };
    let per_page = match payload.per_page {
        Some(per_page) => per_page,
        None => 10,
    };
    let connection = establish_connection().await;

    let admin_users = app_state
        .admin_user_repository
        .paginate(connection, per_page, current_page)
        .await;
    Json(admin_users).into_response()
}
