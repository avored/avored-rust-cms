use rust_i18n::t;
use serde::Deserialize;

use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StoreRoleRequest {
    pub name: String,
    pub identifier: String,
    pub permissions: Vec<String>,
}

impl StoreRoleRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.name.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string()
            };

            errors.push(error_message);
        }

        if self.identifier.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: String::from(format!("Identifier is a required field {}", t!("identifier")))
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
