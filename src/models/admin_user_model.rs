use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use utoipa::ToSchema;
use crate::models::role_model::RoleModel;

use super::Pagination;

#[derive(Serialize, Debug, Deserialize, Clone, Default, ToSchema)]
pub struct AdminUserModel {
    pub id: String,
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    #[schema(value_type=String)]
    pub created_at: Datetime,
    #[schema(value_type=String)]
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    pub roles: Vec<RoleModel>,
}

impl TryFrom<Object> for AdminUserModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<AdminUserModel> {
        let id = match val.get("id") {
            Some(val) => {
                
                match val.clone() {
                    Value::Thing(v) => {
                        let id = v.id;
                        id.to_string()
                    }
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };
        let full_name = match val.get("full_name") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let email = match val.get("email") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let password = match val.get("password") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let mut profile_image = match val.get("profile_image") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        if profile_image.is_empty() {
            profile_image = String::from("https://place-hold.it/250x250");
        } else {
            //@todo fix this
            profile_image = format!("http://localhost:8080/public/{}", profile_image);
        }

        let is_super_admin = match val.get("is_super_admin") {
            Some(val) => {
                
                match val.clone() {
                    Value::Bool(v) => v,
                    _ => false,
                }
            }
            None => false,
        };

        let roles = match val.get("roles") {
            Some(val) => {
                
                match val.clone() {
                    Value::Array(v) => {
                        let mut arr = Vec::new();

                        for array in v.into_iter() {
                            let object = match array.clone() {
                                Value::Object(v) => v,
                                _ => surrealdb::sql::Object::default(),
                            };

                            let role_model: RoleModel = object.try_into()?;

                            arr.push(role_model)
                        }
                        arr
                    }
                    _ => Vec::new(),
                }
            }
            None => Vec::new(),
        };


        let created_at = match val.get("created_at") {
            Some(val) => {
                
                match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                }
            }
            None => Datetime::default(),
        };
        let updated_at = match val.get("updated_at") {
            Some(val) => {
                
                match val.clone() {
                    Value::Datetime(v) => v,
                    _ => Datetime::default(),
                }
            }
            None => Datetime::default(),
        };

        let created_by = match val.get("created_by") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
            }
            None => String::from(""),
        };

        let updated_by = match val.get("updated_by") {
            Some(val) => {
                
                match val.clone() {
                    Value::Strand(v) => v.as_string(),
                    _ => String::from(""),
                }
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
            roles
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableAdminUserModel {
    pub full_name: String,
    pub email: String,
    pub password: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
    pub role_ids: Vec<String>
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableAdminUserModel {
    pub id: String,
    pub full_name: String,
    pub profile_image: String,
    pub is_super_admin: bool,
    pub logged_in_username: String,
    pub role_ids: Vec<String>
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AdminUserPagination {
    pub data: Vec<AdminUserModel>,
    pub pagination: Pagination,
}
