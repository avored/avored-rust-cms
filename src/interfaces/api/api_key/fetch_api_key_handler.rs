use axum::{
    extract::{Path, State},
    Json,
};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::ApiKeyResponse;
use crate::error::Result;

pub async fn fetch_api_key_handler(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<ApiKeyResponse>> {
    todo!()
}
