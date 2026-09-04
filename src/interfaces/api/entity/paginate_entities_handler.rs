use axum::extract::Query;
use axum::{extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{EntityPaginationResponse, PaginateEntityCommand};
use crate::error::Result;

pub async fn paginate_entities_handler(
    State(state): State<AppState>,
    Query(command): Query<PaginateEntityCommand>,
) -> Result<Json<EntityPaginationResponse>> {
    let response = state.entity_use_case.paginate(command).await?;
    Ok(Json(response))
}
