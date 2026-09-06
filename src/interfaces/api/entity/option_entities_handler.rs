use axum::{extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{ EntityOptionResponse};
use crate::error::Result;
 
pub async fn option_entities_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<EntityOptionResponse>>> {
    let entities = state.entity_use_case.list_options().await?;

    let entities_options = entities
        .into_iter()
        .map(|entity| EntityOptionResponse {
            id: entity.id,
            name: entity.name,
        })
        .collect::<Vec<EntityOptionResponse>>();


    Ok(Json(entities_options))
    
}
