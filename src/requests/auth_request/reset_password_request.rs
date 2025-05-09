use email_address::EmailAddress;
use rust_i18n::t;
use crate::api::proto::auth::ResetPasswordRequest;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

impl ResetPasswordRequest {
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<(bool, String)> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;

        if self.email.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !EmailAddress::is_valid(&self.email) {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        if self.password.is_empty() {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        if self.password != self.confirm_password {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("password_match_error").to_string(),
            };

            valid = false;
            errors.push(error_message);
        }
        
        let validated_token_result = state
            .auth_service
            .validate_token(
                &self.token,
                &self.email,
                &state.db,
            ).await?;

        if !validated_token_result {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("not_valid_password_reset_token").to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

        let error_response = ErrorResponse {
            status: false,
            errors,
        };

        let error_string = serde_json::to_string(&error_response)?;


        Ok((valid ,error_string))
    }
}