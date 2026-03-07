use crate::{
    domain::models::admin_user::AdminUserModel,
    infra::repositories::surreal_user_repository::SurrealUserRepository,
};
use async_trait::async_trait;
use surrealdb::types::Value;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Option<AdminUserModel>;
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
    async fn find_by_email(&self, email: &str) -> Option<AdminUserModel> {
        let mut response: surrealdb::IndexedResults = self
            .db
            .query("SELECT * FROM admin_users WHERE email = $email")
            .bind(("email", email.to_string()))
            .await
            .ok()?;

        // let object = response
        let results = response.take::<Vec<Value>>(0).ok()?;

        // println!("Results: {:#?}", results);

        let user = match results.first() {
            Some(value) => value,
            None => return None,
        };
        let user: AdminUserModel = user.try_into().unwrap();

        Some(user)
    }
}
