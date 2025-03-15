use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::error::{Result, Error};
use crate::grpc_auth::{LoginRequest, LoginResponse};
use crate::providers::avored_database_provider::DB;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::models::token_claim_model::TokenClaims;
use jsonwebtoken::{encode, EncodingKey, Header};

pub struct AuthService {
    admin_user_repository: AdminUserRepository,
}

impl AuthService {
    pub(crate) async fn auth_user(
        &self,
        request: LoginRequest,
        (datastore, database_session): &DB,
        jwt_secret_key: &str
    ) -> Result<LoginResponse> {

    let admin_user_model = self
        .admin_user_repository
        .find_by_email(datastore, database_session, &request.email)
        .await?;

        let is_password_match: bool = self
            .compare_password(request.password, admin_user_model.password.clone())?;

        if !is_password_match {
            return Err(Error::Generic("Email and password did not match".to_string()));
        }

        let claims: TokenClaims = admin_user_model.clone().try_into()?;

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
        )?;

        let login_response = LoginResponse {
            status: true,
            data: token,
        };

        Ok(login_response)
    }

    pub fn compare_password(
        &self,
        plain_password: String,
        encrypted_password: String,
    ) -> Result<bool> {
        let argon2 = Argon2::default();

        let parsed_hash = PasswordHash::new(&encrypted_password)?;

        Ok(argon2
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok())
    }

}

impl AuthService {
    pub async fn new(admin_user_repository: AdminUserRepository) -> Result<AuthService> {
        Ok(AuthService {admin_user_repository})
    }
}