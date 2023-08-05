use serde::Serialize;
use serde_derive::Deserialize;
use surrealdb::sql::{Object, Value, Datetime};

use crate::error::{Error, Result};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct AdminUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for AdminUser {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUser> {
        let id = match val.get("id") {
            Some(val) => val.clone(),
            None => Value::Null,
        };

        let name = match val.get("name") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let email = match val.get("email") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let password = match val.get("password") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let created_at = match val.get("created_at") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let updated_at = match val.get("updated_at") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let created_by = match val.get("created_by") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let updated_by = match val.get("updated_by") {
            Some(val) => val.clone(),
            None => Value::Null,
        };

        Ok(AdminUser {
            id: id.to_string(),
            name: name.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            created_at: created_at.to_datetime(),
            updated_at: updated_at.to_datetime(),
            created_by: created_at.to_string(),
            updated_by: updated_by.to_string(),
        })
    }
}
