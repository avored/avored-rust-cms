use crate::core::domain::entities::api_key::{
    ApiKeyModel, ApiKeyPermission, StorableApiKey, UpdatableApiKey,
};
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
        use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
        use rand::Rng;
        use sha2::{Digest, Sha256};

        // 1. Generate 32 random bytes → base64url plain secret
        let random_bytes: [u8; 32] = rand::thread_rng().gen();
        let plain_secret = format!("avr_{}", URL_SAFE_NO_PAD.encode(random_bytes));

        // 2. Take first 8 chars as visible prefix
        let key_prefix = plain_secret.chars().take(8).collect::<String>();

        // 3. SHA-256 hash the full plain secret
        let mut hasher = Sha256::new();
        hasher.update(plain_secret.as_bytes());
        let key_hash = format!("{:x}", hasher.finalize());

        // 4. Persist
        let storable = StorableApiKey {
            name,
            key_prefix,
            key_hash,
            permissions,
            created_by: actor,
        };
        let model = self.repo.create(storable).await?;

        Ok((model, plain_secret))
    }

    pub async fn list(&self, page: u64, limit: u64) -> Result<(Vec<ApiKeyModel>, u64)> {
        self.repo.list(page, limit).await
    }

    pub async fn find_by_id(&self, id: &str) -> Result<ApiKeyModel> {
        self.repo.find_by_id(id).await
    }

    pub async fn find_by_hash_public(&self, key_hash: &str) -> Result<Option<ApiKeyModel>> {
        self.repo.find_by_hash(key_hash).await
    }

    pub async fn update(
        &self,
        id: &str,
        name: String,
        permissions: Vec<ApiKeyPermission>,
        actor: String,
    ) -> Result<ApiKeyModel> {
        let updatable = UpdatableApiKey {
            name,
            permissions,
            updated_by: actor,
        };
        self.repo.update(id, updatable).await
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        self.repo.delete(id).await
    }
}
