use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::{ApiKeyResponse, CreateApiKeyRequest};
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;
use crate::interfaces::api::api_key::api_key_model_to_response;

pub async fn create_api_key_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<TokenClaims>,
    Json(body): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiKeyResponse>)> {
    let (model, plain_secret) = state
        .api_key_use_case
        .create(body.name, body.permissions, claims.email)
        .await?;

    let mut response = api_key_model_to_response(model);
    response.plain_secret = Some(plain_secret);
    Ok((StatusCode::CREATED, Json(response)))
}
