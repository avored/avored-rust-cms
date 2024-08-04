use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object};

use super::{BaseModel, Pagination};

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AssetModel {
    pub id: String,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_type: String,
    pub information: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for AssetModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<AssetModel> {
        let id = val.get("id").get_id()?;
        let file_name = val.get("file_name").get_string()?;
        let file_path = val.get("file_path").get_string()?;
        let file_size = val.get("file_size").get_int()?;
        let file_type = val.get("file_type").get_string()?;
        let information = val.get("information").get_string()?;
        let created_at = val.get("created_at").get_datetime()?;
        let updated_at = val.get("updated_at").get_datetime()?;
        let created_by = val.get("created_by").get_string()?;
        let updated_by = val.get("updated_by").get_string()?;

        Ok(AssetModel {
            id,
            file_name,
            file_path,
            file_size,
            file_type,
            information,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct AssetPagination {
    pub data: Vec<AssetModel>,
    pub pagination: Pagination,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct CreatableAssetModel {
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub file_type: String,
    pub information: String,
    pub logged_in_username: String,
}