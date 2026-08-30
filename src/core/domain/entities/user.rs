use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String
}



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableUser {
    pub name: String,
    pub email: String,
    pub password: String
}



#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for User {
    type Error = crate::error::Error;
    fn try_from(mut obj: surrealdb::types::Object) -> Result<Self, Self::Error> {
        let id = match obj.remove("id") {
            Some(surrealdb::types::Value::RecordId(v)) => match v.key {
                surrealdb::types::RecordIdKey::String(k) => format!("{}:{}", v.table, k),
                _ => format!("{}:{:?}", v.table, v.key),
            },
                Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        let email = match obj.remove("email") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        let name = match obj.remove("name") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        let password = match obj.remove("password") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        Ok(User { id, name, email, password })
    }
}

