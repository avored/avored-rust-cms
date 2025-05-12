use email_address::EmailAddress;
use rust_i18n::t;
use crate::api::proto::admin_user::StoreAdminUserRequest;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

impl StoreAdminUserRequest {
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

        // if profile photo exist then certain type of photo is only allowed

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

        let error_response = ErrorResponse {
            status: valid,
            errors,
        };

        let error_string = serde_json::to_string(&error_response)?;


        Ok((valid ,error_string))
    }
}
