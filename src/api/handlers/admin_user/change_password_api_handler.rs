use crate::api::handlers::admin_user::request::change_password_request::ChangePasswordRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use crate::responses::ApiResponse;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::State;
use axum::{Extension, Json};
use rust_i18n::t;
use std::sync::Arc;

pub async fn change_password_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<bool>>> {
    println!("->> {:<12} - change_password_api_handler", "HANDLER");

    let mut error_messages = payload.validate()?;

    let is_password_match: bool = state.admin_user_service.compare_password(
        payload.current_password.clone(),
        logged_in_user.admin_user_model.password,
    )?;

    if !is_password_match {
        let error_message = ErrorMessage {
            key: String::from("password"),
            message: t!("password_match_error").to_string(),
        };
        error_messages.push(error_message);
    }

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
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
        .update_password_by_email(&state.db, password_hash, logged_in_user.email)
        .await?;

    let response_data = ApiResponse {
        status: true,
        data: update_password_status,
    };

    Ok(Json(response_data))
}
