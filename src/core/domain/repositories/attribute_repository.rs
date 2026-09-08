use crate::core::domain::entities::attribute::UpdableIdentifierAttribute;
use crate::core::domain::entities::modal_count::ModalCount;
use crate::core::domain::entities::{AttributeModel, StorableAttribute};
use crate::error::Result;

#[async_trait::async_trait]
pub trait AttributeRepository: Send + Sync {
    async fn create(&self, storable_attribute: StorableAttribute) -> Result<AttributeModel>;

    async fn find_by_id(&self, id: &str) -> Result<AttributeModel>;

    async fn find_by_identifier(&self, identifier: &str) -> Result<AttributeModel>;

    async fn paginate(&self, page: u64, page_size: u64) -> Result<Vec<AttributeModel>>;

    async fn count(&self) -> Result<ModalCount>;

    async fn update(
        &self,
        id: &str,
        storable_attribute: StorableAttribute,
    ) -> Result<AttributeModel>;

    async fn delete(&self, id: &str) -> Result<bool>;

    async fn list_options(&self) -> Result<Vec<AttributeModel>>;

    async fn update_identifier(
        &self,
        id: &str,
        updatable_identifier: UpdableIdentifierAttribute,
    ) -> Result<AttributeModel>;
}
