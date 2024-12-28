use rust_i18n::t;
use serde::Deserialize;

use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StoreModelRequest {
    pub name: String,
    pub identifier: String,
}

impl StoreModelRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };

            errors.push(error_message);
        }

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: format!("Identifier is a required field {}", t!("identifier")),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
