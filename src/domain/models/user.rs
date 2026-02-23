use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use surrealdb::types::{RecordIdKey, Value};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
}

impl TryFrom<&Value> for User {
    type Error = Error;
    fn try_from(val: &Value) -> Result<Self> {
        let id = val.get_id()?;
        let email = val.get("email").get_string()?;
        let password_hash = val.get("password_hash").get_string()?;

        Ok(Self {
            id,
            email,
            password_hash,
        })
    }
}

pub trait BaseModel {
    /// get ID from Value
    fn get_id(&self) -> Result<String>;

    /// get String from Value
    fn get_string(&self) -> Result<String>;

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
    fn get(&self, field: &str) -> Option<&Value> {
        match self {
            Some(val) => val.get(field),
            None => None,
        }
    }
}
