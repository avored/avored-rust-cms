use crate::core::domain::{entities::{User, user::StorableUser}, repositories::MiscRepository};

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

    pub async fn setup(&self, storable_user: StorableUser) -> crate::error::Result<User> {
        println!("->> {:<12} - setup", "MISC_USE_CASE");

        self.repository.create_user(storable_user).await
    }
}
