use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use crate::api::handlers::admin_user::request::admin_user_reset_password_request::AdminUserResetPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

#[derive(Serialize, Default)]
pub struct ForgotPasswordViewModel {
    pub link: String
}

pub async fn admin_user_reset_password_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AdminUserResetPasswordRequest>,
) -> Result<Json<ResponseData>> {
    println!("->> {:<12} - admin_user_reset_password_api_handler", "HANDLER");

    let mut error_messages = payload.validate()?;

    //@tood move this token validation logic to validate method.
    let password_reset_model = state
        .admin_user_service
        .get_password_reset_by_email(&state.db, payload.email.clone())
        .await?;

    if password_reset_model.token != payload.token {
        let error_message = ErrorMessage {
            key: String::from("token"),
            message: String::from("token did not match please try again")
        };
        error_messages.push(error_message);
    }

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };
        return Err(Error::BadRequestError(error_response));
    }


    let password = payload.password.as_bytes();
    let salt = SaltString::from_b64(&state.config.password_salt)?;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let update_password_status = state
        .admin_user_service
        .update_password_by_email(&state.db, password_hash, payload.email)
        .await?;


    let response_data = ResponseData {
        status: update_password_status
    };

    Ok(Json(response_data))
}


#[derive(Serialize)]
pub struct ResponseData {
    status: bool
}