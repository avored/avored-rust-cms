use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{CreateEntityCommand, EntityResponse};
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn create_entity_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Json(payload): Json<CreateEntityCommand>,
) -> Result<(StatusCode, Json<EntityResponse>)> {
    let locale = "en";
    
    payload.validate(locale, &state.entity_use_case).await?;

    let storable_entity = payload.to_storable(logged_in_user.email);

    let entity = state.entity_use_case.create(storable_entity).await?;

    Ok((StatusCode::CREATED, Json(entity)))
}
