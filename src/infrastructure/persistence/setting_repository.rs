use std::collections::BTreeMap;
use std::sync::Arc;

use surrealdb::types::{Object, RecordIdKey, Value};
use surrealdb_rpc::QueryResult;

use crate::core::domain::constants::SETTINGS_TABLE_NAME;
use crate::core::domain::entities::setting::{SettingModel, StorableSetting};
use crate::core::domain::repositories::setting_repository::SettingRepository;
use crate::error::{Error, Result};
use crate::infrastructure::persistence::into_iter_objects;
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;

#[derive(Clone)]
pub struct SettingRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl SettingRepositoryImpl {
    pub fn new(db: Arc<AvoRedDatabaseProvider>) -> Self {
        Self {
            database_provider: db,
        }
    }
}

#[async_trait::async_trait]
impl SettingRepository for SettingRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<SettingModel>> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!("SELECT * FROM {SETTINGS_TABLE_NAME};");
        let response = datastore.execute(&sql, session, None).await?;
        let mut list = Vec::new();
        for object_res in into_iter_objects(response)? {
            let object = object_res?;
            list.push(map_object_to_setting(&object)?);
        }
        Ok(list)
    }

    async fn find_by_group(&self, group: &str) -> Result<Vec<SettingModel>> {
        let (datastore, session) = &self.database_provider.db;
        let sql = format!("SELECT * FROM {SETTINGS_TABLE_NAME} WHERE setting_group = $group;");
        let values = BTreeMap::from([("group".to_string(), Value::String(group.to_string()))]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        let mut list = Vec::new();
        for object_res in into_iter_objects(response)? {
            let object = object_res?;
            list.push(map_object_to_setting(&object)?);
        }
        Ok(list)
    }

    async fn find_by_identifier(&self, identifier: &str) -> Result<Option<SettingModel>> {
        let (datastore, session) = &self.database_provider.db;
        let sql =
            format!("SELECT * FROM {SETTINGS_TABLE_NAME} WHERE identifier = $identifier LIMIT 1;");
        let values = BTreeMap::from([(
            "identifier".to_string(),
            Value::String(identifier.to_string()),
        )]);
        let response = datastore
            .execute(&sql, session, Some(values.into()))
            .await?;
        for object_res in into_iter_objects(response)? {
            let object = object_res?;
            return Ok(Some(map_object_to_setting(&object)?));
        }
        Ok(None)
    }

    async fn upsert(&self, setting: StorableSetting) -> Result<SettingModel> {
        let (datastore, session) = &self.database_provider.db;
        let existing = self.find_by_identifier(&setting.identifier).await?;

        if let Some(existing) = existing {
            let sql = format!(
                "UPDATE {} SET value = $value, setting_group = $setting_group, updated_at = time::now() RETURN AFTER;",
                existing.id
            );
            let values = BTreeMap::from([
                ("value".to_string(), Value::String(setting.value)),
                (
                    "setting_group".to_string(),
                    Value::String(setting.setting_group),
                ),
            ]);
            let response = datastore
                .execute(&sql, session, Some(values.into()))
                .await?;
            let object = first_object(response, "Setting update returned no record")?;
            map_object_to_setting(&object)
        } else {
            let sql = format!(
                "CREATE {SETTINGS_TABLE_NAME} SET identifier = $identifier, value = $value, setting_group = $setting_group, created_at = time::now(), updated_at = time::now() RETURN AFTER;"
            );
            let values = BTreeMap::from([
                ("identifier".to_string(), Value::String(setting.identifier)),
                ("value".to_string(), Value::String(setting.value)),
                (
                    "setting_group".to_string(),
                    Value::String(setting.setting_group),
                ),
            ]);
            let response = datastore
                .execute(&sql, session, Some(values.into()))
                .await?;
            let object = first_object(response, "Setting create returned no record")?;
            map_object_to_setting(&object)
        }
    }
}

fn first_object(response: Vec<QueryResult>, missing_msg: &'static str) -> Result<Object> {
    into_iter_objects(response)?
        .next()
        .ok_or_else(|| Error::Generic(missing_msg.to_string()))?
}

fn map_object_to_setting(object: &Object) -> Result<SettingModel> {
    let id = match object.get("id") {
        Some(Value::RecordId(record_id)) => match &record_id.key {
            RecordIdKey::String(s) => format!("{}:{}", record_id.table, s),
            RecordIdKey::Number(n) => format!("{}:{}", record_id.table, n),
            RecordIdKey::Uuid(u) => format!("{}:{}", record_id.table, u),
            _ => format!("{}:{:?}", record_id.table, record_id.key),
        },
        Some(Value::String(s)) => s.clone(),
        _ => return Err(Error::Generic("Setting record id is missing".into())),
    };

    let identifier = object
        .get("identifier")
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default();

    let value = object
        .get("value")
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default();

    let setting_group = object
        .get("setting_group")
        .and_then(|v| match v {
            Value::String(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_default();

    let created_at = match object.get("created_at") {
        Some(Value::Datetime(dt)) => dt.to_string(),
        Some(Value::String(s)) => s.clone(),
        _ => chrono::Utc::now().to_rfc3339(),
    };

    let updated_at = match object.get("updated_at") {
        Some(Value::Datetime(dt)) => dt.to_string(),
        Some(Value::String(s)) => s.clone(),
        _ => chrono::Utc::now().to_rfc3339(),
    };

    Ok(SettingModel {
        id,
        identifier,
        value,
        setting_group,
        created_at,
        updated_at,
    })
}
