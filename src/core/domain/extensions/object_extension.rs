use surrealdb::types::{Datetime, Object};

use crate::error::Result;

pub trait ObjectExtension {
    fn get_id(&self, key: &str) -> Result<String>;

    fn get_string(&self, key: &str) -> Result<String>;
    
    fn get_datetime(&self, key: &str) -> Result<Datetime>;

    fn get_optional_string(&self, key: &str) -> Result<Option<String>>;
    
    fn get_optional_datetime(&self, key: &str) -> Result<Option<Datetime>>;

}

impl ObjectExtension for Object {

    fn get_id(&self, key: &str) -> Result<String> {
        let value = match self.get(key) {
            Some(surrealdb::types::Value::RecordId(v)) => match &v.key {
                surrealdb::types::RecordIdKey::String(k) => k.to_string(),
                _ => format!("{:?}", v.key),
            },
            Some(surrealdb::types::Value::String(v)) => v.to_string(),
            _ => String::new(),
        };
        Ok(value)
    }

    fn get_string(&self, key: &str) -> Result<String> {
        let value = match self.get(key) {
            Some(surrealdb::types::Value::String(v)) => v.to_string(),
            _ => String::new(),
        };

        Ok(value)   
    }

    fn get_datetime(&self, key: &str) -> Result<Datetime> {
        let value = match self.get(key) {
            Some(surrealdb::types::Value::Datetime(v)) => *v,
            _ => Datetime::MIN_UTC,
        };

        Ok(value)
    }

     fn get_optional_string(&self, key: &str) -> Result<Option<String>> {
        let value = match self.get(key) {
            Some(surrealdb::types::Value::String(v)) => Some(v.to_string()),
            _ => None,
        };


        Ok(value)
    }
    fn get_optional_datetime(&self, key: &str) -> Result<Option<Datetime>> {
        let value = match self.get(key) {
            Some(surrealdb::types::Value::Datetime(v)) => Some(*v),
            _ => None,
        };

        Ok(value)
    }
}
