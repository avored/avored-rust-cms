use rust_i18n::t;
use serde::Deserialize;
use crate::avored_state::AvoRedState;
use crate::models::password_rest_model::PasswordResetTokenStatus;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone)]
pub struct AdminUserResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub token: String
}

impl AdminUserResetPasswordRequest {
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.email.required()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string()
            };

            errors.push(error_message);
        }

        if !self.email.validate_email()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string()
            };

            errors.push(error_message);
        }
        if !self.password.required()? {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string()
            };

            errors.push(error_message);
        }
        if !self.confirm_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("confirm_password"),
                message: t!("validation_required", attribute = t!("confirm_password")).to_string()
            };

            errors.push(error_message);
        }

        if self.password != self.confirm_password {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("password_did_not_match_confirmation_password").to_string()
            };

            errors.push(error_message);
        }

        if !self.token.required()? {
            let error_message = ErrorMessage {
                key: String::from("token"),
                message: t!("validation_required", attribute = t!("token")).to_string()
            };

            errors.push(error_message);
        }

        let password_reset_model = state
            .admin_user_service
            .get_password_reset_by_email(&state.db, self.email.clone(), self.token.clone())
            .await?;

        if password_reset_model.status == PasswordResetTokenStatus::Expire {
            let error_message = ErrorMessage {
                key: String::from("token"),
                message: t!("password_reset_token_expire").to_string()
            };
            errors.push(error_message);
        }

        Ok(errors)
    }
}