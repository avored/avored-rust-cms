
use crate::core::application::dtos::collection_dto::{
    CollectionPaginationResponse, PaginateCollectionCommand,
};
use crate::core::application::dtos::entity_dto::PaginateEntityCommand;
use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};
use crate::core::domain::entities::entity::{
    StorableEntityAttribute, UpdableIdentifierEntity, UpdatableEntity,
};
use crate::core::domain::entities::{AttributeModel, EntityModel, StorableEntity};
use crate::core::domain::repositories::EntityRepository;
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct EntityUseCase<R>
where
    R: EntityRepository,
{
    repository: R,
}

impl<R> EntityUseCase<R>
where
    R: EntityRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel> {
        let attributes = storable_entity.attributes.clone();
        let logged_in_user = storable_entity.logged_in_user_email.clone();
        let mut entity_model = self.repository.create(storable_entity).await?;

        for attribute in attributes {
            let attrobute_model = self
                .repository
                .create_attribute(attribute, entity_model.id.clone(), logged_in_user.clone())
                .await?;

            entity_model.attributes.push(attrobute_model);
        }


        self.repository
            .create_collection_table(&entity_model.identifier)
            .await?;


        Ok(entity_model)
    }

    pub async fn get_by_id(&self, id: &str) -> Result<EntityModel> {
        self.repository.find_by_id(id).await
    }

    pub async fn paginate(&self, query: PaginateEntityCommand) -> Result<(Vec<EntityModel>, u64)> {
        let page: u64 = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
        let entities = self.repository.paginate(page, page_size).await?;
        let modal_count = self.repository.count().await?;

        Ok((entities, modal_count.total))
    }

    pub async fn update(&self, id: &str, updatable_entity: UpdatableEntity) -> Result<EntityModel> {
        let mut updated = self.repository.update(id, updatable_entity.clone()).await?;
        let logged_in_user = updatable_entity.logged_in_user_email.clone();

        for attribute in updatable_entity.attributes {
            let attrobute_model : AttributeModel;

            if attribute.is_new {
                let storable_entity_attribute = StorableEntityAttribute {
                    name: attribute.name,
                    identifier: attribute.identifier,
                    data_type: attribute.data_type,
                    field_type: attribute.field_type,
                };
                attrobute_model = self
                    .repository
                    .create_attribute(
                        storable_entity_attribute,
                        id.to_string(),
                        logged_in_user.clone(),
                    )
                    .await?;
            } else {
                attrobute_model = self
                    .repository
                    .update_attribute(
                        attribute,
                        logged_in_user.clone(),
                    )
                    .await?;
            }

            updated.attributes.push(attrobute_model);
        }

        Ok(updated)
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = self.repository.delete(id).await?;

        self.repository.delete_attribute(id).await?;
        // if result

        Ok(result)
    }

    pub async fn get_by_identifier(&self, identifier: &str) -> Result<EntityModel> {
        self.repository.find_by_identifier(identifier).await
    }

    pub async fn entity_count_by_identifier(&self, identifier: &str) -> Result<u64> {
        let entity = self.repository.find_by_identifier(identifier).await;
        match entity {
            Ok(_) => Ok(1),
            Err(Error::NotFound(_)) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub async fn list_options(&self) -> Result<Vec<EntityModel>> {
        self.repository.list_options().await
    }

    pub async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierEntity,
    ) -> Result<EntityModel> {
        self.repository
            .update_identifier(id, updatable_identifier)
            .await
    }

    pub async fn paginate_collection(
        &self,
        entity_id: &str,
        query: PaginateCollectionCommand,
    ) -> Result<CollectionPaginationResponse> {
        let entity = self.repository.find_by_id(entity_id).await?;
        let page: u64 = query.page.unwrap_or(DEFAULT_PAGE);
        let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

        let (data, total) = self
            .repository
            .paginate_collection(&entity.identifier, page, page_size)
            .await?;

        Ok(CollectionPaginationResponse {
            entity,
            data,
            total,
        })
    }

    pub async fn fetch_collection_by_id(
        &self,
        entity_id: &str,
        record_id: &str,
    ) -> Result<serde_json::Value> {
        let entity = self.repository.find_by_id(entity_id).await?;
        self.repository
            .fetch_collection_by_id(&entity.identifier, record_id)
            .await
    }

    pub async fn create_collection(
        &self,
        entity_id: &str,
        mut record: serde_json::Map<String, serde_json::Value>,
        logged_in_user: &str,
    ) -> Result<serde_json::Value> {
        let entity = self.repository.find_by_id(entity_id).await?;


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

        record = match strip_nulls(serde_json::Value::Object(record)).as_object().cloned() {
            Some(filtered) => filtered,
            None => serde_json::Map::new(),
        };


        let now = chrono::Utc::now().to_rfc3339();
        record.insert("created_at".to_string(), serde_json::Value::String(now.clone()));
        record.insert("created_by".to_string(), serde_json::Value::String(logged_in_user.to_string()));
        record.insert("updated_at".to_string(), serde_json::Value::String(now));
        record.insert("updated_by".to_string(), serde_json::Value::String(logged_in_user.to_string()));

        self.repository
            .create_collection(&entity.identifier, record)
            .await
    }

    pub async fn update_collection(
        &self,
        entity_id: &str,
        record_id: &str,
        mut record: serde_json::Map<String, serde_json::Value>,
        logged_in_user: &str,
    ) -> Result<serde_json::Value> {
        let entity = self.repository.find_by_id(entity_id).await?;

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

        record = match strip_nulls(serde_json::Value::Object(record)).as_object().cloned() {
            Some(filtered) => filtered,
            None => serde_json::Map::new(),
        };

        record.remove("id");
        record.remove("created_at");
        record.remove("created_by");
        record.remove("deleted_at");
        record.remove("deleted_by");

        let now = chrono::Utc::now().to_rfc3339();
        record.insert("updated_at".to_string(), serde_json::Value::String(now.clone()));
        record.insert("updated_by".to_string(), serde_json::Value::String(logged_in_user.to_string()));

        self.repository
            .update_collection_by_id(&entity.identifier, record_id, record)
            .await
    }

    pub async fn delete_collection(&self, entity_id: &str, record_id: &str) -> Result<bool> {
        let entity = self.repository.find_by_id(entity_id).await?;
        self.repository
            .delete_collection_by_id(&entity.identifier, record_id)
            .await
    }

    /// List dynamic records for a given entity type with optional attribute
    /// equality filters and pagination.
    ///
    /// Returns the raw JSON records together with the pagination envelope
    /// values (`page`, `limit`, `total_items`, `total_pages`).
    pub async fn list_entities(
        &self,
        entity_type: &str,
        page: u64,
        limit: u64,
        filters: std::collections::HashMap<String, String>,
    ) -> Result<(Vec<serde_json::Value>, u64, u64, u64)> {
        use crate::core::domain::constants::{DEFAULT_PAGE, DEFAULT_PAGE_SIZE};

        let page = if page == 0 { DEFAULT_PAGE } else { page };
        let limit = if limit == 0 {
            DEFAULT_PAGE_SIZE
        } else {
            limit.min(100)
        };

        // Resolve entity type to confirm it exists and get its table identifier.
        let entity = self.repository.find_by_identifier(entity_type).await?;

        let (records, total_items) = self
            .repository
            .list_entities(&entity.identifier, page, limit, &filters)
            .await?;

        let total_pages = if total_items == 0 {
            1
        } else {
            total_items.div_ceil(limit)
        };

        Ok((records, total_items, total_pages, limit))
    }
}

