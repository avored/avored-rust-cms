use crate::api::proto::content::StoreContentRequest;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, ErrorResponse, Validate};
use rust_i18n::t;

impl StoreContentRequest {
    /// validate
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        let model_counte = state
            .content_service
            .count_of_identifier(&state.db, &self.content_type, &self.identifier)
            .await?;

        if model_counte.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_count", attribute = t!("identifier")).to_string(),
            };

            errors.push(error_message);
        }

        // if profile photo exist then certain type of photo is only allowed
        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string(),
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
