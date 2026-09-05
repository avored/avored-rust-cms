use tracing::error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::core::domain::entities::error_message::ErrorResponse;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    ConfigMissing(String),
    BadRequest(ErrorResponse),
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
        Self::Generic(format!("there is an issue with password hashing"))
    }
}


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(str) => (StatusCode::BAD_REQUEST, str).into_response(),
            err => (StatusCode::INTERNAL_SERVER_ERROR, format!("error 500: {:?}", err)).into_response(),
        }
    }
}


impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let validation_errors = match serde_json::to_string(&self) {
            Ok(str) => str,
            _ => "validation error 400.".to_string(),
        };

        (StatusCode::BAD_REQUEST, validation_errors).into_response()
    }
}
