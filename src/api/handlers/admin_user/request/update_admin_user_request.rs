use crate::models::validation_error::{ErrorMessage, Validate};
use rust_i18n::t;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateAdminUserRequest {
    pub full_name: String,
    pub is_super_admin: bool,
    pub role_ids: Vec<String>,
}

impl UpdateAdminUserRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        if !self.full_name.required()? {
            let error_message = ErrorMessage {
                key: String::from("full_name"),
                message: t!("validation_required", attribute = t!("full_name")).to_string(),
            };

            errors.push(error_message);
        }
        Ok(errors)
    }
}
