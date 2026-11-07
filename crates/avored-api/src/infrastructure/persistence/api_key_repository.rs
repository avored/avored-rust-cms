use std::collections::BTreeMap;
use std::sync::Arc;

use serde_json::Value as JsonValue;
use surrealdb::types::{Number, Object, RecordIdKey, Value};

use crate::core::domain::constants::API_KEYS_TABLE_NAME;
use crate::core::domain::entities::api_key::{
    ApiKeyModel, ApiKeyPermission, StorableApiKey, UpdatableApiKey,
};
use crate::core::domain::repositories::api_key_repository::ApiKeyRepository;
use crate::error::{Error, Result};
use crate::infrastructure::persistence::{into_iter_objects, RecordIdExtension};
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;

#[derive(Clone)]
pub struct ApiKeyRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl ApiKeyRepositoryImpl {
    pub fn new(db: Arc<AvoRedDatabaseProvider>) -> Self {
        Self {
            database_provider: db,
        }
    }
}

#[async_trait::async_trait]
impl ApiKeyRepository for ApiKeyRepositoryImpl {
    async fn create(&self, storable: StorableApiKey) -> Result<ApiKeyModel> {
        let (datastore, session) = &self.database_provider.db;

        let sql = format!(
            "CREATE {API_KEYS_TABLE_NAME} SET \
             name = $name, key_prefix = $key_prefix, key_hash = $key_hash, \
             permissions = $permissions, created_at = time::now(), \
             created_by = $created_by, updated_at = time::now(), \
             updated_by = $updated_by RETURN AFTER;"
        );

        let values = BTreeMap::from([
            ("name".to_string(), Value::String(storable.name)),
            ("key_prefix".to_string(), Value::String(storable.key_prefix)),
            ("key_hash".to_string(), Value::String(storable.key_hash)),
            (
                "permissions".to_string(),
                json_to_surreal_value(serde_json::to_value(storable.permissions)?),
            ),
            (
                "created_by".to_string(),
                Value::String(storable.created_by.clone()),
            ),
            ("updated_by".to_string(), Value::String(storable.created_by)),
        ]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let object = first_object(response, "API key create returned no record")?;
        api_key_from_object(object)
    }

    async fn find_by_id(&self, id: &str) -> Result<ApiKeyModel> {
        let (datastore, session) = &self.database_provider.db;

        let sql = format!("SELECT * FROM {API_KEYS_TABLE_NAME} WHERE id = $id LIMIT 1;");
        let values = BTreeMap::from([(
            "id".to_string(),
            Value::RecordId(id.to_record_id(API_KEYS_TABLE_NAME)),
        )]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        first_object_or_not_found(response, &format!("API key '{id}' was not found"))
            .and_then(api_key_from_object)
    }

    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<ApiKeyModel>> {
        let (datastore, session) = &self.database_provider.db;

        let sql =
            format!("SELECT * FROM {API_KEYS_TABLE_NAME} WHERE key_hash = $key_hash LIMIT 1;");
        let values =
            BTreeMap::from([("key_hash".to_string(), Value::String(key_hash.to_string()))]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        match into_iter_objects(response)?.next() {
            Some(Ok(object)) => Ok(Some(api_key_from_object(object)?)),
            Some(Err(err)) => Err(err),
            None => Ok(None),
        }
    }

    async fn list(&self, page: u64, limit: u64) -> Result<(Vec<ApiKeyModel>, u64)> {
        let (datastore, session) = &self.database_provider.db;

        let page = page.max(1);
        let limit = limit.clamp(1, 100);
        let skip = page.saturating_sub(1).saturating_mul(limit);

        let sql = format!(
            "SELECT * FROM {API_KEYS_TABLE_NAME} ORDER BY created_at DESC \
             LIMIT $limit START $skip;"
        );
        let values = BTreeMap::from([
            (
                "limit".to_string(),
                Value::Number(Number::Int(limit.min(i64::MAX as u64) as i64)),
            ),
            (
                "skip".to_string(),
                Value::Number(Number::Int(skip.min(i64::MAX as u64) as i64)),
            ),
        ]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let mut keys = Vec::new();
        for object in into_iter_objects(response)? {
            keys.push(api_key_from_object(object?)?);
        }

        let count_sql = format!("SELECT count() AS total FROM {API_KEYS_TABLE_NAME} GROUP ALL;");
        let count_response = datastore.execute(&count_sql, session, None).await?;
        let total = match into_iter_objects(count_response)?.next() {
            Some(Ok(obj)) => read_count_from_object(&obj)?,
            _ => 0,
        };

        Ok((keys, total))
    }

    async fn update(&self, id: &str, updatable: UpdatableApiKey) -> Result<ApiKeyModel> {
        let (datastore, session) = &self.database_provider.db;

        let sql = format!(
            "UPDATE {API_KEYS_TABLE_NAME} SET \
             name = $name, permissions = $permissions, \
             updated_at = time::now(), updated_by = $updated_by \
             WHERE id = $id RETURN AFTER;"
        );

        let values = BTreeMap::from([
            (
                "id".to_string(),
                Value::RecordId(id.to_record_id(API_KEYS_TABLE_NAME)),
            ),
            ("name".to_string(), Value::String(updatable.name)),
            (
                "permissions".to_string(),
                json_to_surreal_value(serde_json::to_value(updatable.permissions)?),
            ),
            (
                "updated_by".to_string(),
                Value::String(updatable.updated_by),
            ),
        ]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        first_object_or_not_found(response, &format!("API key '{id}' was not found"))
            .and_then(api_key_from_object)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let (datastore, session) = &self.database_provider.db;

        let sql = format!("DELETE {API_KEYS_TABLE_NAME} WHERE id = $id RETURN BEFORE;");
        let values = BTreeMap::from([(
            "id".to_string(),
            Value::RecordId(id.to_record_id(API_KEYS_TABLE_NAME)),
        )]);

        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        match into_iter_objects(response)?.next() {
            Some(Ok(_)) => Ok(true),
            Some(Err(err)) => Err(err),
            None => Err(Error::NotFound(format!("API key '{id}' was not found"))),
        }
    }
}

fn first_object(responses: Vec<surrealdb_rpc::QueryResult>, message: &str) -> Result<Object> {
    into_iter_objects(responses)?
        .next()
        .ok_or_else(|| Error::Generic(message.to_string()))?
}

fn first_object_or_not_found(
    responses: Vec<surrealdb_rpc::QueryResult>,
    message: &str,
) -> Result<Object> {
    match into_iter_objects(responses)?.next() {
        Some(Ok(object)) => Ok(object),
        Some(Err(error)) => Err(error),
        None => Err(Error::NotFound(message.to_string())),
    }
}

fn read_count_from_object(object: &Object) -> Result<u64> {
    let total = object
        .get("total")
        .or_else(|| object.get("count"))
        .and_then(|value| match value {
            Value::Number(number) => (*number).into_int().ok(),
            _ => None,
        })
        .unwrap_or(0);
    Ok(u64::try_from(total).unwrap_or(0))
}

fn required_string(object: &Object, field: &str) -> Result<String> {
    match object.get(field) {
        Some(Value::String(value)) if !value.is_empty() => Ok(value.to_string()),
        Some(Value::RecordId(value)) if field == "id" => match &value.key {
            RecordIdKey::String(key) => Ok(key.clone()),
            key => Ok(format!("{key:?}")),
        },
        _ => Err(Error::Generic(format!(
            "ApiKey persistence field '{field}' is missing or invalid"
        ))),
    }
}

fn format_datetime(value: Option<&Value>) -> String {
    match value {
        Some(Value::Datetime(dt)) => dt.to_string(),
        Some(Value::String(s)) => s.clone(),
        _ => chrono::Utc::now().to_rfc3339(),
    }
}

fn api_key_from_object(object: Object) -> Result<ApiKeyModel> {
    let id = required_string(&object, "id")?;
    let name = required_string(&object, "name")?;
    let key_prefix = required_string(&object, "key_prefix")?;
    let key_hash = required_string(&object, "key_hash")?;

    let permissions_value = object
        .get("permissions")
        .ok_or_else(|| Error::Generic("ApiKey permissions field is missing".to_string()))?
        .clone()
        .into_json_value();
    let permissions: Vec<ApiKeyPermission> = serde_json::from_value(permissions_value)?;

    let created_at = format_datetime(object.get("created_at"));
    let created_by = required_string(&object, "created_by").unwrap_or_default();
    let updated_at = format_datetime(object.get("updated_at"));
    let updated_by = required_string(&object, "updated_by").unwrap_or_default();

    Ok(ApiKeyModel {
        id,
        name,
        key_prefix,
        key_hash,
        permissions,
        created_at,
        created_by,
        updated_at,
        updated_by,
    })
}

fn json_to_surreal_value(value: JsonValue) -> Value {
    match value {
        JsonValue::Null => Value::None,
        JsonValue::Bool(value) => Value::Bool(value),
        JsonValue::Number(value) => {
            if let Some(integer) = value.as_i64() {
                Value::Number(Number::Int(integer))
            } else if let Some(float) = value.as_f64() {
                Value::Number(Number::Float(float))
            } else {
                Value::String(value.to_string())
            }
        }
        JsonValue::String(value) => Value::String(value),
        JsonValue::Array(items) => {
            Value::Array(items.into_iter().map(json_to_surreal_value).collect())
        }
        JsonValue::Object(items) => {
            let mut object = Object::new();
            for (key, value) in items {
                object.insert(key, json_to_surreal_value(value));
            }
            Value::Object(object)
        }
    }
}
