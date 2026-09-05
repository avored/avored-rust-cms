use jsonwebtoken::{EncodingKey, Header, encode};

use crate::core::{
    application::dtos::{LoginCommand, LoginResult}, domain::{entities::user::TokenClaims, extensions::string_extension::StringExtension, repositories::AuthRepository},
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

        if password.password_verification(&encrypted_password).is_err() {
            return Err(crate::error::Error::Generic(
                "Invalid credentials".to_string(),
            ));
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
}
