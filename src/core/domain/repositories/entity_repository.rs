use crate::core::domain::entities::entity::{
    EntityModel, StorableEntity, StorableEntityAttribute, UpdableIdentifierEntity, UpdatableEntity,
    UpdatableEntityAttribute,
};
use crate::core::domain::entities::modal_count::ModalCount;
use crate::core::domain::entities::AttributeModel;
use crate::error::Result;

#[async_trait::async_trait]
pub trait EntityRepository: Send + Sync {
    async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel>;

    async fn create_attribute(
        &self,
        attribute: StorableEntityAttribute,
        entity_id: String,
        logged_in_user: String,
    ) -> Result<AttributeModel>;

    async fn find_by_id(&self, id: &str) -> Result<EntityModel>;

    async fn find_by_identifier(&self, identifier: &str) -> Result<EntityModel>;

    async fn paginate(&self, page: u64, page_size: u64) -> Result<Vec<EntityModel>>;

    async fn count(&self) -> Result<ModalCount>;

    async fn update(&self, id: &str, updatable_entity: UpdatableEntity) -> Result<EntityModel>;

    async fn delete(&self, id: &str) -> Result<bool>;

    async fn list_options(&self) -> Result<Vec<EntityModel>>;

    async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierEntity,
    ) -> Result<EntityModel>;

    async fn update_attribute(
        &self,
        attribute: UpdatableEntityAttribute,
        logged_in_user: String,
    ) -> Result<AttributeModel>;

    async fn delete_attribute(&self, entity_id: &str) -> Result<bool>;

    async fn paginate_collection(
        &self,
        table_name: &str,
        page: u64,
        page_size: u64,
    ) -> Result<(Vec<serde_json::Value>, u64)>;

    async fn fetch_collection_by_id(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<serde_json::Value>;

    async fn create_collection_table(&self, table_name: &str) -> Result<()>;

    async fn create_collection(
        &self,
        table_name: &str,
        record: serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value>;

    async fn update_collection_by_id(
        &self,
        table_name: &str,
        record_id: &str,
        record: serde_json::Map<String, serde_json::Value>,
    ) -> Result<serde_json::Value>;
}
