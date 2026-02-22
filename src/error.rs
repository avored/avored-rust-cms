use leptos::config::errors::LeptosConfigError;
use tracing::error;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String)
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

//LeptosConfigError
