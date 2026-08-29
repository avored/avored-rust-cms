use tracing::error;

/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
    ConfigMissing(String),
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
    
