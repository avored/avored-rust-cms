use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::types::{Number, Value};

use crate::core::domain::constants::{ATTRIBUTES_TABLE_NAME, ENTITIES_TABLE_NAME};
use crate::core::domain::entities::modal_count::ModalCount;
use crate::core::domain::entities::{AttributeModel, StorableAttribute};
use crate::core::domain::repositories::AttributeRepository;
use crate::error::Result;
use crate::infrastructure::persistence::into_iter_objects;
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;

#[derive(Clone)]
pub struct AttributeRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl AttributeRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

#[async_trait::async_trait]
impl AttributeRepository for AttributeRepositoryImpl {
    async fn create(&self, storable_attribute: StorableAttribute) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!("
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
                    key: surrealdb::types::RecordIdKey::String(
                        storable_attribute
                            .entity_id
                            .to_string(),
                    ),
                }),
            ),
            ("data_type".into(), Value::String(storable_attribute.data_type.into())),
            ("field_type".into(), Value::String(storable_attribute.field_type.into())),
            (
                "created_by".into(),
                Value::String(storable_attribute.logged_in_user_email.clone().into()),
            ),
            (
                "updated_by".into(),
                Value::String(storable_attribute.logged_in_user_email.into()),
            )
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No attribute returned from insert".to_string()))??;

        let attribute: AttributeModel = result_object.try_into()?;
        Ok(attribute)
    }

    async fn find_by_id(&self, id: &str) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: ATTRIBUTES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!("
            SELECT * 
            FROM {} 
            WHERE id = $id AND deleted_at = NONE;", 
            ATTRIBUTES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [("id".into(), Value::RecordId(target_record))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut it = into_iter_objects(responses)?;
        if let Some(obj_res) = it.next() {
            let obj = obj_res?;
            let model: AttributeModel = obj.try_into()?;
            return Ok(model);
        }

        Err(crate::error::Error::NotFound(format!(
            "Attribute with ID '{}' not found",
            id
        )))
    }

    async fn find_by_identifier(&self, identifier: &str) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!("
            SELECT * 
            FROM {} 
            WHERE identifier = $identifier AND deleted_at = NONE;", 
            ATTRIBUTES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> =
            [("identifier".into(), Value::String(identifier.into()))].into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let mut it = into_iter_objects(responses)?;
        if let Some(obj_res) = it.next() {
            let obj = obj_res?;
            let model: AttributeModel = obj.try_into()?;
            return Ok(model);
        }

        Err(crate::error::Error::NotFound(format!(
            "Attribute with identifier '{}' not found",
            identifier
        )))
    }

    async fn paginate(&self, page: u64, page_size: u64) -> Result<Vec<AttributeModel>> {
        let (datastore, database_session) = &self.database_provider.db;

        let skip = page.saturating_sub(1) * page_size;

        let number_page_size = Number::Int(page_size as i64);
        let number_skip = Number::Int(skip as i64);

        let sql = format!("
            SELECT * 
            FROM {} 
            WHERE deleted_at = NONE 
            LIMIT $limit 
            START $skip;", 
            ATTRIBUTES_TABLE_NAME
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
            let model: AttributeModel = obj.try_into()?;
            list.push(model);
        }

        Ok(list)
    }

    async fn update(&self, id: &str, storable_attribute: StorableAttribute) -> Result<AttributeModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let id_clean = id.trim_start_matches("attributes:").to_string();
        let target_record = surrealdb::types::RecordId {
            table: ATTRIBUTES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id_clean),
        };

        let sql = format!("
            UPDATE {} 
            SET name=$name, identifier=$identifier, updated_at=time::now(), updated_by=$updated_by 
            WHERE id = $id AND deleted_at = NONE;", 
            ATTRIBUTES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [
            ("id".into(), Value::RecordId(target_record)),
            ("entity_id".into(), Value::String(storable_attribute.entity_id.into())),
            ("name".into(), Value::String(storable_attribute.name.into())),
            (
                "identifier".into(),
                Value::String(storable_attribute.identifier.into()),
            ),
            (
                "data_type".into(),
                Value::String(storable_attribute.data_type.into()),
            ),
            (
                "field_type".into(),
                Value::String(storable_attribute.field_type.into()),
            ),
            (
                "updated_by".into(),
                Value::String(storable_attribute.logged_in_user_email.into()),
            ),
        ]
        .into();

        let responses = datastore
            .execute(&sql, database_session, Some(data.into()))
            .await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No attribute returned from update".to_string()))??;

        let attribute: AttributeModel = result_object.try_into()?;
        Ok(attribute)
    }

    async fn delete(&self, id: &str) -> Result<bool> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: ATTRIBUTES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!("
            UPDATE {} 
            SET deleted_at=time::now(), updated_at=time::now() 
            WHERE id = $id;", 
            ATTRIBUTES_TABLE_NAME
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

        let sql = format!("
            SELECT count(id) 
            FROM {} 
            WHERE deleted_at = NONE 
            GROUP ALL;", 
            ATTRIBUTES_TABLE_NAME
        );

            let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No attribute returned from count".to_string()))??;

        let count: ModalCount = result_object.try_into()?;
        Ok(count)
    }

    async fn list_options(&self) -> Result<Vec<AttributeModel>> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!("
            SELECT id, name 
            FROM {} 
            WHERE deleted_at = NONE;", 
            ATTRIBUTES_TABLE_NAME
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let it = into_iter_objects(responses)?;
        let mut list = Vec::new();
        for obj_res in it {
            let obj = obj_res?;
            let model: AttributeModel = obj.try_into()?;
            list.push(model);
        }

        Ok(list)
    }
}

pub async fn test_attribute_repository() -> AttributeRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    AttributeRepositoryImpl::new(Arc::new(provider))
}
