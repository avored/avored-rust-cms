use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, Validate};
use axum::extract::State;
use rust_i18n::t;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct PutCollectionRequest {
    pub identifier: String,
}

impl PutCollectionRequest {
    pub async fn validate(
        &self,
        state: State<Arc<AvoRedState>>,
    ) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string(),
            };

            errors.push(error_message);
        }

        let collection_count = state
            .collection_service
            .count_of_identifier(&state.db, self.identifier.clone())
            .await?;

        if collection_count.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_count", attribute = t!("identifier")).to_string(),
            };
            errors.push(error_message);
        }

        Ok(errors)
    }
}
