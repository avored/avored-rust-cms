use std::collections::BTreeMap;
use std::sync::Arc;
use surrealdb::types::{Number, Value};

use crate::core::domain::constants::ENTITIES_TABLE_NAME;
use crate::core::domain::entities::entity::{EntityModel, StorableEntity, UpdableIdentifierEntity};
use crate::core::domain::entities::modal_count::ModalCount;
use crate::core::domain::repositories::EntityRepository;
use crate::error::Result;
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

#[async_trait::async_trait]
impl EntityRepository for EntityRepositoryImpl {
    async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!("
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

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No entity returned from insert".to_string()))??;

        let entity: EntityModel = result_object.try_into()?;
        Ok(entity)
    }

    async fn find_by_id(&self, id: &str) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let target_record = surrealdb::types::RecordId {
            table: ENTITIES_TABLE_NAME.into(),
            key: surrealdb::types::RecordIdKey::String(id.to_string()),
        };

        let sql = format!("
            SELECT * 
            FROM {} 
            WHERE id = $id AND deleted_at = NONE;", 
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

        let sql = format!("
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

        let sql = format!("
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

    async fn update(&self, id: &str, storable_entity: StorableEntity) -> Result<EntityModel> {
        let (datastore, database_session) = &self.database_provider.db;

        let id_clean = id.trim_start_matches("entities:").to_string();
        let target_record = surrealdb::types::RecordId {
            table: "entities".into(),
            key: surrealdb::types::RecordIdKey::String(id_clean),
        };

        let sql = format!("
            UPDATE {} 
            SET name=$name, identifier=$identifier, updated_at=time::now(), updated_by=$updated_by 
            WHERE id = $id AND deleted_at = NONE;", 
            ENTITIES_TABLE_NAME
        );
        let data: BTreeMap<String, Value> = [
            ("id".into(), Value::RecordId(target_record)),
            ("name".into(), Value::String(storable_entity.name.into())),
            (
                "identifier".into(),
                Value::String(storable_entity.identifier.into()),
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

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No entity returned from update".to_string()))??;

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

        let sql = format!("
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

        let sql = format!("
            SELECT count(id) 
            FROM {} 
            WHERE deleted_at = NONE 
            GROUP ALL;", 
            ENTITIES_TABLE_NAME
        );

            let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No entity returned from count".to_string()))??;

        let count: ModalCount = result_object.try_into()?;
        Ok(count)
    }

    async fn list_options(&self) -> Result<Vec<EntityModel>> {
        let (datastore, database_session) = &self.database_provider.db;

        let sql = format!("
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

        let sql = format!("
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

        let result_object = into_iter_objects(responses)?
            .next()
            .ok_or_else(|| crate::error::Error::Generic("No entity returned from update identifier".to_string()))??;

        let model: EntityModel = result_object.try_into()?;
        Ok(model)
    }
}

pub async fn test_entity_repository() -> EntityRepositoryImpl {
    let provider = AvoRedDatabaseProvider::register("mem://", "test", "auth")
        .await
        .expect("in-memory database should initialize");

    EntityRepositoryImpl::new(Arc::new(provider))
}
