#[derive(Debug, Clone)]
pub struct LoginCommand {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct LoginResult {
    pub token: String,
    pub user: crate::core::domain::entities::User,
    pub authenticated: bool,
}
