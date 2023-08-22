//! Crate AvoRed Prelude

pub use crate::error::AvoRedError;

pub type AvoRedResult<T> = core::result::Result<T, AvoRedError>;

pub struct W<T>(pub T);