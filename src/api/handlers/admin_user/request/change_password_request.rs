use rust_i18n::t;
use serde::Deserialize;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub password: String,
    pub confirm_password: String,
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.current_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("current_password"),
                message: t!("validation_required", attribute = t!("current_password")).to_string()
            };

            errors.push(error_message);
        }

        if !self.password.required()? {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string()
            };

            errors.push(error_message);
        }

        if !self.confirm_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("validation_required", attribute = t!("confirm_password")).to_string()
            };

            errors.push(error_message);
        }

        if self.password != self.confirm_password {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("current_not_same_as_new_password").to_string()
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}