use axum::{extract::State, Extension, Json};

use crate::avored_state::AppState;
use crate::core::application::dtos::{
    AdminProfileResponse, ChangePasswordCommand, UpdateProfileCommand,
};
use crate::core::domain::entities::user::TokenClaims;
use crate::error::Result;

pub async fn get_profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<TokenClaims>,
) -> Result<Json<AdminProfileResponse>> {
    let profile = state.auth_use_case.fetch_profile(&claims.sub).await?;
    Ok(Json(profile))
}

pub async fn update_profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<UpdateProfileCommand>,
) -> Result<Json<AdminProfileResponse>> {
    let profile = state
        .auth_use_case
        .update_profile(&claims.sub, payload)
        .await?;
    Ok(Json(profile))
}

#[derive(serde::Serialize)]
pub struct PasswordChangeResponse {
    pub success: bool,
    pub message: String,
}

pub async fn change_password_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<TokenClaims>,
    Json(payload): Json<ChangePasswordCommand>,
) -> Result<Json<PasswordChangeResponse>> {
    let salt = &state.config.password_salt;
    state
        .auth_use_case
        .change_password(&claims.sub, payload, salt)
        .await?;

    Ok(Json(PasswordChangeResponse {
        success: true,
        message: "Password updated successfully".to_string(),
    }))
}
