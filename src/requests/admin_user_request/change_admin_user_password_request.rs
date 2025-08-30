use crate::{
    api::proto::admin_user::ChangeAdminUserPasswordRequest,
    models::validation_error::{ErrorMessage, ErrorResponse, Validate},
};
use rust_i18n::t;

impl ChangeAdminUserPasswordRequest {
    /// validate
    pub fn validate(&self) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.password.required()? {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }
        
        if !self.new_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("new_password"),
                message: t!("validation_required", attribute = t!("new_password")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !self.confirm_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("validation_required", attribute = t!("confirm_password")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if self.new_password != self.confirm_password {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("validation_confirmed", attribute = t!("confirm_password")).to_string(),
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
