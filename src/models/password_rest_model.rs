use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::models::BaseModel;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PasswordResetModel {
    pub id: String,
    pub email: String,
    pub token: String,
    pub created_at: Datetime,
}

impl TryFrom<Object> for PasswordResetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PasswordResetModel> {
        let id = val.get("id").get_id()?;
        let email = val.get("email").get_string()?;
        let token = val.get("token").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;

        Ok(PasswordResetModel {
            id,
            email,
            token,
            created_at,
        })
    }
}



#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatablePasswordResetModel {
    pub email: String,
    pub token: String,
}