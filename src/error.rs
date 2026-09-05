use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use leptos::config::errors::LeptosConfigError;
use tonic::Status;
use tracing::error;

use crate::domain::models::validation_error::ErrorResponse;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),

    /// Error when the password encryption has some issue.
    Argon2(Box<argon2::password_hash::Error>),

    /// Error when the request is bad.
    BadRequest(ErrorResponse),

    /// Error when the request is forbidden.
    InvalidArgument(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<LeptosConfigError> for Error {
    fn from(val: LeptosConfigError) -> Self {
        error!("there is an issue with creating io error: {val:?}");
        Self::Generic("tokio file create folder error ".to_string())
    }
}

impl From<surrealdb::Error> for Error {
    fn from(actual_error: surrealdb::Error) -> Self {
        error!("Surreal DB error: {actual_error:?}");
        Self::Generic("500 internal".to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(actual_error: argon2::password_hash::Error) -> Self {
        error!("argon2 password hash error: {actual_error:?}");
        Self::Argon2(Box::new(actual_error))
    }
}

impl From<dotenvy::Error> for Error {
    fn from(val: dotenvy::Error) -> Self {
        error!("there is an issue with loading env file: {val:?}");
        Self::Generic("there is an issue with loading env file".to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        error!("there is an issue with JSON Web token: {val:?}");
        Self::Generic("there is an issue with JSON Web token".to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        error!("IO error: {error:?}");
        Self::Generic(format!("IO error: {error}"))
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        error!("serde_json error: {error:?}");
        Self::Generic(format!("serde_json error: {error}"))
    }
}

// 


//

impl From<Error> for Status {
    fn from(val: Error) -> Self {
        match val {
            // Error::InvalidArgument(error_response) => Self::invalid_argument(error_response),
            // Error::Unauthorizeed(resource_name) => {
            //     let error_message = format!("unauthorized: you do not have access to access this ({resource_name}) resource");
            //     Self::permission_denied(error_message)
            // }
            // Error::Unauthenticated(error_message) => {
            //     Self::unauthenticated(error_message)
            // },
            Error::Argon2(boxed_error) => Self::internal(format!("Argon2 error: {boxed_error:?}")),
            Error::InvalidArgument(error_response) => Self::invalid_argument(error_response),
            Error::BadRequest(str) => {
                let validation_errors = match serde_json::to_string(&str) {
                    Ok(str) => str,
                    _ => "validation error 400.".to_string(),
                };

                Self::invalid_argument(validation_errors)
            }
            _ => Self::internal("500 Internal server error"),
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

//LeptosConfigError
