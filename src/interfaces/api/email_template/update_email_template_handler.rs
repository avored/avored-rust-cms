use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    avored_state::AppState,
    core::{
        application::dtos::email_template_dto::UpdateEmailTemplateCommand,
        domain::entities::user::TokenClaims,
    },
    error::Result,
};

pub async fn update_email_template_handler(
    State(state): State<AppState>,
    Extension(_logged_in_user): Extension<TokenClaims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateEmailTemplateCommand>,
) -> Result<Json<crate::core::domain::entities::EmailTemplateModel>> {
    let template = state.email_template_use_case.update(&id, payload).await?;
    Ok(Json(template))
}
