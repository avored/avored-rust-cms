use crate::{
    api::proto::entity::DeleteEntityRequest,
    models::validation_error::{ErrorMessage, ErrorResponse, Validate},
};
use rust_i18n::t;

impl DeleteEntityRequest {
    /// validate
    pub fn validate(&self, locale: String) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.entity_id.required()? {
            let error_message = ErrorMessage {
                key: String::from("entity_id"),
                message: t!("validation_required", locale = locale, attribute = t!("entity_id", locale = locale)).to_string(),
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
