use serde::Deserialize;
use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub password: String,
    pub confirm_password: String,
}

impl ChangePasswordRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.current_password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("current_password"),
                message: String::from("Current Password is a required field")
            };

            errors.push(error_message);
        }

        if self.password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: String::from("Password is a required field")
            };

            errors.push(error_message);
        }

        if self.confirm_password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: String::from("Confirm Password is a required field")
            };

            errors.push(error_message);
        }

        if self.password != self.confirm_password {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: String::from("Confirm Password is not same as new password")
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}