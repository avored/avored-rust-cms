use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{requests::roles_request::RolesRequest, routes::{AppState, establish_connection}};

pub async fn roles_handler(
    app_state: State<Arc<AppState>>,
    Query(payload): Query<RolesRequest>,
) -> impl IntoResponse {
    let current_page: u64 = payload.current_page;
    let per_page = payload.per_page;

    let connection = establish_connection().await;

    let roles = app_state
        .role_repository
        .paginate(connection, per_page, current_page)
        .await;
    Json(roles).into_response()
}
