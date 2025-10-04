use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::error::{Error, Result};
use super::BaseModel;
use crate::api::proto::dashboard::VisitByYear as GrpcVisitByYear;
use crate::api::proto::dashboard::VisitByContentType as GrpcVisitByContentType;

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

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
/// Visit by year
pub struct VisitByYear {

    /// The unique identifier for the visitor log.
    pub visits: i64,

    /// The content tpye of the visitor log.
    pub month: String,

    /// The content id of the visitor log.
    pub month_number: i64,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
/// Visit by year
pub struct VisitByContentType {

    /// The unique identifier for the visitor log.
    pub visits: i64,

    /// The content tpye of the visitor log.
    pub month: String,

    /// The content id of the visitor log.
    pub month_number: i64,
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



impl TryFrom<Object> for VisitByYear {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        
        let visits = val.get("visits").get_int()?;
        let month: String = val.get("month").get_string()?;
        let month_number = val.get("month_number").get_int()?;

        Ok(Self {
            visits,
            month,
            month_number
        })
    }
}



impl TryFrom<Object> for VisitByContentType {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
        
        let visits = val.get("visits").get_int()?;
        let month: String = val.get("month").get_string()?;
        let month_number = val.get("month_number").get_int()?;

        Ok(Self {
            visits,
            month,
            month_number
        })
    }
}


impl TryFrom<VisitByYear> for GrpcVisitByYear {
    type Error = Error;

    fn try_from(val: VisitByYear) -> Result<Self> {

        let model: Self = Self {
            visits: val.visits,
            month: val.month
        };

        Ok(model)
    }
}

impl TryFrom<VisitByContentType> for GrpcVisitByContentType {
    type Error = Error;

    fn try_from(val: VisitByContentType) -> Result<Self> {

        let model: Self = Self {
            visits: val.visits,
            month: val.month,
            content_type: String::from(""),
        };

        Ok(model)
    }
}


// endregion: impl