use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Object, Value};

#[derive(Serialize, Debug, Clone, Deserialize)]
pub struct AdminUserModel {
    pub id: String,
    pub email: String,
    pub password: String,
}

impl TryFrom<Object> for AdminUserModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUserModel> {
        let id = match val.get("id") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Thing(v) => {
                        let id = v.id;
                        id.to_string()
                    }
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        let email = match val.get("email") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        let password = match val.get("password") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        Ok(AdminUserModel {
            id,
            email,
            password,
        })
    }
}
