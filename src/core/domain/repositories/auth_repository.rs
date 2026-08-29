use crate::core::domain::entities::User;

pub trait AuthRepository {
    fn authenticate(&self, email: &str, password: &str) -> Option<User>;
}
