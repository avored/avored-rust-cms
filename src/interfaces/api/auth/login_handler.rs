use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rust_i18n::t;

use crate::{
    avored_state::AppState,
    core::{
        application::dtos::LoginCommand,
        domain::{
            entities::{ErrorMessageResponse, ErrorResponse},
            extensions::string_extension::StringExtension,
        },
    },
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

#[derive(Debug, serde::Serialize)]
pub struct LoginErrorResponse {
    pub message: String,
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
            Json(LoginErrorResponse {
                message: String::from("Invalid credentials"),
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

impl LoginCommand {
    fn validate(&self, locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.email.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("email"),
                message: t!("required", locale = locale, attribute = t!("email", locale = locale)).to_string(),
            });
            valid = false;
        }
        if !self.password.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("password"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("password", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(crate::error::Error::BadRequest(ErrorResponse {
                status: false,
                errors,
            }));
        }

        Ok(errors)
    }
}
