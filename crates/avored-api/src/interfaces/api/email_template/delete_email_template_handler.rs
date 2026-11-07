use axum::{extract::Path, extract::State, Json};
use serde::Serialize;

use crate::{avored_state::AppState, error::Result};

#[derive(Debug, Serialize)]
pub struct DeleteEmailTemplateResponse {
    pub success: bool,
}

pub async fn delete_email_template_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<DeleteEmailTemplateResponse>> {
    let success = state.email_template_use_case.delete(&id).await?;
    Ok(Json(DeleteEmailTemplateResponse { success }))
}
