use axum::extract::State;
use axum::Extension;
use axum::{response::IntoResponse, Json};
use std::sync::Arc;

use crate::repositories::admin_user_repository::AdminUser;
use crate::routes::{AppState, establish_connection};

pub async fn home_handler(
    app_state: State<Arc<AppState>>,
    Extension(_current_user): Extension<AdminUser>,
) -> impl IntoResponse {
    let per_page: u64 = 2;
    let page: u64 = 0;
    let connection = establish_connection().await;

    let admin_users = app_state
        .admin_user_repository
        .paginate(connection, per_page, page)
        .await;
    Json(admin_users).into_response()
}
