use axum::{extract::{Path, State}, Json};
use crate::avored_state::AppState;
use crate::error::Result;

pub async fn fetch_collection_handler(
    State(state): State<AppState>,
    Path((entity_id, record_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>> {
    let response = state
        .entity_use_case
        .fetch_collection_by_id(&entity_id, &record_id)
        .await?;

    Ok(Json(response))
}
