use crate::core::application::use_cases::EntityUseCase;
use crate::core::domain::entities::entity::{
    EntityModel, StorableEntity, StorableEntityAttribute, UpdableIdentifierEntity,
};
use crate::core::domain::entities::error_message::{ErrorMessageResponse, ErrorResponse};
use crate::core::domain::entities::AttributeModel;
use crate::core::domain::extensions::string_extension::StringExtension;
use crate::core::domain::repositories::EntityRepository;
use crate::error::Result;
use rust_i18n::t;
use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;


/*  #region Paginate Command */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaginateEntityCommand {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

/* #endregion */

/* #region Create Attribute */

#[derive(Debug, Deserialize, Default, Clone)]
pub struct CreateAttributeCommand {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub field_type: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateEntityCommand {
    pub name: String,
    pub identifier: String,
    pub attributes: Vec<CreateAttributeCommand>,
}

impl CreateEntityCommand {
    pub fn to_storable(&self, logged_in_user_email: String) -> StorableEntity {
        StorableEntity {
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            logged_in_user_email,
            attributes: self
                .attributes
                .iter()
                .map(|attribute| attribute.to_storable())
                .collect(),
        }
    }

    pub async fn validate(
        &self,
        locale: &str,
        entity_usecase: &EntityUseCase<impl EntityRepository>,
    ) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("name"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("name", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        if !self.identifier.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        let identifier_count = entity_usecase
            .entity_count_by_identifier(&self.identifier)
            .await?;

        if identifier_count > 0 {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "unique",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
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

impl CreateAttributeCommand {
    pub fn to_storable(&self) -> StorableEntityAttribute {
        StorableEntityAttribute {
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            data_type: self.data_type.clone(),
            field_type: self.field_type.clone(),
        }
    }
}

/* #endregion */

/* #region Update entity command */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateEntityCommand {
    pub name: String,
    pub identifier: String,
}

impl UpdateEntityCommand {
    pub fn to_storable(&self, logged_in_user_email: String) -> StorableEntity {
        StorableEntity {
            name: self.name.clone(),
            identifier: self.identifier.clone(),
            logged_in_user_email,
            attributes: vec![],
        }
    }

    pub async fn validate(
        &self,
        locale: &str,
        entity_usecase: &EntityUseCase<impl EntityRepository>,
    ) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.name.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("name"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("name", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        if !self.identifier.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        let identifier_count = entity_usecase
            .entity_count_by_identifier(&self.identifier)
            .await?;

        if identifier_count > 0 {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "unique",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
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

/* #endregion */

/* #region Entity option response */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityOptionResponse {
    pub id: String,
    pub name: String,
}
/* #endregion */

/* #region From trait for entity option response */
impl From<EntityModel> for EntityOptionResponse {
    fn from(model: EntityModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
        }
    }
}
/* #endregion */

/* #region Update identifier command */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PutEntityIdentifierCommand {
    pub identifier: String,
}

impl PutEntityIdentifierCommand {
    pub fn to_updatable_identifier(&self, logged_in_user_email: String) -> UpdableIdentifierEntity {
        UpdableIdentifierEntity {
            identifier: self.identifier.clone(),
            logged_in_user_email,
        }
    }

    pub async fn validate(
        &self,
        locale: &str,
        entity_usecase: &EntityUseCase<impl EntityRepository>,
    ) -> Result<Vec<ErrorMessageResponse>> {
        let mut errors: Vec<ErrorMessageResponse> = vec![];
        let mut valid = true;

        if !self.identifier.is_required()? {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "required",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
            });
            valid = false;
        }

        let identifier_count = entity_usecase
            .entity_count_by_identifier(&self.identifier)
            .await?;

        if identifier_count > 0 {
            errors.push(ErrorMessageResponse {
                key: String::from("identifier"),
                message: t!(
                    "unique",
                    locale = locale,
                    attribute = t!("identifier", locale = locale)
                )
                .to_string(),
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
/* #endregion */

/* #region Entity response */

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttributeResponse {
    pub id: String,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityResponse {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
    pub attributes: Vec<AttributeResponse>,
}

impl From<EntityModel> for EntityResponse {
    fn from(model: EntityModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            identifier: model.identifier,
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
            deleted_at: model.deleted_at,
            deleted_by: model.deleted_by,
            attributes: model.attributes.iter().map(|f| f.clone().into() ).collect()
        }
    }
}

impl From<AttributeModel> for AttributeResponse {
    fn from(model: AttributeModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            identifier: model.identifier,
            field_type: model.field_type,
            data_type: model.data_type,
            created_at: model.created_at,
            created_by: model.created_by,
            updated_at: model.updated_at,
            updated_by: model.updated_by,
            deleted_at: model.deleted_at,
            deleted_by: model.deleted_by,
        }
    }
}
/* #endregion */

/* #region Entity Paginate response */
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EntityPaginationResponse {
    pub data: Vec<EntityResponse>,
    pub total: u64,
}
/* #endregion */
