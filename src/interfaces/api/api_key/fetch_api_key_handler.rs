use axum::{
    extract::{Path, State},
    Json,
};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::ApiKeyResponse;
use crate::error::Result;
use crate::interfaces::api::api_key::api_key_model_to_response;

pub async fn fetch_api_key_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ApiKeyResponse>> {
    let model = state.api_key_use_case.find_by_id(&id).await?;
    Ok(Json(api_key_model_to_response(model)))
}
