use crate::avored_state::AppState;
use crate::error::Result;
use axum::{extract::Path, extract::State, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DeleteEntityResponse {
    pub success: bool,
}

pub async fn delete_entity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeleteEntityResponse>> {
    let success = state.entity_use_case.delete(&id).await?;
    Ok(Json(DeleteEntityResponse { success }))
}
