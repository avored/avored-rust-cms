use std::sync::Arc;
use crate::infra::repositories::user_repository::UserRepository;

pub struct LoginUserUseCase {
    pub user_repository: Arc<dyn UserRepository>,
}

impl LoginUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, email: &str, password: &str) -> bool {
        if let Some(user) = self.user_repository.find_by_email(email).await {
            // In a real app, you would check the password hash here
            // For now, let's just compare (demo purpose)

            println!("User found: {:?}", user);
            return user.password_hash == password;
        }
        false
    }
}
