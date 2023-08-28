//! AvoRed QS gives support to deserialize the query string style string and forms for axum.
//!
//! Querystrings are not formally defined and loosely take the form of
//! _nested_ urlencoded queries.
//!
//! This library aims for compatability with the syntax of
//! [qs](https://github.com/ljharb/qs) and also of the
//! [`Rack::Utils::parse_nested_query`](http://www.rubydoc.info/github/rack/rack/Rack/Utils#parse_nested_query-class_method)
//! implementation.
//!
//! For users who do *not* require nested URL parameters, it is highly
//! recommended that the `serde_urlencoded` crate is used instead, which
//! will almost certainly perform better for deserializing simple inputs.
//!
//! ## Supported Types
//!
//! At the **top level**, `serde_qs` only supports `struct`, `map`, and `enum`.
//! These are the only top-level structs which can be de/serialized since
//! Querystrings rely on having a (key, value) pair for each field, which
//! necessitates this kind of structure.
//!
//! However, after the top level you should find all supported types can be
//! de/serialized.
//!
//! Note that integer keys are reserved for array indices. That is, a string of
//! the form `a[0]=1&a[1]=3` will deserialize to the ordered sequence `a =
//! [1,3]`.
//!
//! ## Usage
//!
//! ```
//! #[macro_use]
//! extern crate serde_derive;
//! extern crate serde_qs as qs;
//!
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct Address {
//!     city: String,
//!     postcode: String,
//! }
//! #[derive(Debug, PartialEq, Deserialize, Serialize)]
//! struct QueryParams {
//!     id: u8,
//!     name: String,
//!     address: Address,
//!     phone: u32,
//!     user_ids: Vec<u8>,
//! }
//!
//! # fn main() {
//! let params = QueryParams {
//!     id: 42,
//!     name: "Acme".to_string(),
//!     phone: 12345,
//!     address: Address {
//!         city: "Carrot City".to_string(),
//!         postcode: "12345".to_string(),
//!     },
//!     user_ids: vec![1, 2, 3, 4],
//! };
//! let rec_params: QueryParams = qs::from_str("\
//!     name=Acme&id=42&phone=12345&address[postcode]=12345&\
//!     address[city]=Carrot+City&user_ids[0]=1&user_ids[1]=2&\
//!     user_ids[2]=3&user_ids[3]=4")
//!     .unwrap();
//! assert_eq!(rec_params, params);
//!
//! # }
//! ```
//!
//! ## Strict vs Non-Strict modes
//!
//! `serde_qs` supports two operating modes, which can be specified using
//! [`Config`](struct.Config.html).
//! Strict mode has two parts:
//! - how `serde_qs` handles square brackets
//! - how `serde_qs` handles invalid UTF-8 percent decoded characters
//!
//! ### Square Brackets
//!
//! Technically, square brackets should be encoded in URLs as `%5B` and `%5D`.
//! However, they are often used in their raw format to specify querystrings
//! such as `a[b]=123`.
//!
//! In strict mode, `serde_qs` will only tolerate unencoded square brackets
//! to denote nested keys. So `a[b]=123` will decode as `{"a": {"b": 123}}`.
//! This means that encoded square brackets can actually be part of the key.
//! `a[b%5Bc%5D]=123` becomes `{"a": {"b[c]": 123}}`.
//!
//! However, since some implementations will automatically encode everything
//! in the URL, we also have a non-strict mode. This means that `serde_qs`
//! will assume that any encoded square brackets in the string were meant to
//! be taken as nested keys. From the example before, `a[b%5Bc%5D]=123` will
//! now become `{"a": {"b": {"c": 123 }}}`.
//!
//! Non-strict mode can be useful when, as said before, some middleware
//! automatically encodes the brackets. But care must be taken to avoid
//! using keys with square brackets in them, or unexpected things can
//! happen.
//!
//! ### Invalid UTF-8 Percent Encodings
//!
//! Sometimes querystrings may have percent-encoded data which does not decode
//! to UTF-8. In some cases it is useful for this to cause errors, which is how
//! `serde_qs` works in strict mode (the default). Whereas in other cases it
//! can be useful to just replace such data with the unicode replacement
//! character (� `U+FFFD`), which is how `serde_qs` works in non-strict mode.
//!
//! ## Flatten workaround
//!
//! A current [known limitation](https://github.com/serde-rs/serde/issues/1183)
//! in `serde` is deserializing `#[serde(flatten)]` structs for formats which
//! are not self-describing. This includes query strings: `12` can be an integer
//! or a string, for example.
//!
//! 
//! 
//! 
use axum::{extract::{FromRequest, RawForm}, BoxError, async_trait, body::HttpBody, http::{Request, StatusCode}, response::{Response, IntoResponse}, RequestExt};
use serde::de::DeserializeOwned;
use urlencoding::decode_binary;

#[derive(Debug, Clone, Copy, Default)]
pub struct AvoRedForm<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for AvoRedForm<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = AvoRedError;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {

        match req.extract().await {
            Ok(RawForm(bytes)) => {

                let decoded = decode_binary(&bytes).into_owned();
                let decoded = String::from_utf8_lossy(&decoded).into_owned();
                println!("--> avored qs: {}", decoded);
                match serde_qs::from_str(&decoded) {
                    Ok(value) => Ok(AvoRedForm(value)),
                    Err(_) => Err(AvoRedError::ParseFormError)
                }
            }
            Err(_) => Err(AvoRedError::ParseFormError),
        }
    }
}


pub enum AvoRedError {
    ParseFormError,
}


impl IntoResponse for AvoRedError {
    fn into_response(self) -> Response {
        let status = StatusCode::UNPROCESSABLE_ENTITY;

        let body = "there is an issue while parsing the form data. It might be an syntaxt issue";

        (status, body).into_response()
    }
}
