use crate::{
    api::proto::admin_user::DeleteAdminUserRequest,
    models::validation_error::{ErrorMessage, ErrorResponse, Validate},
};
use rust_i18n::t;

impl DeleteAdminUserRequest {
    /// validate
    pub fn validate(&self, locale: String) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.admin_user_id.required()? {
            let error_message = ErrorMessage {
                key: String::from("admin_user"),
                message: t!("validation_required", locale = locale, attribute = t!("admin_user", locale = locale)).to_string(),
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
