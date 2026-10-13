use axum::{
    extract::{Path, State},
    Json,
};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::{ApiKeyResponse, UpdateApiKeyRequest};
use crate::error::Result;

pub async fn update_api_key_handler(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
    Json(_payload): Json<UpdateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>> {
    todo!()
}
