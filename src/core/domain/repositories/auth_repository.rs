use crate::core::domain::entities::UserModel;
use crate::error::Result;

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    async fn authenticate(&self, email: &str) -> Result<UserModel>;
    async fn fetch_by_id(&self, id: &str) -> Result<UserModel>;
    async fn update_profile(&self, id: &str, name: &str, email: &str) -> Result<UserModel>;
    async fn update_password(&self, id: &str, password_hash: &str) -> Result<bool>;
}
