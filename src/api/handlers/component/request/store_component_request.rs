use serde::Deserialize;
use rust_i18n::t;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone)]
pub struct StoreComponentRequest {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<CreatableElementRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableElementRequest {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_data_type: String,
    pub element_data: Option<Vec<CreatableComponentElementDataRequest>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableComponentElementDataRequest {
    pub label: String,
    pub value: String,
}

impl StoreComponentRequest {
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
