use serde::{Deserialize, Serialize};
use surrealdb::types::Datetime;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Datetime,
    pub created_by: String,
    pub updated_at: Datetime,
    pub updated_by: String,
    pub deleted_at: Option<Datetime>,
    pub deleted_by: Option<String>,
}



#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub processing_user: String,
}



#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for UserModel {
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
        let created_at = match obj.remove("created_at") {
            Some(surrealdb::types::Value::Datetime(v)) => v,
            _ => Datetime::default(),
        };
        let created_by = match obj.remove("created_by") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        let updated_at = match obj.remove("updated_at") {
            Some(surrealdb::types::Value::Datetime(v)) => v,
            _ => Datetime::default(),
        };
        let updated_by = match obj.remove("updated_by") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };
        let deleted_at = match obj.remove("deleted_at") {
            Some(surrealdb::types::Value::Datetime(v)) => Some(v),
            _ => None,
        };
        let deleted_by = match obj.remove("deleted_by") {
            Some(surrealdb::types::Value::String(v)) => Some(v),
            _ => None,
        };
        Ok(UserModel { id, name, email, password, created_at, created_by, updated_at, updated_by, deleted_at, deleted_by })
    }
}
