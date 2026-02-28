use std::sync::Arc;
use crate::domain::models::admin_user::StorableAdminUser;
use crate::error::Result;
use crate::infra::repositories::misc_repository::MiscRepository;

#[derive(Clone)]
pub struct MiscUseCase {
    pub misc_repository: Arc<dyn MiscRepository>,
}

impl MiscUseCase {
    pub fn new(misc_repository: Arc<dyn MiscRepository>) -> Self {
        Self { misc_repository }
    }

    pub async fn setup(&self, storable_admin_user: StorableAdminUser) -> Result<bool> {
        self.misc_repository.setup(storable_admin_user).await
    }
}
