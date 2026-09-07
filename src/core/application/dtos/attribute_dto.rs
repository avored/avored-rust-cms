use serde::{Deserialize, Serialize};
use rust_i18n::t;
use surrealdb::types::Datetime;
use crate::core::application::use_cases::AttributeUseCase;
use crate::core::domain::entities::{AttributeModel, StorableAttribute};
use crate::core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse};
use crate::core::domain::extensions::string_extension::StringExtension;
use crate::core::domain::repositories::AttributeRepository;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginateAttributeCommand {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateAttributeCommand {
    pub entity_id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
}

impl CreateAttributeCommand {
    pub fn to_storable(&self, logged_in_user_email: String) -> StorableAttribute {
        StorableAttribute {
            entity_id: self.entity_id.clone(),
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            data_type: self.data_type.clone(),
            field_type: self.field_type.clone(),
            logged_in_user_email
        }
    }

    pub async fn validate(&self, locale: &str, attribute_usecase: &AttributeUseCase<impl AttributeRepository>) -> Result<Vec<ErrorMessageResponse>> {
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


        if !self.field_type.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("field_type"),
                message: t!("required", locale = locale, attribute = t!("field_type", locale = locale)).to_string(),
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


        let identifier_count = attribute_usecase.attribute_count_by_identifier(&self.identifier).await?;

        if identifier_count > 0 {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!("unique", locale = locale, attribute = t!("identifier", locale = locale)).to_string(),
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
pub struct UpdateAttributeCommand {
    pub entity_id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
}

impl UpdateAttributeCommand {
    
    pub fn to_storable(&self, logged_in_user_email: String) -> StorableAttribute {
        StorableAttribute {
            entity_id: self.entity_id.clone(),
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            data_type: self.data_type.clone(),
            field_type: self.field_type.clone(),
            logged_in_user_email,
        }
    }

    pub async fn validate(&self, locale: &str, _attribute_usecase: &AttributeUseCase<impl AttributeRepository>) -> Result<Vec<ErrorMessageResponse>> {

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
pub struct AttributeResponse {
    pub id: String,
    pub entity_id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
}

impl From<AttributeModel> for AttributeResponse {
    fn from(model: AttributeModel) -> Self {
        Self {
            id: model.id,
            entity_id: model.entity_id,
            name: model.name,
            identifier: model.identifier,
            data_type: model.data_type,
            field_type: model.field_type,
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
            deleted_at: model.deleted_at,
            deleted_by: model.deleted_by,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttributePaginationResponse {
    pub data: Vec<AttributeResponse>,
    pub total: u64,
}
