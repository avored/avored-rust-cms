use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::avored_state::AppState;
use crate::error::Result;

#[derive(Debug, Deserialize)]
pub struct DeleteCollectionQuery {
    entity_id: String,
}

#[derive(Debug, Serialize)]
pub struct DeleteCollectionResponse {
    pub success: bool,
}

pub async fn delete_collection_handler(
    State(state): State<AppState>,
    Path(record_id): Path<String>,
    Query(query): Query<DeleteCollectionQuery>,
) -> Result<Json<DeleteCollectionResponse>> {
    let success = state
        .entity_use_case
        .delete_collection(&query.entity_id, &record_id)
        .await?;

    Ok(Json(DeleteCollectionResponse { success }))
}
