use axum::{extract::{Path, State}, Extension, Json};
use crate::avored_state::AppState;
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn update_collection_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path((entity_id, record_id)): Path<(String, String)>,
    Json(payload): Json<serde_json::Map<String, serde_json::Value>>,
) -> Result<Json<serde_json::Value>> {
    let response = state
        .entity_use_case
        .update_collection(&entity_id, &record_id, payload, &logged_in_user.email)
        .await?;

    Ok(Json(response))
}
