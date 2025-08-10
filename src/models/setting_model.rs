use crate::error::{Error, Result};
use crate::models::BaseModel;
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use surrealdb::sql::{Datetime, Object};

/// Represents a setting in the system
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SettingModel {

    /// Unique identifier for the setting
    pub id: String,

    /// Identifier for the setting, used to access its value
    pub identifier: String,

    /// Value of the setting, which can be a string representation of any data type
    pub value: String,

    /// Timestamps for creation and last update
    pub created_at: Datetime,

    /// Timestamps for creation and last update
    pub updated_at: Datetime,

    /// Username of the user who created the setting
    pub created_by: String,

    /// Username of the user who last updated the setting
    pub updated_by: String,
}

impl TryFrom<Object> for SettingModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<SettingModel> {
        let id = val.get("id").get_id()?;
        let identifier = val.get("identifier").get_string()?;
        let value = val.get("value").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(SettingModel {
            id,
            identifier,
            value,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

impl TryFrom<SettingModel> for crate::api::proto::setting::SettingModel {
    type Error = Error;

    fn try_from(val: SettingModel) -> Result<crate::api::proto::setting::SettingModel> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model = crate::api::proto::setting::SettingModel {
            id: val.id,
            value: val.value,
            identifier: val.identifier,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
        };

        Ok(model)
    }
}

//

/// Represents a setting that can be updated by the user
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableSettingModel {

    /// Unique identifier for the setting
    pub id: String,

    /// Identifier for the setting, used to access its value
    pub value: String,
    
    /// Username of the user who is updating the setting
    pub logged_in_username: String,
}
