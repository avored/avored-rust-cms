use axum::{extract::State, http::StatusCode, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{CreateEntityCommand, EntityResponse};
use crate::error::Result;

pub async fn create_entity_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateEntityCommand>,
) -> Result<(StatusCode, Json<EntityResponse>)> {
    let locale = "en";
    payload.validate(locale)?;

    let entity = state.entity_use_case.create(payload).await?;

    Ok((StatusCode::CREATED, Json(entity)))
}
