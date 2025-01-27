use crate::models::validation_error::{ErrorMessage, Validate};
use rust_i18n::t;
use serde::Deserialize;
use crate::models::collection_model::{CollectionFieldDataType, CollectionFieldFieldType};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct StoreCollectionRequest {
    pub name: String,
    pub identifier: String,
    pub collection_fields: Vec<CreatableCollectionFieldRequest>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CreatableCollectionFieldRequest {
    pub name: String,
    pub identifier: String,
    pub data_type: CollectionFieldDataType,
    pub field_type: CollectionFieldFieldType,
}

impl StoreCollectionRequest {
    pub fn validate(&self) -> crate::error::Result<Vec<ErrorMessage>> {
        let mut errors: Vec<ErrorMessage> = vec![];

        if !self.name.required()? {
            let error_message = ErrorMessage {
                key: String::from("name"),
                message: t!("validation_required", attribute = t!("name")).to_string(),
            };

            errors.push(error_message);
        }

        if !self.identifier.required()? {
            let error_message = ErrorMessage {
                key: String::from("identifier"),
                message: format!("Identifier is a required field {}", t!("identifier")),
            };

            errors.push(error_message);
        }

        Ok(errors)
    }
}
