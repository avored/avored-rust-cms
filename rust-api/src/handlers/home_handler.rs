use axum::Extension;
use axum::{response::IntoResponse, Json};
use axum::{extract::{State}};
use std::sync::Arc;

use crate::repositories::admin_user_repository::AdminUser;
use crate::routes::AppState;


pub async fn home_handler (
    app_state : State<Arc<AppState>>,
    Extension(_current_user): Extension<AdminUser>,
) -> impl IntoResponse { 
    let per_page: u64 = 1;
    let page: u64 = 1;

    let admin_users = app_state.admin_user_repository.paginate(per_page, page).await;
    Json(admin_users).into_response()
}
