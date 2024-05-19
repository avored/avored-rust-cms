use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

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
        let id = match val.get("id") {
            Some(val) => match val.clone() {
                Value::Thing(v) => {
                    let id = v.id;
                    id.to_string()
                }
                _ => String::from(""),
            },
            None => String::from(""),
        };
        let email = match val.get("email") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let token = match val.get("token") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let created_at = match val.get("created_at") {
            Some(val) => match val.clone() {
                Value::Datetime(v) => v,
                _ => Datetime::default(),
            },
            None => Datetime::default(),
        };

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