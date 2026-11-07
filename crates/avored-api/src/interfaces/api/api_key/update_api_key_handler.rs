use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::{ApiKeyResponse, UpdateApiKeyRequest};
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;
use crate::interfaces::api::api_key::api_key_model_to_response;

pub async fn update_api_key_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<TokenClaims>,
    Path(id): Path<String>,
    Json(body): Json<UpdateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>> {
    let model = state
        .api_key_use_case
        .update(&id, body.name, body.permissions, claims.email)
        .await?;
    Ok(Json(api_key_model_to_response(model)))
}
