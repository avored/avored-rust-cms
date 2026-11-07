use crate::avored_state::AppState;
use crate::core::application::dtos::collection_dto::{
    CollectionPaginationResponse, PaginateCollectionCommand,
};
use crate::error::Result;
use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CollectionQuery {
    pub entity_id: String,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

pub async fn paginate_collections_handler(
    State(state): State<AppState>,
    Query(query): Query<CollectionQuery>,
) -> Result<Json<CollectionPaginationResponse>> {
    let command = PaginateCollectionCommand {
        page: query.page,
        page_size: query.page_size,
    };

    let response = state
        .entity_use_case
        .paginate_collection(&query.entity_id, command)
        .await?;

    Ok(Json(response))
}
