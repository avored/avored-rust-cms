use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::models::BaseModel;

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
//
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct UpdatableSettingModel {
    pub id: String,
    pub value: String,
    pub logged_in_username: String,
}