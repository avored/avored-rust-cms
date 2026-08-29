use crate::core::domain::entities::User;

#[async_trait::async_trait]
pub trait AuthRepository: Send + Sync {
    async fn authenticate(&self, email: &str, password: &str) -> Option<User>;
}


