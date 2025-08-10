use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Value::{Bool, Number};
use surrealdb::sql::{Datetime, Object, Value};


/// admin user module
pub mod admin_user_model;

/// asset module
pub mod asset_model;

/// collection module
pub mod collection_model;

/// content module
pub mod content_model;

/// ldap config module
pub mod ldap_config_model;

/// password reset module
pub mod password_rest_model;

/// role module
pub mod role_model;

/// security alert module
pub mod security_alert_model;

/// security audit module
pub mod security_audit_model;

/// setting module
pub mod setting_model;

/// token claim module
pub mod token_claim_model;

/// validation module
pub mod validation_error;


/// Pagination struct
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct Pagination {
    /// The total number of items in the collection
    pub total: i64,

    /// The number of items per page
    pub per_page: i64,

    /// The current page number
    pub current_page: i64,

    /// The from number of collection
    pub from: i64,

    /// The to number of collection
    pub to: i64,

    /// Whether there is a next page
    pub has_next_page: bool,

    /// Whether there is a previous page
    pub has_previous_page: bool,

    /// The next page number
    pub next_page_number: i64,

    /// The previous page number
    pub previous_page_number: i64,
}

/// `ModelCount` struct
#[derive(Serialize, Debug, Deserialize, Clone, Default)]
pub struct ModelCount {

    /// The total count of items in the model
    pub total: i64,
}

/// `BaseModel` trait
pub trait BaseModel {
    /// get ID from Value
    fn get_id(&self) -> Result<String>;

    /// get String from Value
    fn get_string(&self) -> Result<String>;

    /// get Datetime from Value
    fn get_datetime(&self) -> Result<Datetime>;

    /// get Bool from Value
    fn get_bool(&self) -> Result<bool>;

    /// get Int from Value
    fn get_int(&self) -> Result<i64>;

    /// get Float from Value
    fn get_float(&self) -> Result<f64>;

    // get Array from Value  fix this as we not able to returns an array of generic types
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
                _ => String::new(),
            },
            None => String::new(),
        };

        Ok(value)
    }
    fn get_string(&self) -> Result<String> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Value::Strand(v) => v.as_string(),
                _ => String::new(),
            },
            None => String::new(),
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

    fn get_float(&self) -> Result<f64> {
        let value = match self.to_owned() {
            Some(val) => match val.clone() {
                Number(v) => v.as_float(),
                _ => 0.00,
            },
            None => 0.00,
        };

        Ok(value)
    }

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
    fn try_from(val: Object) -> Result<Self> {
        let count = match val.get("count") {
            Some(val) => match val.clone() {
                Number(v) => v,
                _ => surrealdb::sql::Number::Int(0),
            },
            None => surrealdb::sql::Number::Int(0),
        };

        let count = count.as_int();

        Ok(Self { total: count })
    }
}
