use crate::{
    domain::models::user::User, infra::repositories::surreal_user_repository::SurrealUserRepository,
};
use surrealdb::types::Value;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Option<User>;
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
            Some(value) => value,
            None => return None,
        };
        let user: User = user.try_into().unwrap();

        Some(user)
    }
}
