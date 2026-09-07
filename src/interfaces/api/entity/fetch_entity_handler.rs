use axum::{extract::Path, extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::EntityResponse;
use crate::error::Result;

pub async fn fetch_entity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<EntityResponse>> {
    let entity_model = state.entity_use_case.get_by_id(&id).await?;

    Ok(Json(entity_model.into()))
}
