use email_address::EmailAddress;
use rust_i18n::t;
use serde::Deserialize;
use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone)]
pub struct AdminUserResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub token: String
}

impl AdminUserResetPasswordRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.email.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string()
            };

            errors.push(error_message);
        }

        if ! EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string()
            };

            errors.push(error_message);
        }
        if self.password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string()
            };

            errors.push(error_message);
        }
        if self.confirm_password.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("validation_required", attribute = t!("confirm_password")).to_string()
            };

            errors.push(error_message);
        }
        if self.token.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("token"),
                message: t!("validation_required", attribute = t!("token")).to_string()
            };

            errors.push(error_message);
        }
        Ok(errors)
    }
}