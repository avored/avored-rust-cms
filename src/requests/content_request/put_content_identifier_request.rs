use crate::api::proto::content::PutContentIdentifierRequest;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use rust_i18n::t;

impl PutContentIdentifierRequest {
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if self.identifier.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        let role_identifier_count = state
            .admin_user_service
            .count_of_role_identifier(&state.db, &self.identifier.clone())
            .await?;

        if role_identifier_count.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_count", attribute = t!("email")).to_string(),
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
