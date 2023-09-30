use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};

use super::Pagination;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub content: String,
    pub created_at: Datetime,
    pub updated_at: Datetime,
    pub created_by: String,
    pub updated_by: String,
}

impl TryFrom<Object> for PageModel {
    type Error = Error;
    fn try_from(val: Object) -> Result<PageModel> {
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
        let name = match val.get("name") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let identifier = match val.get("identifier") {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        let content = match val.get("content") {
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

        Ok(PageModel {
            id,
            name,
            identifier,
            content,
            created_at,
            updated_at,
            created_by,
            updated_by,
        })
    }
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct PagePagination {
    pub data: Vec<PageModel>,
    pub pagination: Pagination,
}


#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct CreatablePageModel {
    pub name: String,
    pub identifier: String,
    pub content: String,
    pub logged_in_username: String,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct UpdatablePageModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub content: String,
    pub logged_in_username: String,
}
