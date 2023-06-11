use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::State};

use crate::routes::AppState;

pub async fn home_handler(app_state : State<Arc<AppState>>) -> impl IntoResponse {
    Json(app_state.post_repository.all()).into_response()
}
