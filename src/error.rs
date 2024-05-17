use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use crate::models::validation_error::ErrorResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize)]
pub enum Error {
    ConfigMissing(String),

    Generic(String),

    CreateModelError(String),

    BadRequestError(ErrorResponse),

    AuthenticationError
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(_val: serde_json::Error) -> Self {
        Error::Generic("Serde struct to string  Error".to_string())
    }
}

impl From<surrealdb::err::Error> for Error {
    fn from(_val: surrealdb::err::Error) -> Self {
        Error::Generic("Surreal Error".to_string())
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(_val: argon2::password_hash::Error) -> Self {
        Error::Generic("Password hasher error".to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // println!("->> {:<12} - {self:?}", "INTO_RES");
        let response = match self {
            Error::BadRequestError(str) => {
                // let tets = serde_json::to_string(&str);

                (StatusCode::BAD_REQUEST, str).into_response()
            },
            Error::AuthenticationError => {
                (StatusCode::UNAUTHORIZED, "Invalid Login detail given.").into_response()
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
