use std::net::AddrParseError;
use std::num::ParseIntError;
use tonic::Status;
use tracing::error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    ConfigMissing(String),
    TonicError(Status),

    Argon2Error(argon2::password_hash::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::TonicError(status)
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
        error!("there is with tonic status: {val:?}");
        Self::invalid_argument("500 Internal server error")
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
