use axum::Extension;
use axum::{extract::Path, extract::State, Json};
use crate::avored_state::AppState;
use crate::core::application::dtos::UpdateAttributeCommand;
use crate::core::application::dtos::attribute_dto::AttributeResponse;
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn update_attribute_handler(
    State(state): State<AppState>,
    Extension(logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateAttributeCommand>,
) -> Result<Json<AttributeResponse>> {
    let locale = "en";
    payload.validate(locale, &state.attribute_use_case).await?;

    let updatable = payload.to_storable(logged_in_user.email.clone());

    let model = state.attribute_use_case.update(&id, updatable).await?;
    Ok(Json(model.into()))
}
