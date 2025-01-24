use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use utoipa::ToSchema;

use super::{BaseModel, Pagination};

#[derive(Serialize, Debug, Deserialize, Clone, Default, ToSchema)]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    #[schema(value_type=String)]
    pub created_at: Datetime,
    #[schema(value_type=String)]
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleOptionModel {
    pub label: String,
    pub value: String,
}

impl TryFrom<Object> for RoleModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<RoleModel> {
        let id = val.get("id").get_id()?;
        let name = val.get("name").get_string()?;
        let identifier = val.get("identifier").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;
        let permissions = match val.get("permissions") {
            Some(val) => match val.clone() {
                Value::Array(v) => {
                    let mut arr = Vec::new();

                    for array in v.into_iter() {
                        arr.push(array.as_string())
                    }
                    arr
                }
                _ => Vec::new(),
            },
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
    pub logged_in_username: String,
    pub permissions: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutRoleIdentifierModel {
    pub id: String,
    pub identifier: String,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RolePagination {
    pub data: Vec<RoleModel>,
    pub pagination: Pagination,
}
