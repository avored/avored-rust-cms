use rand::RngExt;

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
        use sha2::{Digest, Sha256};

        // 1. Generate 32 random bytes → base64url plain secret
        let random_bytes: [u8; 32] = rand::rng().random();
        let plain_secret = format!("avr_{}", URL_SAFE_NO_PAD.encode(random_bytes));

        // 2. Take first 8 chars as visible prefix
        let key_prefix = plain_secret.chars().take(8).collect::<String>();

        // 3. SHA-256 hash the full plain secret
        let mut hasher = Sha256::new();
        hasher.update(plain_secret.as_bytes());
        let key_hash = hasher
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();

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

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    #[derive(Clone, Default)]
    struct MockApiKeyRepository;

    #[async_trait]
    impl ApiKeyRepository for MockApiKeyRepository {
        async fn create(&self, storable: StorableApiKey) -> Result<ApiKeyModel> {
            Ok(ApiKeyModel {
                id: "api-key-1".to_string(),
                key_prefix: storable.key_prefix,
                key_hash: storable.key_hash,
                permissions: storable.permissions,
                created_at: "2026-01-01T00:00:00Z".to_string(),
                created_by: storable.created_by.clone(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                updated_by: storable.created_by,
                name: storable.name,
            })
        }

        async fn find_by_id(&self, id: &str) -> Result<ApiKeyModel> {
            Ok(ApiKeyModel {
                id: id.to_string(),
                name: "Billing API".to_string(),
                key_prefix: "avr_ab12c".to_string(),
                key_hash: "hash".to_string(),
                permissions: vec![ApiKeyPermission {
                    endpoint: "/api/v1/entities".to_string(),
                    method: "GET".to_string(),
                }],
                created_at: "2026-01-01T00:00:00Z".to_string(),
                created_by: "admin".to_string(),
                updated_at: "2026-01-01T00:00:00Z".to_string(),
                updated_by: "admin".to_string(),
            })
        }

        async fn find_by_hash(&self, _key_hash: &str) -> Result<Option<ApiKeyModel>> {
            Ok(None)
        }

        async fn list(&self, _page: u64, _limit: u64) -> Result<(Vec<ApiKeyModel>, u64)> {
            Ok((vec![], 0))
        }

        async fn update(&self, id: &str, updatable: UpdatableApiKey) -> Result<ApiKeyModel> {
            Ok(ApiKeyModel {
                id: id.to_string(),
                name: updatable.name,
                key_prefix: "avr_ab12c".to_string(),
                key_hash: "updated-hash".to_string(),
                permissions: updatable.permissions,
                created_at: "2026-01-01T00:00:00Z".to_string(),
                created_by: "admin".to_string(),
                updated_at: "2026-01-02T00:00:00Z".to_string(),
                updated_by: updatable.updated_by,
            })
        }

        async fn delete(&self, _id: &str) -> Result<bool> {
            Ok(true)
        }
    }

    #[tokio::test]
    async fn create_generates_plain_secret_and_hash() {
        let use_case = ApiKeyUseCase::new(MockApiKeyRepository);
        let permissions = vec![ApiKeyPermission {
            endpoint: "/api/v1/entities".to_string(),
            method: "GET".to_string(),
        }];

        let (model, plain_secret) = use_case
            .create("Billing API".to_string(), permissions, "admin".to_string())
            .await
            .expect("api key creation should succeed");

        assert!(plain_secret.starts_with("avr_"));
        assert_eq!(model.name, "Billing API");
        assert_eq!(model.created_by, "admin");
        assert_eq!(model.key_prefix, plain_secret.chars().take(8).collect::<String>());
        assert!(!model.key_hash.is_empty());
    }
}
