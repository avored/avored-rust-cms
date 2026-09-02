use crate::core::domain::{entities::user::StorableUser, repositories::MiscRepository};

#[derive(Clone)]
pub struct MiscUseCase<R>
where
    R: MiscRepository,
{
    repository: R,
}

impl<R> MiscUseCase<R>
where
    R: MiscRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn setup(&self, storable_user: StorableUser) -> crate::error::Result<bool> {
        println!("->> {:<12} - setup", "MISC_USE_CASE");

        match self.repository.create_user(storable_user).await {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
