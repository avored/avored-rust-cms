use crate::error::{Error, Result};
use crate::extensions::email_message_builder::EmailMessageBuilder;
use crate::extensions::string_extension::StringExtension;
use crate::models::ldap_config_model::LdapConfig;
use crate::models::password_rest_model::{CreatablePasswordResetModel, ForgotPasswordViewModel};
use crate::models::token_claim_model::TokenClaims;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use crate::providers::avored_database_provider::DB;
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::services::ldap_auth_service::LdapAuthService;
use crate::services::local_auth_service::LocalAuthService;
use crate::services::multi_auth_service::MultiAuthService;
use crate::Error::TonicError;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{AsyncTransport, Message};
use rand::distr::Alphanumeric;
use rand::Rng;
use rust_i18n::t;
use std::sync::Arc;
use tonic::Status;
use tracing::error;

pub struct AuthService {
    admin_user_repository: AdminUserRepository,
    password_reset_repository: PasswordResetRepository,
    multi_auth_service: MultiAuthService,
}

impl AuthService {
    pub async fn forgot_password(
        &self,
        db: &DB,
        template: &AvoRedTemplateProvider,
        react_admin_url: &str,
        to_address: &str,
    ) -> Result<bool> {
        let admin_user_model = self
            .admin_user_repository
            .find_by_email(&db.0, &db.1, to_address)
            .await?;

        // is it ok to move this email as part of configuration on admin?
        // or may be moved this as part of the setup process?
        let from_address = String::from("info@avored.com");
        let email_subject = "Forgot your password?";
        let token = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(22)
            .map(char::from)
            .collect();

        let creatable_password_reset_model = CreatablePasswordResetModel {
            email: admin_user_model.clone().email,
            token,
        };

        // expiring any old token that might have been an active.
        // as most commonly user tries again as they have not received an email or received inside a spam inbox.
        self.password_reset_repository
            .expire_password_token_by_email(&db.0, &db.1, &admin_user_model.email)
            .await?;

        let password_reset_model = self
            .password_reset_repository
            .create_password_reset(&db.0, &db.1, creatable_password_reset_model)
            .await?;

        let link = format!(
            "{react_admin_url}/admin/reset-password/{}",
            password_reset_model.token
        );
        let data = ForgotPasswordViewModel { link };

        let forgot_password_email_content = template.handlebars.render("forgot-password", &data)?;
        let email_message = Message::builder().build_email_message(
            &from_address,
            &to_address,
            &email_subject,
            forgot_password_email_content,
        )?;

        // Send the email
        match template.mailer.send(email_message).await {
            Ok(_) => Ok(true),
            Err(er) => {
                error!("email sent error: {:?}", er);
                Err(Error::Generic(String::from("error while sending an email")))
            }
        }
    }
    pub(crate) async fn auth_user(
        &self,
        email: &str,
        password: &str,
        db: &DB,
        jwt_secret_key: &str,
    ) -> Result<String> {
        // Use multi-provider authentication
        let admin_user_model = match self
            .multi_auth_service
            .authenticate(email, password, db, None)
            .await
        {
            Ok(user) => user,
            Err(_e) => {
                let mut errors: Vec<ErrorMessage> = vec![];
                let error_message = ErrorMessage {
                    key: String::from("email"),
                    message: t!("email_address_password_not_match").to_string(),
                };

                errors.push(error_message);
                let error_response = ErrorResponse {
                    status: false,
                    errors,
                };
                let error_string = serde_json::to_string(&error_response)?;

                return Err(TonicError(Status::invalid_argument(error_string)));
            }
        };

        let claims: TokenClaims = admin_user_model.clone().try_into()?;

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
        )?;

        Ok(token)
    }

    pub fn compare_password(
        &self,
        plain_password: &str,
        encrypted_password: String,
    ) -> Result<bool> {
        let argon2 = Argon2::default();
        let parsed_hash = PasswordHash::new(&encrypted_password)?;

        Ok(argon2
            .verify_password(plain_password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    pub(crate) async fn reset_password(
        &self,
        db: &DB,
        email: &str,
        password: String,
        password_salt: &str,
        token: &str,
    ) -> Result<bool> {
        let password_hash = password.get_password_hash(password_salt)?;

        let status = self
            .admin_user_repository
            .update_password_by_email(&db.0, &db.1, email, password_hash)
            .await?;

        if !status {
            return Err(Error::Generic(String::from(
                "there is an issue while updating password.",
            )));
        }

        let expire_token_status = self
            .password_reset_repository
            .expire_password_token_by_email_and_token(&db.0, &db.1, email, token)
            .await?;

        Ok(expire_token_status)
    }

    pub(crate) async fn validate_token(
        &self,
        token: &str,
        email: &str,
        db: &DB,
    ) -> Result<bool> {
        match self
            .password_reset_repository
            .get_password_reset_by_email_and_token(&db.0, &db.1, email, token)
            .await
        {
            Ok(_model) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

impl AuthService {
    pub async fn new(
        admin_user_repository: AdminUserRepository,
        password_reset_repository: PasswordResetRepository,
    ) -> Result<AuthService> {
        // Initialize multi-provider authentication system
        let mut multi_auth_service = MultiAuthService::new();

        // Add local authentication provider (always enabled)
        let local_auth_service = LocalAuthService::new(admin_user_repository.clone());
        multi_auth_service.add_provider(Arc::new(local_auth_service));

        // Add LDAP authentication provider if enabled
        match LdapConfig::from_env() {
            Ok(ldap_config) => {
                if ldap_config.enabled {
                    let ldap_auth_service = LdapAuthService::new(ldap_config, admin_user_repository.clone());
                    multi_auth_service.add_provider(Arc::new(ldap_auth_service));
                }
            }
            Err(e) => {
                error!("Failed to load LDAP configuration: {}", e);
                // Continue without LDAP authentication
            }
        }

        Ok(AuthService {
            admin_user_repository,
            password_reset_repository,
            multi_auth_service,
        })
    }
}
