use axum::{extract::State, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::api_key_dto::ListApiKeysResponse;
use crate::error::Result;

pub async fn paginate_api_keys_handler(
    State(_state): State<AppState>,
) -> Result<Json<ListApiKeysResponse>> {
    todo!()
}
