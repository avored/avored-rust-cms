use crate::core::domain::entities::api_key::{ApiKeyModel, ApiKeyPermission};
use crate::core::domain::repositories::api_key_repository::ApiKeyRepository;
use crate::error::Result;

#[derive(Clone)]
pub struct ApiKeyUseCase<R: ApiKeyRepository> {
    repo: R,
}

impl<R: ApiKeyRepository> ApiKeyUseCase<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Returns (model, plain_secret) — plain_secret only shown once.
    pub async fn create(
        &self,
        name: String,
        permissions: Vec<ApiKeyPermission>,
        actor: String,
    ) -> Result<(ApiKeyModel, String)> {
        todo!()
    }

    pub async fn list(&self, page: u64, limit: u64) -> Result<(Vec<ApiKeyModel>, u64)> {
        todo!()
    }

    pub async fn find_by_id(&self, id: &str) -> Result<ApiKeyModel> {
        todo!()
    }

    pub async fn update(
        &self,
        id: &str,
        name: String,
        permissions: Vec<ApiKeyPermission>,
        actor: String,
    ) -> Result<ApiKeyModel> {
        todo!()
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        todo!()
    }
}
