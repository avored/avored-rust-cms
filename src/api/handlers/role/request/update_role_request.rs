use serde::Deserialize;

use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub identifier: String,
    pub permissions: Vec<String>,
}

impl UpdateRoleRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if self.name.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: String::from("Name is a required field")
            };

            errors.push(error_message);
        }

        if self.identifier.len() <= 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: String::from("Identifier is a required field")
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
