use rust_i18n::t;
use serde::Deserialize;
use crate::models::collection_model::{CollectionFieldDataType, CollectionFieldFieldType};
use crate::models::validation_error::{ErrorMessage, Validate};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdateCollectionRequest {
    pub name: String,
    pub collection_fields: Vec<UpdatableCollectionFieldRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct UpdatableCollectionFieldRequest {
    pub name: String,
    pub identifier: String,
    pub data_type: CollectionFieldDataType,
    pub field_type: CollectionFieldFieldType,
}

impl UpdateCollectionRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
