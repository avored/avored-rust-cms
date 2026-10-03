use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::types::{Number, Object, Value};

use crate::core::domain::constants::{ATTRIBUTES_TABLE_NAME, ENTITIES_TABLE_NAME};
use crate::core::domain::entities::entity::{
    EntityModel, StorableEntity, StorableEntityAttribute, UpdableIdentifierEntity, UpdatableEntity, UpdatableEntityAttribute,
};
use crate::core::domain::entities::modal_count::ModalCount;
use crate::core::domain::entities::AttributeModel;
use crate::core::domain::repositories::EntityRepository;
use crate::error::{Error, Result};
use crate::infrastructure::persistence::into_iter_objects;
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;

#[derive(Clone)]
pub struct EntityRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl EntityRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

fn serde_json_to_surreal_value(value: serde_json::Value) -> Value {
    match value {
        serde_json::Value::Null => Value::None,
        serde_json::Value::Bool(v) => Value::Bool(v),
        serde_json::Value::Number(v) => {
            if let Some(i) = v.as_i64() {
                Value::Number(Number::Int(i))
            } else if let Some(f) = v.as_f64() {
                Value::Number(Number::Float(f))
            } else {
                Value::String(v.to_string())
            }
        }
        serde_json::Value::String(v) => Value::String(v),
        serde_json::Value::Array(items) => Value::Array(
            items
                .into_iter()
                .filter(|item| !matches!(item, serde_json::Value::Null))
                .map(serde_json_to_surreal_value)
                .collect(),
        ),
        serde_json::Value::Object(map) => {
            let mut object = Object::new();
            for (key, value) in map {
                if !matches!(value, serde_json::Value::Null) {
                    object.insert(key, serde_json_to_surreal_value(value));
                }
            }
            Value::Object(object)
        }
    }
}

fn object_to_json_value(obj: Object) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, value) in obj {
        map.insert(key, value.into_json_value());
    }
    serde_json::Value::Object(map)
}

#[async_trait::async_trait]
impl EntityRepository for EntityRepositoryImpl {
    async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "
                CREATE {} 
                SET 
                    name=$name, 
                    identifier=$identifier, 
                    created_at=time::now(), 
                    created_by=$created_by, 
                    updated_at=time::now(), 
                    updated_by=$updated_by, 
                    deleted_at=NONE;",
            ENTITIES_TABLE_NAME
        );

        let data: BTreeMap<String, Value> = [
            ("name".into(), Value::String(storable_entity.name.into())),
            (
                "identifier".into(),
                Value::String(storable_entity.identifier.into()),
            ),
            (
                "created_by".into(),
                Value::String(storable_entity.logged_in_user_email.clone().into()),
            ),
            (
                "updated_by".into(),
                Value::String(storable_entity.logged_in_user_email.into()),
            ),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No entity returned from insert".to_string())
        })??;

        let entity: EntityModel = result_object.try_into()?;
        Ok(entity)
    }

    async fn find_by_id(&self, id: &str) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: ENTITIES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!(
            "
            SELECT *,
                (SELECT * FROM {} WHERE entity_id = $id AND deleted_at = NONE) AS attributes
            FROM {}
            WHERE id = $id AND deleted_at = NONE;",
            ATTRIBUTES_TABLE_NAME,
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [("id".into(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut it = into_iter_objects(responses)?;
        if let Some(obj_res) = it.next() {
            let obj = obj_res?;
            let model: EntityModel = obj.try_into()?;
            return Ok(model);
        }

        Err(crate::error::Error::NotFound(format!(
            "Entity with ID '{}' not found",
            id
        )))
    }

    async fn find_by_identifier(&self, identifier: &str) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "
            SELECT * 
            FROM {} 
            WHERE identifier = $identifier AND deleted_at = NONE;",
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> =
            [("identifier".into(), Value::String(identifier.into()))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut it = into_iter_objects(responses)?;
        if let Some(obj_res) = it.next() {
            let obj = obj_res?;
            let model: EntityModel = obj.try_into()?;
            return Ok(model);
        }

        Err(crate::error::Error::NotFound(format!(
            "Entity with identifier '{}' not found",
            identifier
        )))
    }

    async fn paginate(&self, page: u64, page_size: u64) -> Result<Vec<EntityModel>> {
        let (datastore, database_session) = &self.database_provider.db;

        let skip = page.saturating_sub(1) * page_size;

        let number_page_size = Number::Int(page_size as i64);
        let number_skip = Number::Int(skip as i64);

        let sql = format!(
            "
            SELECT * 
            FROM {} 
            WHERE deleted_at = NONE 
            LIMIT $limit 
            START $skip;",
            ENTITIES_TABLE_NAME
        );

        let data: BTreeMap<String, Value> = [
            ("limit".into(), Value::Number(number_page_size)),
            ("skip".into(), Value::Number(number_skip)),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let it = into_iter_objects(responses)?;
        let mut list = Vec::new();
        for obj_res in it {
            let obj = obj_res?;
            let model: EntityModel = obj.try_into()?;
            list.push(model);
        }

        Ok(list)
    }

    async fn update(&self, id: &str, storable_entity: UpdatableEntity) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let id_clean = id.trim_start_matches("entities:").to_string();
        let target_record = surrealdb::types::RecordId {
            table: "entities".into(),
            key: surrealdb::types::RecordIdKey::String(id_clean),
        };

        let sql = format!(
            "
            UPDATE {} 
            SET name=$name, updated_at=time::now(), updated_by=$updated_by 
            WHERE id = $id AND deleted_at = NONE;",
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [
            ("id".into(), Value::RecordId(target_record)),
            ("name".into(), Value::String(storable_entity.name.into())),
            (
                "updated_by".into(),
                Value::String(storable_entity.logged_in_user_email.into()),
            ),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No entity returned from update".to_string())
        })??;

        let entity: EntityModel = result_object.try_into()?;
        Ok(entity)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let (datastore, database_session) = &self.database_provider.db;

        // Verify existence
        self.find_by_id(id).await?;

        let target_record = surrealdb::types::RecordId {
            table: ENTITIES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!(
            "
            UPDATE {} 
            SET deleted_at=time::now(), updated_at=time::now() 
            WHERE id = $id;",
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [("id".into(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let _ = into_iter_objects(responses)?;
        Ok(true)
    }

    async fn count(&self) -> Result<ModalCount> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "
            SELECT count(id) 
            FROM {} 
            WHERE deleted_at = NONE 
            GROUP ALL;",
            ENTITIES_TABLE_NAME
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No entity returned from count".to_string())
        })??;

        let count: ModalCount = result_object.try_into()?;
        Ok(count)
    }

    async fn list_options(&self) -> Result<Vec<EntityModel>> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "
            SELECT id, name 
            FROM {} 
            WHERE deleted_at = NONE;",
            ENTITIES_TABLE_NAME
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let it = into_iter_objects(responses)?;
        let mut list = Vec::new();
        for obj_res in it {
            let obj = obj_res?;
            let model: EntityModel = obj.try_into()?;
            list.push(model);
        }

        Ok(list)
    }

    async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierEntity,
    ) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: ENTITIES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!(
            "
            UPDATE {} 
            SET identifier=$identifier, updated_at=time::now(), updated_by=$updated_by 
            WHERE id = $id AND deleted_at = NONE;",
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [
            ("id".into(), Value::RecordId(target_record)),
            (
                "identifier".into(),
                Value::String(updatable_identifier.identifier.into()),
            ),
            (
                "updated_by".into(),
                Value::String(updatable_identifier.logged_in_user_email.into()),
            ),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No entity returned from update identifier".to_string())
        })??;

        let model: EntityModel = result_object.try_into()?;
        Ok(model)
    }

    async fn create_attribute(
        &self,
        storable_attribute: StorableEntityAttribute,
        entity_id: String,
        logged_in_user: String,
    ) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "
                CREATE {} 
                SET 
                    entity_id = $entity_id,
                    name=$name, 
                    identifier=$identifier,
                    data_type=$data_type,
                    field_type=$field_type, 
                    created_at=time::now(), 
                    created_by=$created_by, 
                    updated_at=time::now(), 
                    updated_by=$updated_by, 
                    deleted_at=NONE;",
            ATTRIBUTES_TABLE_NAME
        );

        let data: BTreeMap<String, Value> = [
            ("name".into(), Value::String(storable_attribute.name.into())),
            (
                "identifier".into(),
                Value::String(storable_attribute.identifier.into()),
            ),
            (
                "entity_id".into(),
                Value::RecordId(surrealdb::types::RecordId {
                    table: ENTITIES_TABLE_NAME.into(),
                    key: surrealdb::types::RecordIdKey::String(entity_id),
                }),
            ),
            (
                "data_type".into(),
                Value::String(storable_attribute.data_type.into()),
            ),
            (
                "field_type".into(),
                Value::String(storable_attribute.field_type.into()),
            ),
            ("created_by".into(), Value::String(logged_in_user.clone())),
            ("updated_by".into(), Value::String(logged_in_user)),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No attribute returned from insert".to_string())
        })??;

        let attribute: AttributeModel = result_object.try_into()?;
        Ok(attribute)
    }

    async fn update_attribute(
        &self,
        attribute: UpdatableEntityAttribute,
        logged_in_user: String,
    ) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "SELECT * FROM {} WHERE id = $id AND deleted_at = NONE;",
            ATTRIBUTES_TABLE_NAME
        );
        let attribute_id = match attribute.id {
            Some(id) => id,
            _ => return Err(Error::NotFound(String::from("attribute id not found")))
        };

        let data: BTreeMap<String, Value> = [
            ("id".into(), Value::String(attribute_id)),
            ("name".into(), Value::String(attribute.name.clone())),
            ("identifier".into(), Value::String(attribute.identifier.clone())),
            ("data_type".into(), Value::String(attribute.data_type.clone())),
            ("field_type".into(), Value::String(attribute.field_type.clone())),
            ("updated_by".into(), Value::String(logged_in_user.clone())),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No attribute returned from update".to_string())
        })??;

        let attribute: AttributeModel = result_object.try_into()?;
        Ok(attribute)
    }

    async fn delete_attribute(&self, entity_id: &str) -> Result<bool> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!(
            "DELETE FROM {} WHERE entity_id = $entity_id AND deleted_at = NONE;",
            ATTRIBUTES_TABLE_NAME
        );

        let entity_record_id = surrealdb::types::RecordId {
            table: ENTITIES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(entity_id.to_string()),
        };

        let data: BTreeMap<String, Value> = [
            ("entity_id".into(), Value::RecordId(entity_record_id)),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            crate::error::Error::Generic("No attribute returned from delete".to_string())
        })?;
        
        match result_object {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }

    async fn paginate_collection(
        &self,
        table_name: &str,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<serde_json::Value>, u64)> {
        let (datastore, database_session) = &self.database_provider.db;

        // Sanitize table_name to ensure it is a valid identifier
        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        let skip = page.saturating_sub(1) * page_size;
        let number_page_size = Number::Int(page_size as i64);
        let number_skip = Number::Int(skip as i64);

        let sql = format!(
            "
            SELECT * 
            FROM {} 
            WHERE deleted_at = NONE 
            LIMIT $limit 
            START $skip;",
            table_name
        );

        let data: BTreeMap<String, Value> = [
            ("limit".into(), Value::Number(number_page_size)),
            ("skip".into(), Value::Number(number_skip)),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let it = into_iter_objects(responses)?;
        let mut list = Vec::new();
        for obj_res in it {
            let obj = obj_res?;
            let json_val = serde_json::to_value(&obj)
                .map_err(|e| Error::Generic(e.to_string()))?;
            list.push(json_val);
        }

        let count_sql = format!(
            "
            SELECT count(id) 
            FROM {} 
            WHERE deleted_at = NONE 
            GROUP ALL;",
            table_name
        );

        let count_responses = datastore
            .execute(&count_sql, database_session, None)
            .await?;

        let total = match into_iter_objects(count_responses)?.next() {
            Some(Ok(obj)) => {
                let modal_count: ModalCount = obj.try_into()?;
                modal_count.total
            }
            _ => 0,
        };

        Ok((list, total))
    }

    async fn create_collection_table(&self, table_name: &str) -> Result<()> {
        let (datastore, database_session) = &self.database_provider.db;

        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        let sql = format!("DEFINE TABLE OVERWRITE {} SCHEMALESS;", table_name);

        datastore
            .execute(&sql, database_session, None)
            .await?;

        Ok(())
    }

    async fn fetch_collection_by_id(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<serde_json::Value> {
        let (datastore, database_session) = &self.database_provider.db;

        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        let target_record = surrealdb::types::RecordId {
            table: table_name.to_string().into(),
            key: surrealdb::types::RecordIdKey::String(record_id.to_string()),
        };

        let sql = format!(
            "SELECT * FROM {} WHERE id = $id AND deleted_at = NONE;",
            table_name
        );

        let data: BTreeMap<String, Value> = [("id".into(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut it = into_iter_objects(responses)?;
        if let Some(obj_res) = it.next() {
            let obj = obj_res?;
            return Ok(object_to_json_value(obj));
        }

        Err(Error::NotFound(format!(
            "Collection record '{}' not found in table '{}'",
            record_id, table_name
        )))
    }

    async fn create_collection(
        &self,
        table_name: &str,
        record: serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let (datastore, database_session) = &self.database_provider.db;

        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        fn strip_nulls(value: serde_json::Value) -> serde_json::Value {
            match value {
                serde_json::Value::Object(map) => {
                    let mut filtered = serde_json::Map::new();
                    for (key, child) in map {
                        let cleaned = strip_nulls(child);
                        if !matches!(cleaned, serde_json::Value::Null) {
                            filtered.insert(key, cleaned);
                        }
                    }
                    serde_json::Value::Object(filtered)
                }
                serde_json::Value::Array(items) => serde_json::Value::Array(
                    items
                        .into_iter()
                        .map(strip_nulls)
                        .filter(|value| !matches!(value, serde_json::Value::Null))
                        .collect(),
                ),
                other => other,
            }
        }

        let sanitized = strip_nulls(serde_json::Value::Object(record));
        let mut content_record = sanitized
            .as_object()
            .cloned()
            .unwrap_or_default();

        content_record.remove("deleted_at");
        content_record.remove("deleted_by");

        let mut data = Object::new();
        for (key, value) in content_record {
            if !matches!(value, serde_json::Value::Null) {
                data.insert(key, serde_json_to_surreal_value(value));
            }
        }

        let mut set_clauses = Vec::new();
        let mut bindings: BTreeMap<String, Value> = BTreeMap::new();

        for (index, (key, value)) in data.iter().enumerate() {
            let binding_name = format!("field_{}", index);
            set_clauses.push(format!("{} = ${}", key, binding_name));
            bindings.insert(binding_name, value.clone());
        }

        set_clauses.push("deleted_at = NONE".to_string());
        set_clauses.push("deleted_by = NONE".to_string());

        let sql = format!("CREATE {} SET {};", table_name, set_clauses.join(", "));

        let responses = datastore
            .execute(&sql, database_session, Some(bindings.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            Error::Generic("No record returned from collection insert".to_string())
        })??;

        let json_val = object_to_json_value(result_object);

        Ok(json_val)
    }

    async fn update_collection_by_id(
        &self,
        table_name: &str,
        record_id: &str,
        record: serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let (datastore, database_session) = &self.database_provider.db;

        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        fn strip_nulls(value: serde_json::Value) -> serde_json::Value {
            match value {
                serde_json::Value::Object(map) => {
                    let mut filtered = serde_json::Map::new();
                    for (key, child) in map {
                        let cleaned = strip_nulls(child);
                        if !matches!(cleaned, serde_json::Value::Null) {
                            filtered.insert(key, cleaned);
                        }
                    }
                    serde_json::Value::Object(filtered)
                }
                serde_json::Value::Array(items) => serde_json::Value::Array(
                    items
                        .into_iter()
                        .map(strip_nulls)
                        .filter(|value| !matches!(value, serde_json::Value::Null))
                        .collect(),
                ),
                other => other,
            }
        }

        let sanitized = strip_nulls(serde_json::Value::Object(record));
        let mut content_record = sanitized
            .as_object()
            .cloned()
            .unwrap_or_default();

        content_record.remove("id");
        content_record.remove("deleted_at");
        content_record.remove("deleted_by");

        let target_record = surrealdb::types::RecordId {
            table: table_name.to_string().into(),
            key: surrealdb::types::RecordIdKey::String(record_id.to_string()),
        };

        let mut data = Object::new();
        for (key, value) in content_record {
            if !matches!(value, serde_json::Value::Null) {
                data.insert(key, serde_json_to_surreal_value(value));
            }
        }

        let mut set_clauses = Vec::new();
        let mut bindings: BTreeMap<String, Value> = BTreeMap::new();
        bindings.insert("id".to_string(), Value::RecordId(target_record));

        for (index, (key, value)) in data.iter().enumerate() {
            let binding_name = format!("field_{}", index);
            set_clauses.push(format!("{} = ${}", key, binding_name));
            bindings.insert(binding_name, value.clone());
        }

        let sql = format!(
            "UPDATE {} SET {} WHERE id = $id AND deleted_at = NONE;",
            table_name,
            set_clauses.join(", ")
        );

        let responses = datastore
            .execute(&sql, database_session, Some(bindings.into()))
            .await?;

        let result_object = into_iter_objects(responses)?.next().ok_or_else(|| {
            Error::Generic("No record returned from collection update".to_string())
        })??;

        let json_val = object_to_json_value(result_object);

        Ok(json_val)
    }

    async fn delete_collection_by_id(&self, table_name: &str, record_id: &str) -> Result<bool> {
        let (datastore, database_session) = &self.database_provider.db;

        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!("Invalid table identifier: {}", table_name)));
        }

        let target_record = surrealdb::types::RecordId {
            table: table_name.to_string().into(),
            key: surrealdb::types::RecordIdKey::String(record_id.to_string()),
        };

        let sql = format!(
            "UPDATE {} SET deleted_at = time::now(), updated_at = time::now() WHERE id = $id AND deleted_at = NONE;",
            table_name
        );

        let data: BTreeMap<String, Value> = [("id".into(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let _ = into_iter_objects(responses)?;
        Ok(true)
    }

    async fn list_entities(
        &self,
        table_name: &str,
        page: u64,
        limit: u64,
        filters: &std::collections::HashMap<String, String>,
    ) -> Result<(Vec<serde_json::Value>, u64)> {
        use surrealdb::types::Number;

        let (datastore, database_session) = &self.database_provider.db;

        // Guard: table_name must be a safe identifier.
        if !table_name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(Error::Generic(format!(
                "Invalid table identifier: {}",
                table_name
            )));
        }

        // Validate page / limit to prevent arithmetic overflow.
        let limit = limit.min(100).max(1);
        let page = page.max(1);
        let skip = page.saturating_sub(1).saturating_mul(limit);

        // Build parameterised WHERE conditions for attribute equality filters.
        // Reserved query keys are stripped by the handler before reaching here,
        // so every entry in `filters` is a genuine attribute filter.
        let mut where_clauses: Vec<String> = vec!["deleted_at = NONE".to_string()];
        let mut bindings: BTreeMap<String, Value> = BTreeMap::new();

        for (idx, (key, val)) in filters.iter().enumerate() {
            // Attribute key must be a safe identifier.
            if !key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                return Err(Error::Generic(format!("Invalid filter key: {}", key)));
            }
            let binding = format!("filter_val_{}", idx);
            where_clauses.push(format!("{} = ${}", key, binding));
            bindings.insert(binding, Value::String(val.clone()));
        }

        let where_str = where_clauses.join(" AND ");

        let limit_key = "list_limit".to_string();
        let skip_key = "list_skip".to_string();
        bindings.insert(limit_key.clone(), Value::Number(Number::Int(limit as i64)));
        bindings.insert(skip_key.clone(), Value::Number(Number::Int(skip as i64)));

        let data_sql = format!(
            "SELECT * FROM {} WHERE {} LIMIT $list_limit START $list_skip;",
            table_name, where_str
        );

        let responses = datastore
            .execute(&data_sql, database_session, Some(bindings.clone().into()))
            .await?;

        let it = into_iter_objects(responses)?;
        let mut list = Vec::new();
        for obj_res in it {
            let obj = obj_res?;
            list.push(object_to_json_value(obj));
        }

        // Count query uses the same WHERE conditions but no LIMIT/START bindings.
        let mut count_bindings: BTreeMap<String, Value> = BTreeMap::new();
        for (idx, (_key, val)) in filters.iter().enumerate() {
            let binding = format!("filter_val_{}", idx);
            count_bindings.insert(binding, Value::String(val.clone()));
        }

        let count_sql = format!(
            "SELECT count(id) FROM {} WHERE {} GROUP ALL;",
            table_name, where_str
        );

        let count_responses = datastore
            .execute(&count_sql, database_session, Some(count_bindings.into()))
            .await?;

        let total = match into_iter_objects(count_responses)?.next() {
            Some(Ok(obj)) => {
                let modal_count: crate::core::domain::entities::modal_count::ModalCount =
                    obj.try_into()?;
                modal_count.total
            }
            _ => 0,
        };

        Ok((list, total))
    }
}

pub async fn test_entity_repository() -> EntityRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    EntityRepositoryImpl::new(Arc::new(provider))
}
