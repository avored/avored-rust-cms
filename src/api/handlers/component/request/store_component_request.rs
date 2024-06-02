use serde::Deserialize;
use rust_i18n::t;
use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone)]
pub struct StoreComponentRequest {
    pub name: String,
    pub identifier: String,
    pub fields: Vec<FieldRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FieldRequest {
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_data: Option<Vec<FieldDataRequest>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FieldDataRequest {
    pub label: String,
    pub value: String,
}

impl StoreComponentRequest {
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
                message: t!("validation_required", attribute = t!("identifier")).to_string()
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
