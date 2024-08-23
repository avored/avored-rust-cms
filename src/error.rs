use std::num::ParseIntError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use handlebars::{RenderError, TemplateError};
use lettre::address::AddressError;
use rust_i18n::t;
use serde::Serialize;
use tracing::log::error;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    ConfigMissing(String),

    Generic(String),

    CreateModelError(String),

    BadRequestError(ErrorResponse),

    AuthenticationError,

    Forbidden
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
//
// impl From<<T as TryFrom<Object>>::Error> for Error {
//     fn from(_val: surrealdb::err::Error) -> Self {
//         Error::Generic("Surreal Error".to_string())
//     }
// }

impl From<argon2::password_hash::Error> for Error {
    fn from(_val: argon2::password_hash::Error) -> Self {
        Error::Generic("Password hasher error".to_string())
    }
}
impl From<RenderError> for Error {
    fn from(actual_error: RenderError) -> Self {
        error!("there is an issue while rendering the handlebar template: {actual_error:?}");
        Error::Generic("handlebar error".to_string())
    }
}

// impl TryFrom<Object> for Error {
//     fn try_from(actual_error: Object) -> Self {
//         error!("there is an issue while rendering the handlebar template: {actual_error:?}");
//         Error::Generic("handlebar error".to_string())
//     }
// }

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
        let response = match self {
            Error::BadRequestError(str) => {
                (StatusCode::BAD_REQUEST, str).into_response()
            },
            Error::AuthenticationError => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: String::from(t!("email_password_not_matched"))
                };

                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors
                };
                (StatusCode::UNAUTHORIZED, error_response).into_response()
            },
            Error::Forbidden => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: String::from(t!("admin_user_forbidden"))
                };

                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors
                };
                (StatusCode::FORBIDDEN, error_response).into_response()
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "test 500").into_response()
        };

        // Create a placeholder Axum response.
        // let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        // let mut response = self {
        //
        // }
        // Insert the Error into the response.
        // response.extensions_mut().insert(response);

        response
    }
}
impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {

        let  validation_errors = match serde_json::to_string(&self) {
            Ok(str) => str,
            _ => "validation error 400.".to_string()
        };

        (StatusCode::BAD_REQUEST, validation_errors).into_response()
    }
}
