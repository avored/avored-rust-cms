use rust_i18n::t;
use serde::Deserialize;

use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StorePageRequest {
    pub name: String,
    pub identifier: String,
    pub components_content: Vec<CreatableComponentContentRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableComponentContentRequest {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<CreatableComponentElementContentRequest>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableComponentElementContentRequest {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_content: String,
    pub element_data: Option<Vec<CreatablePageComponentElementDataOptionRequest>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatablePageComponentElementDataOptionRequest {
    pub label: String,
    pub value: String,
}


impl StorePageRequest {
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
