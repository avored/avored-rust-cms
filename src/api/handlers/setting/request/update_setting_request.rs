use crate::models::validation_error::{ErrorMessage, Validate};
use rust_i18n::t;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateSettingRequest {
    pub settings: Vec<UpdatableSettingRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableSettingRequest {
    pub id: String,
    pub identifier: String,
    pub value: String,
}
impl UpdateSettingRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        for setting_request in &self.settings {
            if !setting_request.id.required()? {
                let error_message = ErrorMessage {
                    key: format!("settings.{}.id", setting_request.identifier),
                    message: String::from(t!("validation_required", attribute = t!("id"))),
                };

                errors.push(error_message);
            }
            if !setting_request.value.required()? {
                let error_message = ErrorMessage {
                    key: format!("settings.{}.value", setting_request.identifier),
                    message: String::from(t!("validation_required", attribute = t!("value"))),
                };

                errors.push(error_message);
            }
        }
        Ok(errors)
    }
}
