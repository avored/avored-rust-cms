use crate::error::Result;
use crate::models::admin_user_model::AdminUserModel;
use crate::providers::avored_database_provider::DB;
use async_trait::async_trait;

/// Trait for authentication providers.
#[derive(Debug, Clone)]
pub enum AuthenticationResult {

    /// Represents a successful authentication with the user model.
    Success(AdminUserModel),

    /// Represents a failed authentication attempt with an error message.
    Failed(String),

    /// Represents a case where the user was not found in the database.
    UserNotFound,
}


///// Trait for authentication providers that can authenticate users.
#[async_trait]
pub trait AuthProvider: Send + Sync {

    /// Authenticates a user with the given username and password against the database.
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
        db: &DB,
    ) -> Result<AuthenticationResult>;


    
    /// Returns the type of the authentication provider.
    fn provider_name(&self) -> &'static str;

    /// Returns the type of the authentication provider as an enum.
    fn is_enabled(&self) -> bool;
}


/// Enum representing the type of authentication provider.
#[derive(Debug, Clone)]
pub enum AuthProviderType {

    /// Represents a local authentication provider.
    Local,

    /// Represents an LDAP authentication provider.
    Ldap,
}

impl AuthProviderType {
    /// Returns the string representation of the authentication provider type.
    pub fn as_str(&self) -> &'static str {
        match self {
            AuthProviderType::Local => "local",
            AuthProviderType::Ldap => "ldap",
        }
    }
}
