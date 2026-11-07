use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::avored_state::AppState;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct FetchCollectionQuery {
    pub entity_id: String,
}

pub async fn fetch_collection_handler(
    State(state): State<AppState>,
    Path(record_id): Path<String>,
    Query(query): Query<FetchCollectionQuery>,
) -> Result<Json<serde_json::Value>> {
    let response = state
        .entity_use_case
        .fetch_collection_by_id(&query.entity_id, &record_id)
        .await?;

    Ok(Json(response))
}
