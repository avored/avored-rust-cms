use crate::api::proto::auth::ForgotPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, ErrorResponse, Validate};
use rust_i18n::t;

impl ForgotPasswordRequest {
    /// validate
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<(bool, String)> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if !self.email.required()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !self.email.validate_email()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        let admin_user_model = state
            .admin_user_service
            .count_of_email(&state.db, self.email.clone())
            .await?;

        if admin_user_model.total != 1 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("not_found", attribute = t!("email")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        let error_response = ErrorResponse {
            status: false,
            errors,
        };

        let error_string = serde_json::to_string(&error_response)?;

        Ok((valid, error_string))
    }
}
