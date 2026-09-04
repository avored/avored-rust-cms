use crate::core::domain::entities::entity::{EntityModel, StorableEntity};
use crate::core::domain::entities::modal_count::ModalCount;
use crate::error::Result;

#[async_trait::async_trait]
pub trait EntityRepository: Send + Sync {
    async fn create(&self, storable_entity: StorableEntity) -> Result<EntityModel>;
    async fn find_by_id(&self, id: &str) -> Result<Option<EntityModel>>;
    async fn find_by_identifier(&self, identifier: &str) -> Result<Option<EntityModel>>;
    async fn paginate(&self, page: u64, page_size: u64) -> Result<Vec<EntityModel>>;
    async fn count(&self) -> Result<ModalCount>;
    async fn update(&self, id: &str, storable_entity: StorableEntity) -> Result<EntityModel>;
    async fn delete(&self, id: &str) -> Result<bool>;
}
