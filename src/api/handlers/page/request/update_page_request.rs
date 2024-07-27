use rust_i18n::t;
use serde::Deserialize;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatePageRequest {
    pub name: String,
    pub components_content: Vec<UpdatableComponentContentRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableComponentContentRequest {
    pub name: String,
    pub identifier: String,
    pub elements: Vec<UpdatableComponentElementContentRequest>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableComponentElementContentRequest {
    pub name: String,
    pub identifier: String,
    pub element_type: String,
    pub element_content: String,
    pub element_data: Option<Vec<EditablePageComponentElementDataOptionRequest>>
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct EditablePageComponentElementDataOptionRequest {
    pub label: String,
    pub value: String,
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
