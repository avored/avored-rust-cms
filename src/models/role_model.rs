use crate::error::{Error, Result};
use serde_derive::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for RoleModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<RoleModel> {
        let id = match val.get("id") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let test = id.clone();

        println!("{:?}", test);

        let id = id.as_raw_string();
        let identifier = match id.split(":").nth(1) {
            Some(id) => id,
            None => "",
        };

        let name = match val.get("name") {
            Some(val) => val.clone(),
            None => Value::Null,
        };
        let unique_identifier = match val.get("identifier") {
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

        Ok(RoleModel {
            id: identifier.to_string(),
            name: name.as_raw_string(),
            identifier: unique_identifier.as_raw_string(),
            created_at: created_at.as_datetime(),
            updated_at: updated_at.as_datetime(),
            created_by: created_by.as_raw_string(),
            updated_by: updated_by.as_raw_string(),
        })
    }
}



impl RoleModel {
    pub fn empty() -> Self {
        RoleModel {
            id: String::from(""),
            name: String::from(""),
            identifier: String::from(""),
            created_at: Datetime::from(chrono::Utc::now()),
            updated_at: Datetime::from(chrono::Utc::now()),
            created_by: String::from(""),
            updated_by: String::from(""),
        }
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableRole {
    pub name: String,
    pub identifier: String,
    pub logged_in_user_email: String,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableRole {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub logged_in_user_email: String,
}
