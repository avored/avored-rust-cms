use crate::api::proto::content::DeleteContentRequest;
use crate::models::validation_error::{ErrorMessage, ErrorResponse, Validate};
use rust_i18n::t;

impl DeleteContentRequest {
    /// validate
    pub fn validate(&self) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.content_id.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("content_id")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !self.content_type.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("content_type")).to_string(),
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
