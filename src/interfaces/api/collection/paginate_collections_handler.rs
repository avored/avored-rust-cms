use crate::avored_state::AppState;
use crate::core::application::dtos::collection_dto::{
    CollectionPaginationResponse, PaginateCollectionCommand,
};
use crate::error::Result;
use axum::extract::{Path, Query};
use axum::{extract::State, Json};

pub async fn paginate_collections_handler(
    State(state): State<AppState>,
    Path(entity_id): Path<String>,
    Query(command): Query<PaginateCollectionCommand>,
) -> Result<Json<CollectionPaginationResponse>> {
    let response = state
        .entity_use_case
        .paginate_collection(&entity_id, command)
        .await?;

    Ok(Json(response))
}
