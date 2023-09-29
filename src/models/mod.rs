use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Object, Value};

pub mod admin_user_model;
pub mod component_model;
pub mod field_model;
pub mod page_model;
pub mod role_model;

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct Pagination {
    pub total: i64,
    pub per_page: i64,
    pub current_page: i64,
    pub from: i64,
    pub to: i64,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub next_page_number: i64,
    pub previous_page_number: i64,
}

#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ModelCount {
    pub total: i64,
}

impl TryFrom<Object> for ModelCount {
    type Error = Error;
    fn try_from(val: Object) -> Result<ModelCount> {
        let count = match val.get("count") {
            Some(val) => {
                let value = match val.clone() {
                    Value::Number(v) => v,
                    _ => surrealdb::sql::Number::Int(0),
                };
                value
            }
            None => surrealdb::sql::Number::Int(0),
        };

        let count = count.as_int();

        Ok(ModelCount { total: count })
    }
}
