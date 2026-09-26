use axum::{extract::{Path, State}, Json};

use crate::{
    avored_state::AppState,
    error::Result,
};

pub async fn fetch_email_template_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<crate::core::domain::entities::EmailTemplateModel>> {
    let template = state.email_template_use_case.get_by_id(&id).await?;
    Ok(Json(template))
}
