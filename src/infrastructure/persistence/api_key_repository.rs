use std::sync::Arc;

use crate::core::domain::entities::api_key::{ApiKeyModel, StorableApiKey, UpdatableApiKey};
use crate::core::domain::repositories::api_key_repository::ApiKeyRepository;
use crate::error::Result;
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
    async fn create(&self, _storable: StorableApiKey) -> Result<ApiKeyModel> {
        todo!()
    }
    async fn find_by_id(&self, _id: &str) -> Result<ApiKeyModel> {
        todo!()
    }
    async fn find_by_hash(&self, _key_hash: &str) -> Result<Option<ApiKeyModel>> {
        todo!()
    }
    async fn list(&self, _page: u64, _limit: u64) -> Result<(Vec<ApiKeyModel>, u64)> {
        todo!()
    }
    async fn update(&self, _id: &str, _updatable: UpdatableApiKey) -> Result<ApiKeyModel> {
        todo!()
    }
    async fn delete(&self, _id: &str) -> Result<bool> {
        todo!()
    }
}
