use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Object, Value};
use surrealdb::sql::Value::{Bool, Number};

pub mod admin_user_model;
pub mod component_model;
pub mod page_model;
pub mod role_model;
pub mod asset_model;
pub mod token_claim_model;
pub mod validation_error;
pub mod password_rest_model;
pub mod setting_model;
pub mod model_model;

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

pub trait  BaseModel {
    fn get_id(&self) -> Result<String>;
    fn get_string(&self) -> Result<String>;
    fn get_datetime(&self) -> Result<Datetime>;
    fn get_bool(&self) -> Result<bool>;
    fn get_int(&self) -> Result<i64>;
}
impl BaseModel for Option<&Value> {

    fn get_id(&self) -> Result<String> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Value::Thing(v) => {
                    let id = v.id;
                    id.to_string()
                }
                _ => String::from(""),
            },
            None => String::from(""),
        };

        Ok(value)
    }
    fn get_string(&self) -> Result<String> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::from(""),
            },
            None => String::from(""),
        };

        Ok(value)
    }
    fn get_datetime(&self) -> Result<Datetime> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Value::Datetime(v) => v,
                _ => Datetime::default(),
            },
            None => Datetime::default(),
        };

        Ok(value)
    }

    fn get_bool(&self) -> Result<bool> {
        let value = match self.to_owned() {
            Some(val) => {
                match val.clone() {
                    Bool(v) => v,
                    _ => false,
                }
            }
            None => false,
        };

        Ok(value)
    }

    fn get_int(&self) -> Result<i64> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Number(v) => v.as_int(),
                _ => 0,
            },
            None => 0,
        };

        Ok(value)
    }
}

impl TryFrom<Object> for ModelCount {
    type Error = Error;
    fn try_from(val: Object) -> Result<ModelCount> {
        let count = match val.get("count") {
            Some(val) => {
                
                match val.clone() {
                    Value::Number(v) => v,
                    _ => surrealdb::sql::Number::Int(0),
                }
            }
            None => surrealdb::sql::Number::Int(0),
        };

        let count = count.as_int();

        Ok(ModelCount { total: count })
    }
}
