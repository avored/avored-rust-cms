use crate::api::admin_user::requests::authenticate_admin_user_request::AuthenticateAdminUserRequest;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::{avored_state::AvoRedState, error::Result};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use std::sync::Arc;
use validator::HasLen;

pub async fn authenticate_admin_user_handler(
    mut session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<AuthenticateAdminUserRequest>,
) -> Result<impl IntoResponse> {
    let admin_user_model = state
        .admin_user_service
        .find_by_email(&state.db, payload.email.to_owned())
        .await?;

    let validation_error_list = payload.validate_errors(session.clone())?;

    if validation_error_list.errors().length() > 0 {
        return Ok(Redirect::to("/admin/login").into_response());
    }

    let is_valid = match PasswordHash::new(&admin_user_model.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let validation_error = String::from("your email address or password did not match");

        session
            .insert("validation_error_email", validation_error)
            .expect("Could not store the validation errors into session.");

        return Ok(Redirect::to("/admin/login").into_response());
    }

    session
        .insert("logged_in_user", admin_user_model)
        .expect("Could not store the answer.");

    Ok(Redirect::to("/admin").into_response())
}
