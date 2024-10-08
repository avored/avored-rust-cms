use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use crate::api::handlers::admin_user::request::admin_user_reset_password_request::AdminUserResetPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

#[derive(Serialize, Default)]
pub struct ForgotPasswordViewModel {
    pub link: String
}

pub async fn admin_user_reset_password_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AdminUserResetPasswordRequest>,
) -> Result<Json<ApiResponse<bool>>> {
    println!("->> {:<12} - admin_user_reset_password_api_handler", "HANDLER");

    let error_messages = payload.validate(&state).await?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };
        return Err(Error::BadRequest(error_response));
    }

    let password_hash = state
        .admin_user_service
        .get_password_hash_from_raw_password(payload.password, &state.config.password_salt)?;

    let update_password_status = state
        .admin_user_service
        .update_password_by_email(&state.db, password_hash, payload.email.clone())
        .await?;

    let mut response_data = ApiResponse {
        status: false,
        data: false
    };

    if update_password_status {
        let expired_status = state
            .admin_user_service
            .expire_password_token_by_email_and_token(&state.db, payload.email, payload.token)
            .await?;

        response_data = ApiResponse {
            status: true,
            data: expired_status
        };
    }

    Ok(Json(response_data))
}


#[derive(Serialize)]
pub struct ResponseData {
    status: bool
}