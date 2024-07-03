use rust_i18n::t;
use serde::Deserialize;

use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StorePageRequest {
    // #[validate(length(min = 1, message = "The name is a required field."))]
    pub name: String,

    // #[validate(length(min = 1, message = "The identifier is a required field."))]
    pub identifier: String,

    pub components_content: Vec<CreatableComponentContentRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableComponentContentRequest {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub fields: Vec<CreatableComponentFieldContentRequest>
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableComponentFieldContentRequest {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub field_type: String,
    pub field_content: String,
    pub field_data: Option<Vec<CreatablePageComponentFieldDataOptionRequest>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatablePageComponentFieldDataOptionRequest {
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
