use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Value::Bool;
use surrealdb::sql::{Datetime, Object, Value};

pub mod admin_user_model;
pub mod asset_model;
pub mod collection_model;
// pub mod component_model;
// pub mod model_model;
// pub mod page_model;
// pub mod password_rest_model;
pub mod role_model;
// pub mod setting_model;
pub mod token_claim_model;
pub mod validation_error;
pub mod content_model;

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

pub trait BaseModel {
    fn get_id(&self) -> Result<String>;
    fn get_string(&self) -> Result<String>;
    fn get_datetime(&self) -> Result<Datetime>;
    fn get_bool(&self) -> Result<bool>;
    // fn get_int(&self) -> Result<i64>;

    // fn get_array<T>(&self) -> Result<Vec<T>>;
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
            Some(val) => match val.clone() {
                Bool(v) => v,
                _ => false,
            },
            None => false,
        };

        Ok(value)
    }

    // fn get_int(&self) -> Result<i64> {
    //     let value = match self.to_owned() {
    //         Some(val) => match val.clone() {
    //             Number(v) => v.as_int(),
    //             _ => 0,
    //         },
    //         None => 0,
    //     };
    //
    //     Ok(value)
    // }

    // fn get_array<T>(&self) -> Result<Vec<T>> where T : TryFrom<Object> {
    //     let value = match self.to_owned() {
    //         Some(val) => match val.clone() {
    //             Value::Array(v) => {
    //                 let mut arr: Vec<T> = Vec::new();
    //
    //                 for array in v.into_iter() {
    //                     let object = match array.clone() {
    //                         Value::Object(v) => v,
    //                         _ => Object::default(),
    //                     };
    //                     let field_data_option: T = object.try_into()?;
    //
    //                     arr.push(field_data_option)
    //                 }
    //                 arr
    //             }
    //             _ => Vec::new(),
    //         },
    //         None => Vec::new(),
    //     };
    //
    //     Ok(value)
    // }
}

impl TryFrom<Object> for ModelCount {
    type Error = Error;
    fn try_from(val: Object) -> Result<ModelCount> {
        let count = match val.get("count") {
            Some(val) => match val.clone() {
                Value::Number(v) => v,
                _ => surrealdb::sql::Number::Int(0),
            },
            None => surrealdb::sql::Number::Int(0),
        };

        let count = count.as_int();

        Ok(ModelCount { total: count })
    }
}
