use crate::domain::models::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infra::setup::DB;
use async_trait::async_trait;

pub struct SurrealUserRepository {
    pub db: DB,
}

impl SurrealUserRepository {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn find_by_email(&self, email: &str) -> Option<User> {
        // let mut response = self
        //     .db
        //     .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        //     .bind(("email", email))
        //     .await
        //     .ok()?;

        // let mut users: Vec<User> = response.take(0).ok()?;
        // users.pop()

        let user = User {
            id: "id".to_string(),
            email: email.to_string(),
            password_hash: "admin123".to_string()
        };

        Some(user)
    }
}
