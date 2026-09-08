use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::CreateAttributeCommand;
use crate::core::application::dtos::attribute_dto::AttributeResponse;
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn create_attribute_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Json(payload): Json<CreateAttributeCommand>,
) -> Result<(StatusCode, Json<AttributeResponse>)> {
    let locale = "en";
    
    payload.validate(locale, &state.attribute_use_case).await?;

    let storable = payload.to_storable(logged_in_user.email);
    let model = state.attribute_use_case.create(storable).await?;

    Ok((StatusCode::CREATED, Json(model.into())))
}
