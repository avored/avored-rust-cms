use email_address::EmailAddress;
use serde::Deserialize;
use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone)]
pub struct AuthenticateAdminUserRequest {
    pub email: String,
    pub password: String,
}

impl AuthenticateAdminUserRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.email.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Email is a required field")
            };

            errors.push(error_message);
        }

        if ! EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: String::from("Invalid email address")
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

        Ok(errors)
    }
}