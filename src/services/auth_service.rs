use argon2::{Argon2, PasswordHash, PasswordVerifier};
use crate::error::{Error, Result};
use crate::providers::avored_database_provider::DB;
use crate::repositories::admin_user_repository::AdminUserRepository;
use crate::models::token_claim_model::TokenClaims;
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::{AsyncTransport, Message};
use lettre::message::{header, MultiPart, SinglePart};
use rand::distr::Alphanumeric;
use rand::Rng;
use rust_i18n::t;
use tonic::Status;
use crate::api::proto::auth::{ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse, ResetPasswordRequest, ResetPasswordResponse};
use crate::error::Error::TonicError;
use crate::models::password_rest_model::{CreatablePasswordResetModel, ForgotPasswordViewModel};
use crate::models::validation_error::{ErrorMessage, ErrorResponse};
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::password_reset_repository::PasswordResetRepository;

pub struct AuthService {
    admin_user_repository: AdminUserRepository,
    password_reset_repository: PasswordResetRepository,
}

impl AuthService {
    
    pub async fn forgot_password(
        &self,
        (datastore, database_session): &DB,
        template: &AvoRedTemplateProvider,
        react_admin_url: &str,
        to_address: &str,
    ) -> Result<ForgotPasswordResponse> {

        let admin_user_model = self
            .admin_user_repository
            .find_by_email(datastore, database_session, to_address)
            .await?;

        let from_address = String::from("info@avored.com");
        let email_subject = "Forgot your password?";
        let token = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(22)
            .map(char::from)
            .collect();

        let creatable_password_reset_model = CreatablePasswordResetModel {
            email: admin_user_model.email,
            token,
        };

        //@todo before creating a token make sure expire any past token that could have been generated?

        let password_reset_model = self
            .password_reset_repository
            .create_password_reset(datastore, database_session, creatable_password_reset_model)
            .await?;

        let link = format!(
            "{react_admin_url}/admin/reset-password/{}",
            password_reset_model.token
        );
        let data = ForgotPasswordViewModel { link };

        let forgot_password_email_content = template.handlebars.render("forgot-password", &data)?;

        let email = Message::builder()
            .from(from_address.parse()?)
            .to(to_address.parse()?)
            .subject(email_subject)
            .multipart(
                MultiPart::alternative()
                    // .singlepart(
                    //     SinglePart::builder()
                    //         .header(header::ContentType::TEXT_PLAIN)
                    //         .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
                    // )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(forgot_password_email_content),
                    ),
            )?;

        // Send the email
        match template.mailer.send(email).await {
            Ok(_) => {
                let response = ForgotPasswordResponse {
                    status: true
                };
                return Ok(response);
            },
            Err(er) => {
                println!("email sent error: {:?}", er);
                Err(Error::Generic(String::from("error while sending an email")))
            },
        }
    }
    pub(crate) async fn auth_user(
        &self,
        request: LoginRequest,
        (datastore, database_session): &DB,
        jwt_secret_key: &str
    ) -> Result<LoginResponse> {

        println!("request: {:?}", request);
        let admin_user_model = self
        .admin_user_repository
        .find_by_email(datastore, database_session, &request.email)
        .await?;

        let is_password_match: bool = self
            .compare_password(request.password, admin_user_model.password.clone())?;

        if !is_password_match {
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

        println!("login response: {:?}", login_response);
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

    pub(crate) async fn reset_password(
        &self,
        (datastore, database_session): &DB,
        email: String,
        password_hash: String,
    ) -> Result<ResetPasswordResponse> {
        let status = self.
            admin_user_repository.
            update_password_by_email(
                datastore, 
                database_session,
                email,
                password_hash
            ).await?;
        
        let res = ResetPasswordResponse {
            status
        };
        
        Ok(res)
    }

}

impl AuthService {
    pub async fn new(admin_user_repository: AdminUserRepository, password_reset_repository: PasswordResetRepository) -> Result<AuthService> {
        Ok(AuthService {admin_user_repository, password_reset_repository})
    }
}