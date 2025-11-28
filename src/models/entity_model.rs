use std::time::SystemTime;

use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::error::{Error, Result};
use crate::api::proto::entity::EntityModel as GrpcEntityModel;
use crate::models::BaseModel;
use prost_types::Timestamp;


/// Represents an visitor log model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct EntityModel {

    /// The unique identifier for the visitor log.
    pub id: String,

    /// the name associated with entity model
    pub name: String,

    /// the identifier associated with entity model
    pub identifier: String,

    /// The date and time when the entity was created.
    pub created_at: Datetime,

    /// The date and time when the entity was last updated.
    pub updated_at: Datetime,

    /// The username of the user who created this entity.
    pub created_by: String,

    /// The username of the user who last updated this entity.
    pub updated_by: String,

}

/// Model for creating a new entity
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableEntityModel {
    /// The name of the entity
    pub name: String,

    /// The identifier of the entity
    pub identifier: String,

    /// The username of the logged-in user creating the entity
    pub logged_in_username: String,
}

/// Model for updating an entity
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableEntityModel {
    /// The unique identifier of the entity to be updated
    pub id: String,

    /// The name of the entity
    pub name: String,

    /// The identifier of the entity
    pub logged_in_username: String,
}


/// Represents a model for updating the identifier of an existing entity.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutEntityIdentifierModel {
    /// Unique identifier for the entity to be updated
    pub id: String,

    /// New identifier for the entity, used for API access
    pub identifier: String,

    /// Username of the user updating the entity identifier
    pub logged_in_username: String,
}



impl TryFrom<EntityModel> for GrpcEntityModel {
    type Error = Error;

    fn try_from(val: EntityModel) -> Result<Self> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);


        

        let model: Self = Self {
            id: val.id,
            name: val.name,
            identifier: val.identifier,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
        };

        Ok(model)
    }
}

impl TryFrom<Object> for EntityModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(Self {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}
