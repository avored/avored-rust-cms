use rust_i18n::t;
use serde::Deserialize;
use crate::avored_state::AvoRedState;
use crate::models::validation_error::{ErrorMessage, Validate};
use crate::error::Result;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StorePageRequest {
    pub name: String,
    pub identifier: String,
    pub page_fields: Vec<CreatablePageFieldRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatablePageFieldRequest {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub field_content: String,
}

impl StorePageRequest {
    pub async fn validate(&self, state: &AvoRedState) -> Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string()
            };

            errors.push(error_message);
        }

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_required", attribute = t!("identifier")).to_string()
            };

            errors.push(error_message);
        }

        let page_count = state
            .page_service
            .count_of_identifier(&state.db, self.identifier.clone())
            .await?;

        if page_count.total > 0 {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: t!("validation_count", attribute = t!("identifier")).to_string()
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
