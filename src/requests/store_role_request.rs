use serde::Deserialize;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

use super::ValidateRequest;
use crate::providers::avored_session_provider::AvoRedSession;

#[derive(Deserialize, Debug, Clone, Validate)]
pub struct StoreRoleRequest {
    #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,
    #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,
}

impl ValidateRequest for StoreRoleRequest {
    fn validation_error(&self, session: &mut AvoRedSession) -> ValidationErrors {
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

                        println!("{:?}", field_error);

                        if !message.is_empty() {
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

        validation_error_list
    }
}
