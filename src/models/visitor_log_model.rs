use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::error::{Error, Result};
use super::BaseModel;


// region: Struct initialze

/// Represents an visitor log model in the system.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct VistitorLogModel {

    /// The unique identifier for the visitor log.
    pub id: String,

    /// The content tpye of the visitor log.
    pub content_type: String,

    /// The content id of the visitor log.
    pub content_id: String,

    /// The IP address of the visitor log
    pub ip_address: Option<String>,
  
    /// The date and time when the visitor log was created.
    pub created_at: Datetime
}

/// Represents a model for creating an visitor log.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableVisitorLogModel {
    /// The content type of the visitor log.
    pub content_type: String,

    /// The content identifier of the visitor log.
    pub content_id: String,

    /// The password of the visitor log.
    pub ip_address: Option<String>,
}

// endregion: Struct initialize





// region: impl

impl TryFrom<Object> for VistitorLogModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        let id: String = val.get("id").get_id()?;
        let content_id = val.get("content_id").get_string()?;
        let content_type = val.get("content_type").get_string()?;
        let ip_address = val.get("ip_address").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;

        Ok(Self {
            id,
            content_id,
            content_type,
            ip_address: Some(ip_address),
            created_at,
        })
    }
}


// endregion: impl