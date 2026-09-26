use axum::{extract::State, http::StatusCode, Extension, Json};

use crate::{
    avored_state::AppState,
    core::{
        application::dtos::email_template_dto::CreateEmailTemplateCommand,
        domain::entities::user::TokenClaims,
    },
    error::Result,
};

pub async fn create_email_template_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Json(payload): Json<CreateEmailTemplateCommand>,
) -> Result<(StatusCode, Json<crate::core::domain::entities::EmailTemplateModel>)> {
    let template = state.email_template_use_case.create(payload).await?;
    Ok((StatusCode::CREATED, Json(template)))
}
