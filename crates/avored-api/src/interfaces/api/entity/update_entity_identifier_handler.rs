use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::{
        application::dtos::entity_dto::{EntityResponse, PutEntityIdentifierCommand},
        domain::entities::user::TokenClaims,
    },
    error::Result,
};

pub async fn update_entity_identifier_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    Json(payload): Json<PutEntityIdentifierCommand>,
) -> Result<Json<EntityResponse>> {
    let locale = "en";
    payload.validate(locale, &state.entity_use_case).await?;

    let updatable_identifier = payload.to_updatable_identifier(logged_in_user.email.clone());

    let model = state
        .entity_use_case
        .update_identifier(&id, updatable_identifier)
        .await?;
    Ok(Json(model.into()))
}
