use rust_i18n::t;
use serde::Deserialize;
use crate::models::page_model::{PageDataType, PageFieldContentType, PageFieldType, PageFieldData, PageStatus};
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatePageRequest {
    pub name: String,
    pub identifier: String,
    pub status: PageStatus,
    pub page_fields: Vec<UpdatablePageField>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: PageDataType,
    pub field_type: PageFieldType,
    pub field_content: PageFieldContentType,
    pub field_data: PageFieldData
}

impl UpdatePageRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string()
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
