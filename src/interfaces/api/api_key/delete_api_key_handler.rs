use axum::{
    extract::{Path, State},
    Json,
};

use crate::avored_state::AppState;
use crate::error::Result;

pub async fn delete_api_key_handler(
    State(_state): State<AppState>,
    Path(_id): Path<String>,
) -> Result<Json<bool>> {
    todo!()
}
