use serde::{Deserialize, Serialize};
use rust_i18n::t;
use crate::core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse};
use crate::core::domain::entities::entity::{EntityModel, StorableEntity};
use crate::core::domain::extensions::string_extension::StringExtension;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginateEntityCommand {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateEntityCommand {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
}

impl CreateEntityCommand {
    pub fn to_storable(&self) -> StorableEntity {
        StorableEntity {
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            data_type: self.data_type.clone(),
        }
    }

    pub fn validate(&self, locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("name"),
                message: t!("required", locale = locale, attribute = t!("name", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !self.identifier.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!("required", locale = locale, attribute = t!("identifier", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !self.data_type.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("data_type"),
                message: t!("required", locale = locale, attribute = t!("data_type", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(crate::error::Error::BadRequest(ErrorResponse {
                status: false,
                errors,
            }));
        }

        Ok(errors)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEntityCommand {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
}

impl UpdateEntityCommand {
    pub fn to_storable(&self) -> StorableEntity {
        StorableEntity {
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            data_type: self.data_type.clone(),
        }
    }

    pub fn validate(&self, locale: &str) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("name"),
                message: t!("required", locale = locale, attribute = t!("name", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !self.identifier.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!("required", locale = locale, attribute = t!("identifier", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !self.data_type.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("data_type"),
                message: t!("required", locale = locale, attribute = t!("data_type", locale = locale)).to_string(),
            });
            valid = false;
        }

        if !valid {
            return Err(crate::error::Error::BadRequest(ErrorResponse {
                status: false,
                errors,
            }));
        }

        Ok(errors)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityResponse {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

impl From<EntityModel> for EntityResponse {
    fn from(model: EntityModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            identifier: model.identifier,
            data_type: model.data_type,
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityPaginationResponse {
    pub data: Vec<EntityResponse>,
    pub total: u64,
}
