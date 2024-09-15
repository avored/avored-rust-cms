use rust_i18n::t;
use serde::Deserialize;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatePageRequest {
    pub name: String,
    pub identifier: String,
    pub page_fields: Vec<UpdatablePageField>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatablePageField {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub field_content: String,
}


impl UpdatePageRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
    //
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
