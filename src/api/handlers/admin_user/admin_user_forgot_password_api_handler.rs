use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use crate::api::handlers::admin_user::request::admin_user_forgot_password_request::AdminUserForgotPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::validation_error::ErrorResponse;

#[derive(Serialize, Default)]
pub struct ForgotPasswordViewModel {
    pub link: String
}

pub async fn admin_user_forgot_password_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AdminUserForgotPasswordRequest>,
) -> Result<Json<ResponseData>> {
    println!("->> {:<12} - admin_user_forgot_password_api_handler", "HANDLER");

    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let template = &state.template;
    let frontend_url = &state.config.front_end_app_url;
    let sent_status = state
        .admin_user_service.sent_forgot_password_email(template , frontend_url, payload.email).await?;

    let response_data = ResponseData {
        status: sent_status
    };

    Ok(Json(response_data))
}


#[derive(Serialize)]
pub struct ResponseData {
    status: bool
}