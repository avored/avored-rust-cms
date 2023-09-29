use crate::{error::Result, providers::avored_session_provider::AvoRedSession};
use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct AuthenticateAdminUserRequest {
    #[validate(email(message = "The email field must be a valid email address."))]
    pub email: String,
    #[validate(length(min = 1, message = "The password is a required field."))]
    pub password: String,
}

impl AuthenticateAdminUserRequest {
    pub fn validate_errors(&self, mut session: AvoRedSession) -> Result<ValidationErrors> {
        let validation_error_list = match self.validate() {
            Ok(_) => ValidationErrors::new(),
            Err(errors) => errors,
        };

        for (field_name, error) in validation_error_list.errors() {
            match &error {
                ValidationErrorsKind::Field(field_errors) => {
                    for field_error in field_errors {
                        let message = match &field_error.message {
                            Some(message) => message,
                            None => continue,
                        };

                        if !message.is_empty() {
                            // let key = field_name.clone();
                            let validation_key = format!("validation_error_{}", field_name);
                            session
                                .insert(&validation_key, message)
                                .expect("Could not store the validation errors into session.");
                        }
                    }
                }
                ValidationErrorsKind::Struct(_) => continue,
                ValidationErrorsKind::List(_) => continue,
            }
        }

        Ok(validation_error_list)
    }
}
