use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, Validate};
use rust_i18n::t;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StoreAdminUserRequest {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub confirmation_password: String,
    pub is_super_admin: bool,
    pub role_ids: Vec<String>,
}

impl StoreAdminUserRequest {
    pub async fn validate(&self, state: &AvoRedState) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];
        if !self.full_name.required()? {
            let error_message = ErrorMessage {
                key: String::from("full_name"),
                message: t!("validation_required", attribute = t!("full_name")).to_string(),
            };

            errors.push(error_message);
        }

        if !self.email.required()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", attribute = t!("email")).to_string(),
            };

            errors.push(error_message);
        }

        if !self.email.validate_email()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid").to_string(),
            };

            errors.push(error_message);
        }

        if !self.password.required()? {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", attribute = t!("password")).to_string(),
            };

            errors.push(error_message);
        }

        if !self.confirmation_password.required()? {
            let error_message = ErrorMessage {
                key: String::from("confirmation_password"),
                message: t!(
                    "validation_required",
                    attribute = t!("confirmation_password")
                )
                .to_string(),
            };

            errors.push(error_message);
        }

        if self.password != self.confirmation_password {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("password_did_not_match_confirmation_password").to_string(),
            };

            errors.push(error_message);
        }

        let admin_user_model = state
            .admin_user_service
            .count_of_email(&state.db, self.email.clone())
            .await?;

        if admin_user_model.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_count", attribute = t!("email")).to_string(),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
