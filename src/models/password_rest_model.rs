use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use crate::models::BaseModel;

#[derive(Serialize, Debug, Deserialize, Clone, Default, PartialEq)]
pub struct PasswordResetModel {
    pub id: String,
    pub email: String,
    pub token: String,
    pub status: PasswordResetTokenStatus,
    pub created_at: Datetime,
}

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, Default)]
pub enum PasswordResetTokenStatus {
    Active,
    #[default]
    Expire
}

impl TryFrom<Object> for PasswordResetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PasswordResetModel> {
        let id = val.get("id").get_id()?;
        let email = val.get("email").get_string()?;
        let token = val.get("token").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let status = match val.get("status").get_string()?.as_str() {
            "Active" => PasswordResetTokenStatus::Active,
            "Expire" => PasswordResetTokenStatus::Active,
            _ => PasswordResetTokenStatus::Expire
        };

        Ok(PasswordResetModel {
            id,
            email,
            token,
            created_at,
            status
        })
    }
}



#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatablePasswordResetModel {
    pub email: String,
    pub token: String,
}
