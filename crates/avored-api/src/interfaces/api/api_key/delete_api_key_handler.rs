use axum::{
    extract::{Path, State},
    Json,
};

use crate::avored_state::AppState;
use crate::error::Result;

pub async fn delete_api_key_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    state.api_key_use_case.delete(&id).await?;
    Ok(Json(serde_json::json!({ "status": true })))
}
