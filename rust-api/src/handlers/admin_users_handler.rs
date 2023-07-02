use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::{State, Query}};

use crate::{
    routes::AppState, 
    requests::admin_user_list_request::AdminUsersRequest
};

pub async fn admin_users_handler(
    app_state : State<Arc<AppState>>,
    Query(payload): Query<AdminUsersRequest>
) -> impl IntoResponse {
    
    let current_page: u64 = payload.current_page;
    let per_page = payload.per_page;
    let admin_users = app_state.admin_user_repository.paginate(per_page, current_page).await;
    Json(admin_users).into_response()
}
