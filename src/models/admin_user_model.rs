use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use utoipa::ToSchema;
use crate::models::role_model::RoleModel;

use super::{BaseModel, Pagination};

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
        let id = val.get("id").get_id()?;
        let full_name = val.get("full_name").get_string()?;
        let email = val.get("email").get_string()?;
        let password = val.get("password").get_string()?;
        let mut profile_image = val.get("profile_image").get_string()?;
        if profile_image.is_empty() {
            profile_image = String::from("https://place-hold.it/250x250");
        } else {
            //@todo fix this
            profile_image = format!("http://localhost:8080/public/{}", profile_image);
        }

        let is_super_admin = val.get("is_super_admin").get_bool()?;

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


        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

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
