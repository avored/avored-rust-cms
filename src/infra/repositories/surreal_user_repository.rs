use crate::domain::models::user::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::infra::setup::DB;
use async_trait::async_trait;
use surrealdb::types::Value;

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
        let mut response = self
            .db
            .query("SELECT * FROM users WHERE email = $email")
            .bind(("email", email.to_string()))
            .await
            .ok()?;

        // let object = response
        let results = response.take::<Vec<Value>>(0).ok()?;

        println!("Results: {:#?}", results);

        let user = match results.first() {
            Some(value) => {
                value
            },
            None => return None,
        };
        let user: User = user.try_into().unwrap();        

        Some(user)
    }
}
