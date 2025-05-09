use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct ErrorMessage {
    pub key: String,
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub status: bool,
    pub errors: Vec<ErrorMessage>,
}

pub trait Validate {
    fn required(&self) -> crate::error::Result<bool>;

    // fn validate_email(&self) -> crate::error::Result<bool>;
}

impl Validate for String {
    fn required(&self) -> crate::error::Result<bool> {
        if !self.is_empty() {
            return Ok(true);
        }
        Ok(false)
    }
    
    // fn validate_email(&self) -> crate::error::Result<bool> {
    //     if !EmailAddress::is_valid(self) {
    //         return Ok(false);
    //     }
    //     Ok(true)
    // }
}
