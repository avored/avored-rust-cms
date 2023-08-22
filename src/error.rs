use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum AvoRedError {
	#[error("Generic {0}")]
	Generic(String),
}

impl IntoResponse for AvoRedError {
	fn into_response(self) -> Response {
		println!("->> {:<12} -  {self:?}", "INTO_RESPONSE");

		(StatusCode::INTERNAL_SERVER_ERROR, "Unhandled error").into_response()
	}
}

#[derive(Debug)]
pub enum Error {
	CtxFail,

	XValueNotOfType(&'static str),

	XPropertyNotFound(String),

	StoreFailToCreate(String),

	JsonSerde(serde_json::Error),

	ModqlOperatorNotSupported(String),

	Surreal(surrealdb::err::Error),

	IO(std::io::Error),
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Error::JsonSerde(val)
	}
}
impl From<surrealdb::err::Error> for Error {
	fn from(val: surrealdb::err::Error) -> Self {
		Error::Surreal(val)
	}
}
impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Error::IO(val)
	}
}
// endregion: --- Froms

// region:    --- Error Boiler
impl std::fmt::Display for Error {
	fn fmt(&self, fmt: &mut std::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

// endregion: --- Error Boiler
