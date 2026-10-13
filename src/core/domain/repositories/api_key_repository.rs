use crate::core::domain::entities::api_key::{ApiKeyModel, StorableApiKey, UpdatableApiKey};
use crate::error::Result;

#[async_trait::async_trait]
pub trait ApiKeyRepository: Send + Sync {
    async fn create(&self, storable: StorableApiKey) -> Result<ApiKeyModel>;

    async fn find_by_id(&self, id: &str) -> Result<ApiKeyModel>;

    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<ApiKeyModel>>;

    async fn list(&self, page: u64, limit: u64) -> Result<(Vec<ApiKeyModel>, u64)>;

    async fn update(&self, id: &str, updatable: UpdatableApiKey) -> Result<ApiKeyModel>;

    async fn delete(&self, id: &str) -> Result<bool>;
}
