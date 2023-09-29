use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserModel {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
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
        let full_name = match val.get("full_name") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
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

        let mut profile_image = match val.get("profile_image") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                };
                value
            }
            None => String::from(""),
        };

        if profile_image.is_empty() {
            profile_image = String::from("https://place-hold.it/250x250");
        } else {
            profile_image = format!("/public/{}", profile_image);
        }

        let is_super_admin = match val.get("is_super_admin") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Bool(v) => v,
                    _ => false,
                };
                value
            }
            None => false,
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

        Ok(AdminUserModel {
            id,
            full_name,
            email,
            password,
            profile_image,
            is_super_admin,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableAdminUser {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableAdminUserModel {
    pub id: String,
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserPagination {
    pub data: Vec<AdminUserModel>,
    pub pagination: Pagination,
}
