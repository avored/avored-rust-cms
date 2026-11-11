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
    pub per_page: Option<u64>,
    pub page_size: Option<u64>,
    pub limit: Option<u64>,
}

pub async fn paginate_collections_handler(
    State(state): State<AppState>,
    Query(query): Query<CollectionQuery>,
) -> Result<Json<CollectionPaginationResponse>> {
    let pagination_query = crate::core::application::dtos::pagination_dto::PaginationQuery {
        page: query.page,
        per_page: query.per_page,
        page_size: query.page_size,
        limit: query.limit,
    };
    let command = PaginateCollectionCommand {
        page: Some(pagination_query.current_page()),
        page_size: Some(pagination_query.per_page()),
    };

    let response = state
        .entity_use_case
        .paginate_collection(&query.entity_id, command)
        .await?;

    Ok(Json(response))
}
