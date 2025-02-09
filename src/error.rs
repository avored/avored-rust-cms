use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use axum::extract::multipart::MultipartError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use handlebars::{RenderError, TemplateError};
use lettre::address::AddressError;
use rust_i18n::t;
use serde::Serialize;
use std::num::ParseIntError;
use tracing::log::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    ConfigMissing(String),

    Generic(String),

    CreateModel(String),

    ModelNotFound(String),

    BadRequest(ErrorResponse),

    Authentication(String),

    NotFound(String),

    Forbidden,
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        error!("there is an issue with serde json error: {val:?}");
        Error::Generic("Serde struct to string  Error".to_string())
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(val: jsonwebtoken::errors::Error) -> Self {
        error!("there is an issue with jsonwebtoken error: {val:?}");
        Error::Generic("Json web token error".to_string())
    }
}

impl From<MultipartError> for Error {
    fn from(val: MultipartError) -> Self {
        error!("there is an issue with multipart error: {val:?}");
        Error::Generic("multipart can not find next field".to_string())
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

impl From<surrealdb::err::Error> for Error {
    fn from(val: surrealdb::err::Error) -> Self {
        error!("there is an issue with surreal db: {val:?}");
        Error::Generic("Surreal Error".to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(val: argon2::password_hash::Error) -> Self {
        error!("there is an issue while encrypting password with argon2: {val:?}");
        Error::Generic("Password hasher error".to_string())
    }
}
impl From<RenderError> for Error {
    fn from(actual_error: RenderError) -> Self {
        error!("there is an issue while rendering the handlebar template: {actual_error:?}");
        Error::Generic("handlebar error".to_string())
    }
}

impl From<TemplateError> for Error {
    fn from(actual_error: TemplateError) -> Self {
        error!("there is an issue while registering the handlebar template with avored: {actual_error:?}");
        Error::Generic("handlebar template error".to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(actual_error: ParseIntError) -> Self {
        error!("there is an issue while parsing the env from string to u16: {actual_error:?}");
        Error::Generic("parse int error".to_string())
    }
}

impl From<AddressError> for Error {
    fn from(actual_error: AddressError) -> Self {
        error!("there is an issue while parsing email address: {actual_error:?}");
        Error::Generic("parse lettre address parsing error".to_string())
    }
}

impl From<lettre::error::Error> for Error {
    fn from(actual_error: lettre::error::Error) -> Self {
        error!("there is an issue lettre error: {actual_error:?}");
        Error::Generic("lettre error".to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {

        match self {
            Error::BadRequest(str) => (StatusCode::BAD_REQUEST, str).into_response(),
            Error::Authentication(_e) => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: String::from(t!("email_password_not_matched")),
                };

                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors,
                };
                (StatusCode::UNAUTHORIZED, error_response).into_response()
            }
            Error::Forbidden => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: String::from(t!("admin_user_forbidden")),
                };

                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors,
                };
                (StatusCode::FORBIDDEN, error_response).into_response()
            }
            Error::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
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
