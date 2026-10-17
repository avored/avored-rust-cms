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
            token: token,
            user: user,
            authenticated: true,
        })
    }

    pub async fn fetch_profile(&self, user_id: &str) -> Result<crate::core::application::dtos::AdminProfileResponse> {
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
            return Err(crate::error::Error::Generic("Email is required".to_string()));
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
