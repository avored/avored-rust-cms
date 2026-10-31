use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use rust_i18n::t;
use tracing::error;

use crate::core::domain::entities::error_message::{ApiErrorMessageResponse, ApiErrorResponse};
use crate::core::domain::entities::ErrorMessageResponse;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

pub fn unauthorized(message: impl Into<String>) -> (StatusCode, Json<ApiErrorMessageResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ApiErrorMessageResponse {
            status: false,
            message: message.into(),
        }),
    )
}

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    ConfigMissing(String),
    BadRequest(ApiErrorResponse),
    Authentication,
    Forbidden(String),
    NotFound(String),
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<dotenvy::Error> for Error {
    fn from(val: dotenvy::Error) -> Self {
        error!("there is an issue with loading env file: {val:?}");
        Self::Generic("there is an issue with loading env file".to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("there is an issue with parsing json: {val:?}");
        Self::Generic("there is an issue with parsing json".to_string())
    }
}

impl From<surrealdb::Error> for Error {
    fn from(val: surrealdb::Error) -> Self {
        error!("there is an issue with surreal db: {val:?}");
        Self::Generic(format!("there is an issue with surreal db: {val:?}"))
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(actual_error: argon2::password_hash::Error) -> Self {
        error!("argon2 password hash error: {actual_error:?}");
        Self::Generic("there is an issue with password hashing".to_string())
    }
}

impl From<argon2::password_hash::phc::Error> for Error {
    fn from(actual_error: argon2::password_hash::phc::Error) -> Self {
        error!("argon2 password hash error: {actual_error:?}");
        Self::Generic("there is an issue with password hashing".to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(actual_error: jsonwebtoken::errors::Error) -> Self {
        error!("jsonwebtoken error: {actual_error:?}");
        Self::Generic("there is an issue with generating jwt token".to_string())
    }
}

impl From<leptos::config::errors::LeptosConfigError> for Error {
    fn from(actual_error: leptos::config::errors::LeptosConfigError) -> Self {
        error!("leptos config error: {actual_error:?}");
        Self::Generic("there is an issue with leptos config".to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(actual_error: std::io::Error) -> Self {
        error!("IO error: {actual_error:?}");
        Self::Generic("there is an issue with IO".to_string())
    }
}

impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(actual_error: reqwest::header::InvalidHeaderValue) -> Self {
        error!("Invalid header value error: {actual_error:?}");
        Self::Generic("there is an issue with the header value".to_string())
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(actual_error: std::net::AddrParseError) -> Self {
        error!("Address parse error: {actual_error:?}");
        Self::Generic("there is an issue with the address".to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(response) => (StatusCode::BAD_REQUEST, Json(response)).into_response(),
            Self::NotFound(message) => (
                StatusCode::NOT_FOUND,
                Json(ApiErrorMessageResponse {
                    status: false,
                    message,
                }),
            )
                .into_response(),
            Self::Forbidden(message) => (
                StatusCode::FORBIDDEN,
                Json(ApiErrorMessageResponse {
                    status: false,
                    message,
                }),
            )
                .into_response(),
            Error::Authentication => {
                let mut errors: Vec<ErrorMessageResponse> = vec![];
                let error_message = ErrorMessageResponse {
                    key: String::from("email"),
                    message: String::from(t!("email_password_not_matched")),
                };

                errors.push(error_message);
                let error_response = ApiErrorResponse {
                    status: false,
                    errors,
                };
                (StatusCode::UNAUTHORIZED, Json(error_response)).into_response()
            }
            err => {
                error!(error = ?err, "internal request error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiErrorMessageResponse {
                        status: false,
                        message: "An internal server error occurred.".to_string(),
                    }),
                )
                    .into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    async fn response_json(response: Response) -> serde_json::Value {
        let body = to_bytes(response.into_body(), usize::MAX)
            .await
            .expect("response body should be readable");
        serde_json::from_slice(&body).expect("response body should be valid JSON")
    }

    #[tokio::test]
    async fn bad_request_returns_structured_json() {
        let response = Error::BadRequest(ApiErrorResponse {
            status: false,
            errors: vec![ErrorMessageResponse {
                key: "email".to_string(),
                message: "Required".to_string(),
            }],
        })
        .into_response();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        assert_eq!(
            response_json(response).await,
            serde_json::json!({
                "status": false,
                "errors": [{"key": "email", "message": "Required"}]
            })
        );
    }

    #[tokio::test]
    async fn forbidden_returns_message_json() {
        let response = Error::Forbidden("Not allowed".to_string()).into_response();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            response_json(response).await,
            serde_json::json!({"status": false, "message": "Not allowed"})
        );
    }

    #[tokio::test]
    async fn internal_error_hides_debug_details() {
        let response = Error::Generic("sensitive internal details".to_string()).into_response();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let body = response_json(response).await;
        assert_eq!(
            body,
            serde_json::json!({
                "status": false,
                "message": "An internal server error occurred."
            })
        );
        assert!(!body.to_string().contains("sensitive internal details"));
    }
}
