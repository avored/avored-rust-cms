use crate::core::domain::{entities::User, repositories::AuthRepository};
use crate::providers::avored_database_provider::AvoRedDatabaseProvider;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    pub database_provider: Arc<AvoRedDatabaseProvider>,
}

impl AuthRepositoryImpl {
    pub fn new(database_provider: Arc<AvoRedDatabaseProvider>) -> Self {
        Self { database_provider }
    }
}

impl AuthRepository for AuthRepositoryImpl {
    fn authenticate(&self, email: &str, _password: &str) -> Option<User> {
        // Flow / TODO checklist:
        // 1. Query SurrealDB users table by email (e.g. `SELECT * FROM user WHERE email = $email LIMIT 1`).
        // 2. Extract user record from SurrealDB result.
        // 3. Verify the hashed password with argon2/bcrypt against provided plain password.
        // 4. Map the SurrealDB user record into domain entity `User`.
        // 5. Return `Some(User)` on successful verification or `None` on failure.
        todo!("AuthRepositoryImpl::authenticate - implement SurrealDB query and password check for email: {}", email);
    }
}
