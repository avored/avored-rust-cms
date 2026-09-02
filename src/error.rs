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
        Self::Generic("there is an issue with surreal db".to_string())
    }
}
 


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(str) => (StatusCode::BAD_REQUEST, str).into_response(),
            // Self::Unauthorizeed(resource_name) => {
                // let error_message = format!("unauthorized: you do not have access to access this ({resource_name}) resource");
                // (StatusCode::UNAUTHORIZED, error_message).into_response()
            // }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "test 500").into_response(),
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
