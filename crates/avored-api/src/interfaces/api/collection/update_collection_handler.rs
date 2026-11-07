use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Deserialize;

use crate::avored_state::AppState;
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct UpdateCollectionQuery {
    pub entity_id: String,
}

pub async fn update_collection_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path(record_id): Path<String>,
    Query(query): Query<UpdateCollectionQuery>,
    Json(payload): Json<serde_json::Map<String, serde_json::Value>>,
) -> Result<Json<serde_json::Value>> {
    let response = state
        .entity_use_case
        .update_collection(&query.entity_id, &record_id, payload, &logged_in_user.email)
        .await?;

    Ok(Json(response))
}
