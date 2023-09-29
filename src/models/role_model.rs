use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    pub permissions: Vec<String>,
}

impl TryFrom<Object> for RoleModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<RoleModel> {
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
        let name = match val.get("name") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };
        let created_at = match val.get("created_at") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                };
                value
            }
            None => Datetime::default(),
        };
        let updated_at = match val.get("updated_at") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                };
                value
            }
            None => Datetime::default(),
        };

        let created_by = match val.get("created_by") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        let updated_by = match val.get("updated_by") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        let permissions = match val.get("permissions") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            arr.push(array.as_string())
                        }
                        arr
                    }
                    _ => Vec::new(),
                };
                value
            }
            None => Vec::new(),
        };

        Ok(RoleModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
            permissions,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableRole {
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableRoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub logged_in_username: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RolePagination {
    pub data: Vec<RoleModel>,
    pub pagination: Pagination,
}
