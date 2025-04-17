use std::time::SystemTime;
use prost_types::Timestamp;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};
use super::{BaseModel, Pagination};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
    // pub permissions: Vec<String>,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleOptionModel {
    pub label: String,
    pub value: String,
}


impl TryFrom<RoleModel> for crate::grpc_admin_user::RoleModel {
    type Error = Error;

    fn try_from(val: RoleModel) -> Result<crate::grpc_admin_user::RoleModel> {
        let chrono_utc_created_at= val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at= val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model: crate::grpc_admin_user::RoleModel = crate::grpc_admin_user::RoleModel {
            id: val.id,
            name: val.name,
            identifier: val.identifier,
            created_at: Option::from(created_at),
            updated_at: Option::from(updated_at),
            created_by: val.created_by,
            updated_by: val.updated_by,
        };

        Ok(model)
    }
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
        // let permissions = match val.get("permissions") {
        //     Some(val) => match val.clone() {
        //         Value::Array(v) => {
        //             let mut arr = Vec::new();
        //
        //             for array in v.into_iter() {
        //                 arr.push(array.as_string())
        //             }
        //             arr
        //         }
        //         _ => Vec::new(),
        //     },
        //     None => Vec::new(),
        // };

        Ok(RoleModel {
            id,
            name,
            identifier,
            created_at,
            updated_at,
            created_by,
            updated_by,
            // permissions,
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
