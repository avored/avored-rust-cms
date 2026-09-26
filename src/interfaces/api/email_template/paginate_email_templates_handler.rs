use axum::extract::{Query, State};
use axum::Json;

use crate::avored_state::AppState;
use crate::core::application::dtos::email_template_dto::{
    EmailTemplatePaginationResponse, PaginateEmailTemplateCommand,
};
use crate::error::Result;

pub async fn paginate_email_templates_handler(
    State(state): State<AppState>,
    Query(command): Query<PaginateEmailTemplateCommand>,
) -> Result<Json<EmailTemplatePaginationResponse>> {
    let response = state.email_template_use_case.paginate(command).await?;
    Ok(Json(response))
}
