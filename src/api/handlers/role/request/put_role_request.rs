use std::sync::Arc;
use axum::extract::State;
use rust_i18n::t;
use serde::Deserialize;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PutRoleRequest {
    pub identifier: String,
}

impl PutRoleRequest {
    pub async fn validate(&self, state: State<Arc<AvoRedState>>) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string()
            };

            errors.push(error_message);
        }

        let role_model_count = state
            .role_service
            .count_of_identifier(&state.db, self.identifier.clone())
            .await?;

        if role_model_count.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_count", attribute = t!("identifier")).to_string()
            };
            errors.push(error_message);
        }

        Ok(errors)
    }
}
