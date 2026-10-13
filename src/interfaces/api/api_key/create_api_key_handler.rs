use axum::{extract::State, http::StatusCode, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::{ApiKeyResponse, CreateApiKeyRequest};
use crate::error::Result;

pub async fn create_api_key_handler(
    State(_state): State<AppState>,
    Json(_payload): Json<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiKeyResponse>)> {
    todo!()
}
