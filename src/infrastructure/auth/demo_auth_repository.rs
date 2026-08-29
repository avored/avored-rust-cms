use crate::core::{
    domain::{entities::User, repositories::AuthRepository},
};

#[derive(Debug, Default, Clone)]
pub struct DemoAuthRepository;

impl AuthRepository for DemoAuthRepository {
    fn authenticate(&self, email: &str, password: &str) -> Option<User> {
        if email == "demo@avored.local" && password == "password" {
            Some(User::new(
                "demo-user-id",
                "Demo User",
                "demo@avored.local",
            ))
        } else {
            None
        }
    }
}
