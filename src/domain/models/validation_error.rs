use email_address::EmailAddress;
use serde::Serialize;


/// Represents an error message with a key and a message
#[derive(Debug, Serialize, Clone)]
pub struct ErrorMessage {

    /// Key associated with the error message
    pub key: String,
    
    /// The error message itself
    pub message: String,
}

/// Represents a response containing errors
#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {

    /// Indicates whether the validation was successful
    pub status: bool,

    /// A list of error messages
    pub errors: Vec<ErrorMessage>,
}

/// Trait for validating input data
pub trait Validate {

    /// Checks if the input is required
    fn required(&self) -> crate::error::Result<bool>;

    /// Validates if the input is a valid email address
    fn validate_email(&self) -> crate::error::Result<bool>;
}

impl Validate for String {
    fn required(&self) -> crate::error::Result<bool> {
        if !self.is_empty() {
            return Ok(true);
        }
        Ok(false)
    }

    fn validate_email(&self) -> crate::error::Result<bool> {
        if !EmailAddress::is_valid(self) {
            return Ok(false);
        }
        Ok(true)
    }
}
