use axum::{extract::Path, extract::State, http::StatusCode, response::IntoResponse, Json};
use crate::avored_state::AppState;
use crate::error::Result;

pub async fn fetch_entity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<axum::response::Response> {
    match state.entity_use_case.get_by_id(&id).await? {
        Some(entity) => Ok((StatusCode::OK, Json(entity)).into_response()),
        None => Ok((StatusCode::NOT_FOUND, "Entity not found").into_response()),
    }
}
