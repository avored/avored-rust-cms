use serde::Deserialize;
use rust_i18n::t;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateComponentRequest {
    pub name: String,
    pub identifier: String,
    pub fields: Vec<UpdatableField>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableField {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_data: Option<Vec<UpdatableFieldDataRequest>>
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableFieldDataRequest {
    pub label: String,
    pub value: String,
}

impl UpdateComponentRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string()
            };

            errors.push(error_message);
        }

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string()
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
