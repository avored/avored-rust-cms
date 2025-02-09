use crate::api::handlers::admin_user::request::admin_user_forgot_password_request::AdminUserForgotPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use crate::responses::ApiResponse;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;
use rust_i18n::t;

#[derive(Serialize, Default)]
pub struct ForgotPasswordViewModel {
    pub link: String,
}

pub async fn admin_user_forgot_password_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AdminUserForgotPasswordRequest>,
) -> Result<Json<ApiResponse<bool>>> {
    println!("->> {:<12} - admin_user_forgot_password_api_handler", "HANDLER");

    let error_messages = payload.validate()?;
    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };
        return Err(Error::BadRequest(error_response));
    }

    let template = &state.template;
    let react_admin_url = &state.config.react_admin_app_url;
    match state
        .admin_user_service
        .sent_forgot_password_email(&state.db, template, react_admin_url, &payload.email)
        .await {
        Ok(sent_status) =>  {
            let response_data = ApiResponse {
                status: true,
                data: sent_status,
            };

            Ok(Json(response_data))
        },
        Err(e) => match e {
            Error::ModelNotFound(_e) => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: t!("email_address_not_valid").to_string(),
                };
                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors
                };

                Err(Error::BadRequest(error_response))
            },
            _ => Err(Error::Generic("500 Internal Server Error".to_string()))
        }
    }
}
