use axum::{extract::Path, extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{EntityResponse, UpdateEntityCommand};
use crate::error::Result;

pub async fn update_entity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateEntityCommand>,
) -> Result<Json<EntityResponse>> {
    let locale = "en";
    payload.validate(locale)?;

    let entity = state.entity_use_case.update(&id, payload).await?;
    Ok(Json(entity))
}
