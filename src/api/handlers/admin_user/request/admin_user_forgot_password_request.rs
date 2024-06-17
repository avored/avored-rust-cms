use rust_i18n::t;
use serde::Deserialize;
use crate::error::Result;
use crate::models::validation_error::ErrorMessage;
use crate::models::validation_error::Validate;

#[derive(Deserialize, Debug, Clone)]
pub struct AdminUserForgotPasswordRequest {
    pub email: String,
}

impl AdminUserForgotPasswordRequest {
    pub fn validate(&self) -> Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.email.required()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string()
            };

            errors.push(error_message);
        }

        if !self.email.validate_email()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string()
            };

            errors.push(error_message);
        }
        Ok(errors)
    }
}
