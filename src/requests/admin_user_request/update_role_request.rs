use rust_i18n::t;
use crate::api::proto::admin_user::UpdateRoleRequest;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

impl UpdateRoleRequest {
    pub fn validate(&self) -> crate::error::Result<(bool, String)> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if self.name.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        
        let error_response = ErrorResponse {
            status: valid,
            errors,
        };

        let error_string = serde_json::to_string(&error_response)?;


        Ok((valid ,error_string))
    }
}