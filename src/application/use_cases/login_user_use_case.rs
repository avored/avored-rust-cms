use crate::domain::models::admin_user::TokenClaims;
use crate::error::Result;
use crate::infra::grpc::admin_user_message::AdminUserMessage;
use crate::infra::grpc::auth_user::LoginResponseData;
use crate::infra::setup::get_env;
use crate::{
    application::extensions::string_extension::StringExtension,
    infra::repositories::user_repository::UserRepository,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::sync::Arc;

pub struct LoginUserUseCase {
    pub user_repository: Arc<dyn UserRepository>,
}

impl LoginUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<LoginResponseData> {
        if let Some(admin_user_model) = self.user_repository.find_by_email(email).await {
            let raw_password = String::from(password);
            let is_valid = raw_password.verify_password_hash(&admin_user_model.password_hash)?;

            if is_valid {
                let jwt_secret_key = get_env("AVORED_JWT_SECRET")?;

                let claims: TokenClaims = admin_user_model.clone().try_into()?;

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
                )?;

                let grpc_admin_user: AdminUserMessage = admin_user_model.try_into()?;

                let login_response_data = LoginResponseData {
                    admin_user: Some(grpc_admin_user),
                    token,
                };

                return Ok(login_response_data);
            }
        }
        Ok(LoginResponseData {
            admin_user: None,
            token: String::from(""),
        })
    }
}
