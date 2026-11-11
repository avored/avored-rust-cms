use jsonwebtoken::{encode, EncodingKey, Header};

use crate::core::{
    application::dtos::{LoginCommand, LoginResult},
    domain::{
        entities::user::TokenClaims, extensions::string_extension::StringExtension,
        repositories::AuthRepository,
    },
};
use crate::error::Result;

#[derive(Clone)]
pub struct AuthUseCase<R>
where
    R: AuthRepository,
{
    repository: R,
}

impl<R> AuthUseCase<R>
where
    R: AuthRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn auth(&self, command: LoginCommand, jwt_secret_key: String) -> Result<LoginResult> {
        let user = self.repository.authenticate(&command.email).await?;

        let password = command.password.clone();
        let encrypted_password = user.password.clone();

        if !password
            .password_verification(&encrypted_password)
            .unwrap_or(false)
        {
            return Err(crate::error::Error::Authentication);
        }

        let claims: TokenClaims = user.clone().try_into()?;

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
        )?;

        Ok(LoginResult {
            token,
            user,
            authenticated: true,
        })
    }

    pub async fn fetch_profile(
        &self,
        user_id: &str,
    ) -> Result<crate::core::application::dtos::AdminProfileResponse> {
        let user = self.repository.fetch_by_id(user_id).await?;
        Ok(user.into())
    }

    pub async fn update_profile(
        &self,
        user_id: &str,
        command: crate::core::application::dtos::UpdateProfileCommand,
    ) -> Result<crate::core::application::dtos::AdminProfileResponse> {
        if command.name.trim().is_empty() {
            return Err(crate::error::Error::Generic("Name is required".to_string()));
        }
        if command.email.trim().is_empty() {
            return Err(crate::error::Error::Generic(
                "Email is required".to_string(),
            ));
        }
        let user = self
            .repository
            .update_profile(user_id, command.name.trim(), command.email.trim())
            .await?;
        Ok(user.into())
    }

    pub async fn change_password(
        &self,
        user_id: &str,
        command: crate::core::application::dtos::ChangePasswordCommand,
        password_salt: &str,
    ) -> Result<bool> {
        if command.current_password.is_empty() {
            return Err(crate::error::Error::Generic(
                "Current password is required".to_string(),
            ));
        }
        if command.new_password.len() < 6 {
            return Err(crate::error::Error::Generic(
                "New password must be at least 6 characters long".to_string(),
            ));
        }
        if command.new_password != command.confirm_password {
            return Err(crate::error::Error::Generic(
                "New password and confirm password do not match".to_string(),
            ));
        }

        let user = self.repository.fetch_by_id(user_id).await?;
        if !command
            .current_password
            .password_verification(&user.password)
            .unwrap_or(false)
        {
            return Err(crate::error::Error::Generic(
                "Current password is incorrect".to_string(),
            ));
        }

        let new_hash = command.new_password.get_password_hash(password_salt)?;
        self.repository.update_password(user_id, &new_hash).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{
        application::dtos::{ChangePasswordCommand, LoginCommand, UpdateProfileCommand},
        domain::{entities::UserModel, extensions::string_extension::StringExtension},
    };
    use async_trait::async_trait;
    use surrealdb::types::Datetime;

    #[derive(Clone, Default)]
    struct MockAuthRepository {
        user: UserModel,
    }

    impl MockAuthRepository {
        fn new() -> Self {
            let mut user = UserModel::default();
            user.id = "user-1".to_string();
            user.name = "Alice".to_string();
            user.email = "alice@example.com".to_string();
            user.password = "password123".to_string().get_password_hash("test-salt").unwrap();
            user.created_at = Datetime::default();
            user.updated_at = Datetime::default();
            Self { user }
        }
    }

    #[async_trait]
    impl AuthRepository for MockAuthRepository {
        async fn authenticate(&self, email: &str) -> Result<UserModel> {
            if email == self.user.email {
                Ok(self.user.clone())
            } else {
                Err(crate::error::Error::Authentication)
            }
        }

        async fn fetch_by_id(&self, id: &str) -> Result<UserModel> {
            if id == self.user.id {
                Ok(self.user.clone())
            } else {
                Err(crate::error::Error::NotFound(id.to_string()))
            }
        }

        async fn update_profile(&self, id: &str, name: &str, email: &str) -> Result<UserModel> {
            if id != self.user.id {
                return Err(crate::error::Error::NotFound(id.to_string()));
            }

            let mut user = self.user.clone();
            user.name = name.to_string();
            user.email = email.to_string();
            Ok(user)
        }

        async fn update_password(&self, id: &str, password_hash: &str) -> Result<bool> {
            if id != self.user.id {
                return Err(crate::error::Error::NotFound(id.to_string()));
            }
            assert!(!password_hash.is_empty());
            Ok(true)
        }
    }

    #[tokio::test]
    async fn auth_returns_token_for_valid_credentials() {
        let repository = MockAuthRepository::new();
        let use_case = AuthUseCase::new(repository);

        let result = use_case
            .auth(
                LoginCommand {
                    email: "alice@example.com".to_string(),
                    password: "password123".to_string(),
                },
                "secret-key".to_string(),
            )
            .await
            .expect("valid credentials should authenticate");

        assert!(result.authenticated);
        assert_eq!(result.user.email, "alice@example.com");
        assert!(!result.token.trim().is_empty());
    }

    #[tokio::test]
    async fn update_profile_rejects_empty_name() {
        let use_case = AuthUseCase::new(MockAuthRepository::new());

        let result = use_case
            .update_profile(
                "user-1",
                UpdateProfileCommand {
                    name: "   ".to_string(),
                    email: "updated@example.com".to_string(),
                },
            )
            .await;

        assert!(matches!(result, Err(crate::error::Error::Generic(message)) if message == "Name is required"));
    }

    #[tokio::test]
    async fn change_password_requires_matching_confirmation() {
        let use_case = AuthUseCase::new(MockAuthRepository::new());

        let result = use_case
            .change_password(
                "user-1",
                ChangePasswordCommand {
                    current_password: "password123".to_string(),
                    new_password: "new-password".to_string(),
                    confirm_password: "different-password".to_string(),
                },
                "test-salt",
            )
            .await;

        assert!(matches!(result, Err(crate::error::Error::Generic(message)) if message == "New password and confirm password do not match"));
    }
}
