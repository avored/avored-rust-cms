use crate::error::Result;
use crate::models::admin_user_model::AdminUserModel;
use crate::providers::avored_database_provider::DB;
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub enum AuthenticationResult {
    Success(AdminUserModel),
    Failed(String),
    UserNotFound,
}

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
        db: &DB,
    ) -> Result<AuthenticationResult>;

    fn provider_name(&self) -> &'static str;
    fn is_enabled(&self) -> bool;
}

#[derive(Debug, Clone)]
pub enum AuthProviderType {
    Local,
    Ldap,
}

impl AuthProviderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthProviderType::Local => "local",
            AuthProviderType::Ldap => "ldap",
        }
    }
}
