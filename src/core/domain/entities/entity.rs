use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EntityModel {
    pub id: String,
    pub name: String,
    pub identifier: String,
    pub data_type: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorableEntity {
    pub name: String,
    pub identifier: String,
    pub data_type: String,
}

#[cfg(feature = "ssr")]
impl TryFrom<surrealdb::types::Object> for EntityModel {
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

        let name = match obj.remove("name") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };

        let identifier = match obj.remove("identifier") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };

        let data_type = match obj.remove("data_type") {
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };

        let created_at = match obj.remove("created_at") {
            Some(surrealdb::types::Value::Datetime(v)) => v.to_string(),
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };

        let updated_at = match obj.remove("updated_at") {
            Some(surrealdb::types::Value::Datetime(v)) => v.to_string(),
            Some(surrealdb::types::Value::String(v)) => v,
            _ => String::new(),
        };

        let deleted_at = match obj.remove("deleted_at") {
            Some(surrealdb::types::Value::Datetime(v)) => Some(v.to_string()),
            Some(surrealdb::types::Value::String(v)) => Some(v),
            _ => None,
        };

        Ok(EntityModel {
            id,
            name,
            identifier,
            data_type,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}
