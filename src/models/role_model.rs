use super::{BaseModel, Pagination};
use crate::error::{Error, Result};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use surrealdb::sql::Value;
use surrealdb::sql::{Datetime, Object};


/// Represents a role in the system with its associated admin user.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleModel {

    /// Unique identifier for the role
    pub id: String,

    /// Name of the role
    pub name: String,

    /// Unique identifier for the role, used for API access
    pub identifier: String,

    /// Timestamps for creation and last update
    pub created_at: Datetime,

    /// Timestamp for the last update of the role
    pub updated_at: Datetime,

    /// Username of the user who created the role
    pub created_by: String,

    /// Username of the user who last updated the role
    pub updated_by: String,

    /// List of permissions associated with the role
    pub permissions: Vec<String>,
}

/// Represents a role option model used in dropdowns or selection lists.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RoleOptionModel {
    /// label for the role option
    pub label: String,
    /// Value associated with the role option, typically the role's identifier
    pub value: String,
}

impl TryFrom<RoleModel> for crate::api::proto::admin_user::RoleModel {
    type Error = Error;

    fn try_from(val: RoleModel) -> Result<Self> {
        let chrono_utc_created_at = val.created_at.to_utc();
        let system_time_created_at = SystemTime::from(chrono_utc_created_at);
        let created_at = Timestamp::from(system_time_created_at);

        let chrono_utc_updated_at = val.updated_at.to_utc();
        let system_time_updated_at = SystemTime::from(chrono_utc_updated_at);
        let updated_at = Timestamp::from(system_time_updated_at);

        let model: Self =
            Self {
                id: val.id,
                name: val.name,
                identifier: val.identifier,
                created_at: Option::from(created_at),
                updated_at: Option::from(updated_at),
                created_by: val.created_by,
                updated_by: val.updated_by,
                permissions: val.permissions,
            };

        Ok(model)
    }
}
impl TryFrom<Object> for RoleModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<Self> {
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

                    for array in v {
                        arr.push(array.as_string());
                    }
                    arr
                }
                _ => Vec::new(),
            },
            None => Vec::new(),
        };

        Ok(Self {
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

/// Represents a model for creating a new role.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatableRole {
    /// Name of the role to be created
    pub name: String,

    /// identifier for the role, used for API access
    pub identifier: String,

    /// Username of the user creating the role
    pub logged_in_username: String,

    /// List of permissions to be assigned to the role
    pub permissions: Vec<String>,
}


/// Represents a model for updating an existing role.
#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatableRoleModel {

    /// Unique identifier for the role to be updated
    pub id: String,
    /// Name of the role to be updated
    pub name: String,

    /// identifier for the role, used for API access
    pub logged_in_username: String,

    /// List of permissions to be assigned to the role
    pub permissions: Vec<String>,
}

/// Represents a model for updating the identifier of an existing role.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutRoleIdentifierModel {
    /// Unique identifier for the role to be updated
    pub id: String,

    /// New identifier for the role, used for API access
    pub identifier: String,

    /// Username of the user updating the role identifier
    pub logged_in_username: String,
}


/// Represents a paginated response for roles.
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct RolePagination {
    /// List of roles in the current page
    pub data: Vec<RoleModel>,

    /// Pagination information for the response
    pub pagination: Pagination,
}
