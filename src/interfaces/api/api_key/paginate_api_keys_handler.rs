use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::Json;

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::ListApiKeysResponse;
use crate::error::Result;
use crate::interfaces::api::api_key::api_key_model_to_response;

pub async fn paginate_api_keys_handler(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<ListApiKeysResponse>> {
    let page = params
        .get("page")
        .and_then(|v| v.parse().ok())
        .unwrap_or(1u64);
    let limit = params
        .get("limit")
        .and_then(|v| v.parse().ok())
        .unwrap_or(20u64);

    let (models, total_items) = state.api_key_use_case.list(page, limit).await?;
    let total_pages = total_items.div_ceil(limit);

    let data = models.into_iter().map(api_key_model_to_response).collect();
    Ok(Json(ListApiKeysResponse {
        data,
        page,
        limit,
        total_items,
        total_pages,
    }))
}
