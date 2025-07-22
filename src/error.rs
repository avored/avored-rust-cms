use crate::models::validation_error::ErrorResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use handlebars::{RenderError, TemplateError};
use lettre::address::AddressError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use tonic::Status;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    ConfigMissing(String),
    Tonic(Box<Status>),
    BadRequest(ErrorResponse),
    Unauthorizeed(String),
    Unauthenticated(String),
    InvalidArgument(String),
    Argon2(Box<argon2::password_hash::Error>),
    LdapConnectionError(String),
    LdapAuthenticationError(String),
    LdapSearchError(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::Tonic(Box::new(status))
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Generic(s)
    }
}

impl From<dotenvy::Error> for Error {
    fn from(val: dotenvy::Error) -> Self {
        error!("there is an issue with loading env file: {val:?}");
        Error::Generic("there is an issue with loading env file".to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        error!("there is an issue with creating io error: {val:?}");
        Error::Generic("tokio file create folder error ".to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("serde json error: {val:?}");
        Error::Generic("serde json error".to_string())
    }
}

impl From<Error> for Status {
    fn from(val: Error) -> Self {
        match val {
            Error::Tonic(boxed_status) => *boxed_status,
            Error::InvalidArgument(error_response) => {
                Self::invalid_argument(error_response)
            }
            Error::Unauthorizeed(resource_name) => {
                let error_message = format!("unauthorized: you do not have access to access this ({resource_name}) resource");
                Self::permission_denied(error_message)
            },
            Error::Unauthenticated(error_message) => {
                Self::unauthenticated(error_message)
            },
            Error::Argon2(boxed_error) => {
                Self::internal(format!("Argon2 error: {boxed_error:?}"))
            },
            _ => Self::invalid_argument("500 Internal server error")
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(actual_error: ParseIntError) -> Self {
        error!("there is an issue while parsing the env from string to u16: {actual_error:?}");
        Error::Generic("parse int error".to_string())
    }
}

impl From<AddrParseError> for Error {
    fn from(actual_error: AddrParseError) -> Self {
        error!("there is an issue while parsing email address: {actual_error:?}");
        Error::Generic("500 internal".to_string())
    }
}

impl From<tonic::transport::Error> for Error {
    fn from(actual_error: tonic::transport::Error) -> Self {
        error!("there is an issue while parsing email address: {actual_error:?}");
        Error::Generic("500 internal".to_string())
    }
}

impl From<surrealdb::error::Db> for Error {
    fn from(actual_error: surrealdb::error::Db) -> Self {
        error!("Surreal DB error: {actual_error:?}");
        Error::Generic("500 internal".to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(actual_error: jsonwebtoken::errors::Error) -> Self {
        error!("Json web token error: {actual_error:?}");
        Error::Generic("500 internal".to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(actual_error: argon2::password_hash::Error) -> Self {
        error!("argon2 password hash error: {actual_error:?}");
        Error::Argon2(Box::new(actual_error))
    }
}

impl From<lettre::error::Error> for Error {
    fn from(actual_error: lettre::error::Error) -> Self {
        error!("there is an issue lettre error: {actual_error:?}");
        Error::Generic("lettre error".to_string())
    }
}

impl From<TemplateError> for Error {
    fn from(actual_error: TemplateError) -> Self {
        error!("there is an issue while registering the handlebar template with avored: {actual_error:?}");
        Error::Generic("handlebar template error".to_string())
    }
}

impl From<RenderError> for Error {
    fn from(actual_error: RenderError) -> Self {
        error!("there is an issue while rendering the handlebar template: {actual_error:?}");
        Error::Generic("handlebar error".to_string())
    }
}

impl From<AddressError> for Error {
    fn from(actual_error: AddressError) -> Self {
        error!("there is an issue while parsing email address: {actual_error:?}");
        Error::Generic("parse lettre address parsing error".to_string())
    }
}

impl From<ldap3::LdapError> for Error {
    fn from(actual_error: ldap3::LdapError) -> Self {
        error!("LDAP error: {actual_error:?}");
        Error::Generic("LDAP operation failed".to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::BadRequest(str) => (StatusCode::BAD_REQUEST, str).into_response(),
            Error::Unauthorizeed(resource_name) => {
                let error_message = format!("unauthorized: you do not have access to access this ({resource_name}) resource");
                (StatusCode::UNAUTHORIZED, error_message).into_response()
            }
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
