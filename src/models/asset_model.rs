use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;

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
        let file_name = match val.get("file_name") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let file_path = match val.get("file_path") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let file_size = match val.get("file_size") {
            Some(val) => match val.clone() {
                Value::Number(v) => v.as_int(),
                _ => 0,
            },
            None => 0,
        };

        let file_type = match val.get("file_type") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };
        let information = match val.get("information") {
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
        let updated_at = match val.get("updated_at") {
            Some(val) => match val.clone() {
                Value::Datetime(v) => v,
                _ => Datetime::default(),
            },
            None => Datetime::default(),
        };

        let created_by = match val.get("created_by") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let updated_by = match val.get("updated_by") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

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