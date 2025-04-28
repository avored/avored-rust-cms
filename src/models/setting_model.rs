use std::time::SystemTime;
use prost_types::Timestamp;
use crate::error::{Error, Result};
use crate::models::BaseModel;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct SettingModel {
    pub id: String,
    pub identifier: String,
    pub value: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
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
        let chrono_utc_created_at= val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at= val.updated_at.to_utc();
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


#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableSettingModel {
    pub id: String,
    pub value: String,
    pub logged_in_username: String,
}
