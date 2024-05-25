use rust_i18n::t;
use serde::Deserialize;
use crate::models::validation_error::ErrorMessage;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateSettingRequest {
    pub settings: Vec<UpdatableSettingRequest>,
}


#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableSettingRequest {
    pub id: String,
    pub value: String
}
impl UpdateSettingRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut i = 0;
        for setting_request in &self.settings {
            if setting_request.id.len() <= 0 {
                let error_message = ErrorMessage {
                    key: format!("settings.{i}.id"),
                    message: String::from(t!("validation_required", attribute = t!("id")))
                };

                errors.push(error_message);
            }
            if setting_request.value.len() <= 0 {
                let error_message = ErrorMessage {
                    key: format!("settings.{i}.value"),
                    message: String::from(t!("validation_required", attribute = t!("value")))
                };

                errors.push(error_message);
            }

            i = i+1;
        }
        Ok(errors)
    }
}
