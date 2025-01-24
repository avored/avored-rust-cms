use crate::models::validation_error::{ErrorMessage, Validate};
use rust_i18n::t;
use serde::Deserialize;
use crate::models::content_model::{ContentDataType, ContentFieldContentType, ContentFieldType};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateContentRequest {
    pub name: String,
    pub identifier: String,
    pub content_type: String,
    pub content_fields: Vec<UpdatableContentField>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableContentField {
    pub name: String,
    pub identifier: String,
    pub data_type: ContentDataType,
    pub field_type: ContentFieldType,
    pub field_content: ContentFieldContentType,
}

impl UpdateContentRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
