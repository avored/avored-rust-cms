use crate::core::domain::entities::UserModel;
use crate::error::Result;

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    async fn authenticate(&self, email: &str) -> Result<UserModel>;
}
