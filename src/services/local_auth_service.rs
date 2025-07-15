use crate::error::Result;
use crate::providers::auth_provider::{AuthProvider, AuthenticationResult};
use crate::providers::avored_database_provider::DB;
use crate::repositories::admin_user_repository::AdminUserRepository;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use tracing::{error, info};

pub struct LocalAuthService {
    admin_user_repository: AdminUserRepository,
}

impl LocalAuthService {
    pub fn new(admin_user_repository: AdminUserRepository) -> Self {
        Self {
            admin_user_repository,
        }
    }

    fn compare_password(&self, plain_password: &str, encrypted_password: &str) -> Result<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(encrypted_password)?;

        Ok(argon2
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}

#[async_trait]
impl AuthProvider for LocalAuthService {
    async fn authenticate(
        &self,
        username: &str,
        password: &str,
        db: &DB,
    ) -> Result<AuthenticationResult> {
        if username.is_empty() || password.is_empty() {
            return Ok(AuthenticationResult::Failed(
                "Username and password are required".to_string(),
            ));
        }

        // Find user by email (username is email in local auth)
        let admin_user_model = match self
            .admin_user_repository
            .find_by_email(&db.0, &db.1, username)
            .await
        {
            Ok(user) => user,
            Err(_) => {
                info!("Local user not found: {}", username);
                return Ok(AuthenticationResult::UserNotFound);
            }
        };

        // Verify password
        let is_password_match = match self.compare_password(password, &admin_user_model.password) {
            Ok(result) => result,
            Err(e) => {
                error!("Password comparison failed for user {}: {}", username, e);
                return Ok(AuthenticationResult::Failed(
                    "Authentication failed".to_string(),
                ));
            }
        };

        if !is_password_match {
            info!("Invalid password for local user: {}", username);
            return Ok(AuthenticationResult::Failed(
                "Invalid credentials".to_string(),
            ));
        }

        info!("Local user authenticated successfully: {}", username);
        Ok(AuthenticationResult::Success(admin_user_model))
    }

    fn provider_name(&self) -> &'static str {
        "local"
    }

    fn is_enabled(&self) -> bool {
        true // Local authentication is always enabled
    }
}
