use axum::Extension;
use axum::{extract::Path, extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::entity_dto::{EntityResponse, UpdateEntityCommand};
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn update_entity_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateEntityCommand>,
) -> Result<Json<EntityResponse>> {
    let locale = "en";
    payload.validate(locale, &state.entity_use_case).await?;

    let updatable_entity = payload.to_storable(logged_in_user.email.clone());

    let entity = state.entity_use_case.update(&id, updatable_entity).await?;
    Ok(Json(entity))
}
