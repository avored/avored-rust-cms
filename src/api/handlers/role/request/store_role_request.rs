use rust_i18n::t;
use serde::Deserialize;

use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StoreRoleRequest {
    pub name: String,
    pub identifier: String,
    pub permissions: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Permission {
    pub dashboard: Option<bool>,
    pub admin_user_table: Option<bool>,
    pub admin_user_update: Option<bool>,
    pub admin_user_create: Option<bool>,
    pub role_table: Option<bool>,
    pub role_create: Option<bool>,
    pub role_update: Option<bool>,
    pub role_delete: Option<bool>,
}

impl StoreRoleRequest {
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
                message: String::from(format!("Identifier is a required field {}", t!("name")))
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
