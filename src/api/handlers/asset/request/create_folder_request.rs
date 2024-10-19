use serde::Deserialize;
use rust_i18n::t;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<String>
}

impl CreateFolderRequest {
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
