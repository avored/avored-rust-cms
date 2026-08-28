/// This is custom Result type for the application.
pub type Result<T> = core::result::Result<T, Error>;

/// This is the custom error type for the application.
#[derive(Debug, Clone)]
pub enum Error {
    Generic(String),
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
