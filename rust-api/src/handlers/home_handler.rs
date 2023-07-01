use axum::{response::IntoResponse, Json};
use axum::{extract::{State}};
use std::sync::Arc;

use crate::routes::AppState;

// use crate::{repositories::admin_user_repository::AdminUser};

pub async fn home_handler (
    app_state : State<Arc<AppState>>,
    // Extension(current_user): Extension<AdminUser>,
) -> impl IntoResponse { 

    let admin_users = app_state.admin_user_repository.paginate(1, 1).await;
    Json(admin_users).into_response()
}
