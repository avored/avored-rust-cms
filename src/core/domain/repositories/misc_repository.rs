use crate::core::domain::entities::UserModel;
use crate::core::domain::entities::user::StorableUser;
use crate::error::Result;

#[async_trait::async_trait]
pub trait MiscRepository: Send + Sync {
    async fn create_user(&self, storable_user: StorableUser) -> Result<UserModel>;
}
