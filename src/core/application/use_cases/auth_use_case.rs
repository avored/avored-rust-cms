use crate::core::{
    application::dtos::{LoginCommand, LoginResult},
    domain::{extensions::string_extension::StringExtension, repositories::AuthRepository},
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

    pub async fn auth(&self, command: LoginCommand) -> Result<LoginResult> {
        let user = self.repository.authenticate(&command.email).await?;

        let password = command.password.clone();
        let encrypted_password = user.password.clone();

        if password.password_verification(&encrypted_password).is_err() {
            return Err(crate::error::Error::Generic(
                "Invalid credentials".to_string(),
            ));
        }

        Ok(LoginResult {
            token: format!("demo-token-for-{}", user.id),
            user,
            authenticated: true,
        })
    }
}
