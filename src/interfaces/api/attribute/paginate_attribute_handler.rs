use axum::extract::Query;
use axum::{extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::attribute_dto::{AttributePaginationResponse, PaginateAttributeCommand};
use crate::error::Result;

pub async fn paginate_attributes_handler(
    State(state): State<AppState>,
    Query(command): Query<PaginateAttributeCommand>,
) -> Result<Json<AttributePaginationResponse>> {
    let (models, total) = state.attribute_use_case.paginate(command).await?;

    let response = AttributePaginationResponse {
        data: models.into_iter().map(Into::into).collect(),
        total,
    };
    Ok(Json(response))
}
