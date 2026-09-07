use axum::{extract::Path, extract::State, Json};
use serde::Serialize;
use crate::avored_state::AppState;
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct DeleteAttributeResponse {
    pub success: bool,
}

pub async fn delete_attribute_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeleteAttributeResponse>> {
    let success = state.attribute_use_case.delete(&id).await?;
    Ok(Json(DeleteAttributeResponse { success } ))
}
