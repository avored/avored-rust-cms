use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    avored_state::AppState,
    core::{application::dtos::LoginCommand, domain::entities::ApiErrorMessageResponse},
    error::Result,
};

#[derive(Debug, serde::Serialize)]
pub struct LoginUserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

#[derive(Debug, serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: LoginUserResponse,
    pub authenticated: bool,
}

#[axum::debug_handler(state = AppState)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginCommand>,
) -> Result<axum::response::Response> {
    let locale = "en";
    payload.validate(locale)?;

    let jwt = state.config.jwt_secret_key.clone();

    let result = state.auth_use_case.auth(payload, jwt).await?;

    if !result.authenticated {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(ApiErrorMessageResponse {
                status: false,
                message: String::from("Invalid credentials"),
                request_id: None,
            }),
        )
            .into_response());
    }

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            token: result.token,
            user: LoginUserResponse {
                id: result.user.id,
                name: result.user.name,
                email: result.user.email,
            },
            authenticated: true,
        }),
    )
        .into_response())
}
