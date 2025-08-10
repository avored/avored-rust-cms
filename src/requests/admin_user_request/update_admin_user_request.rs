use crate::api::proto::admin_user::UpdateAdminUserRequest;
use crate::models::validation_error::{ErrorMessage, ErrorResponse, Validate};
use rust_i18n::t;

impl UpdateAdminUserRequest {
    /// validate
    pub async fn validate(&self) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.full_name.required()? {
            let error_message = ErrorMessage {
                key: String::from("full_name"),
                message: t!("validation_required", attribute = t!("full_name")).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        if !valid {
            let error_response = ErrorResponse {
                status: valid,
                errors,
            };
            let error_string = serde_json::to_string(&error_response)?;
            return Err(crate::error::Error::InvalidArgument(error_string));
        }

        Ok(())
    }
}
