use crate::error::{Result};
use surrealdb::types::{Datetime, RecordIdKey, Value};


pub mod user;
pub mod admin_user;



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

    /// get reference to a field if it's an object
    fn get(&self, field: &str) -> Option<&Value>;
}

fn format_record_id(id: &surrealdb::types::RecordId) -> String {
    let table = id.table.as_str();
    let key = match &id.key {
        RecordIdKey::String(s) => s.to_string(),
        RecordIdKey::Number(n) => n.to_string(),
        RecordIdKey::Uuid(u) => u.to_string(),
        _ => format!("{:?}", id.key),
    };
    format!("{}:{}", table, key)
}

impl BaseModel for &Value {
    fn get_id(&self) -> Result<String> {
        match self {
            Value::RecordId(id) => Ok(format_record_id(id)),
            Value::Object(obj) => match obj.get("id") {
                Some(Value::RecordId(id)) => Ok(format_record_id(id)),
                _ => Ok(String::new()),
            },
            _ => Ok(String::new()),
        }
    }

    fn get_string(&self) -> Result<String> {
        match self {
            Value::String(s) => Ok(s.to_string()),
            _ => Ok(String::new()),
        }
    }

    fn get_datetime(&self) -> Result<Datetime> {
        match self {
            Value::Datetime(dt) => Ok(dt.clone()),
            _ => Ok(Datetime::default()),
        }
    }

    fn get_bool(&self) -> Result<bool> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Ok(false),
        }
    }

    fn get_int(&self) -> Result<i64> {
        match self {
            Value::Number(n) => Ok(n.to_int().unwrap()),
            _ => Ok(0),
        }
    }

    fn get_float(&self) -> Result<f64> {
        match self {
            Value::Number(n) => Ok(n.to_f64().unwrap()),
            _ => Ok(0.0),
        }
    }

    fn get(&self, field: &str) -> Option<&Value> {
        match self {
            Value::Object(obj) => obj.get(field),
            _ => None,
        }
    }
}

impl BaseModel for Option<&Value> {
    fn get_id(&self) -> Result<String> {
        match self {
            Some(val) => val.get_id(),
            None => Ok(String::new()),
        }
    }
    fn get_string(&self) -> Result<String> {
        match self {
            Some(val) => val.get_string(),
            None => Ok(String::new()),
        }
    }

    fn get_datetime(&self) -> Result<Datetime> {
        match self {
            Some(val) => val.get_datetime(),
            None => Ok(Datetime::default()),
        }
    }

    fn get_bool(&self) -> Result<bool> {
        match self {
            Some(val) => val.get_bool(),
            None => Ok(false),
        }
    }

    fn get_int(&self) -> Result<i64> {
        match self {
            Some(val) => val.get_int(),
            None => Ok(0),
        }
    }

    fn get_float(&self) -> Result<f64> {
        match self {
            Some(val) => val.get_float(),
            None => Ok(0.0),
        }
    }

    fn get(&self, field: &str) -> Option<&Value> {
        match self {
            Some(val) => val.get(field),
            None => None,
        }
    }
}
