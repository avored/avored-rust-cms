use leptos::{config::errors::LeptosConfigError};
use tonic::Status;
use tracing::error;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),

    /// Error when the password encryption has some issue.
    Argon2(Box<argon2::password_hash::Error>),

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
            Error::Argon2(boxed_error) => {
                Self::internal(format!("Argon2 error: {boxed_error:?}"))
            },
            _ => Self::invalid_argument("500 Internal server error")
        }
    }
}


impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        error!("IO error: {error:?}");
        Self::Generic(format!("IO error: {error}"))
    }
}

//LeptosConfigError
