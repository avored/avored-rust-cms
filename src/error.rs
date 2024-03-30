use std::sync::Arc;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    ConfigMissing(&'static str),

    Generic(&'static str),

    CreateModelError(&'static str),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<serde_json::Error> for Error {
    fn from(_val: serde_json::Error) -> Self {
        Error::Generic("Serde struct to string  Error")
    }
}

impl From<surrealdb::err::Error> for Error {
    fn from(_val: surrealdb::err::Error) -> Self {
        Error::Generic("Surreal Error")
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        // Create a placeholder Axum response.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the response.
        response.extensions_mut().insert(self);

        response
    }
}
