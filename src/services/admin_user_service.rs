use crate::api::handlers::admin_user::admin_user_forgot_password_api_handler::ForgotPasswordViewModel;
use crate::error::Error;
use crate::models::admin_user_model::CreatableAdminUserModel;
use crate::models::password_rest_model::{CreatablePasswordResetModel, PasswordResetModel};
use crate::models::token_claim_model::{LoggedInUser, TokenClaims};
use crate::models::ModelCount;
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::role_repository::RoleRepository;
use axum::http::{header as AxumHeader, Response};
use crate::{
    error::Result,
    models::{
        admin_user_model::{AdminUserModel, AdminUserPagination, UpdatableAdminUserModel},
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::admin_user_repository::AdminUserRepository,
    PER_PAGE,
};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{AsyncTransport, Message};
use rand::distr::Alphanumeric;
use rand::Rng;
use serde_json::json;
use crate::api::handlers::admin_user::admin_user_login_api_handler::LoginResponseData;
use crate::api::handlers::admin_user::request::authenticate_admin_user_request::AuthenticateAdminUserRequest;

pub struct AdminUserService {
    admin_user_repository: AdminUserRepository,
    role_repository: RoleRepository,
    password_reset_repository: PasswordResetRepository,
}

impl AdminUserService {

    pub fn new(
        admin_user_repository: AdminUserRepository,
        role_repository: RoleRepository,
        password_reset_repository: PasswordResetRepository,
    ) -> Result<Self> {
        Ok(AdminUserService {
            admin_user_repository,
            role_repository,
            password_reset_repository,
        })
    }

    pub async fn sent_forgot_password_email(
        &self,
        (datastore, database_session): &DB,
        template: &AvoRedTemplateProvider,
        react_admin_url: &str,
        to_address: &str,
    ) -> Result<bool> {

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
            Ok(_) => Ok(true),
            Err(_) => Err(Error::Generic(String::from("error while sending an email"))),
        }
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

    pub async fn has_permission(
        &self,
        logged_in_user: LoggedInUser,
        permission_identifier: String,
    ) -> Result<bool> {
        if logged_in_user.admin_user_model.is_super_admin {
            return Ok(true);
        }
        let mut has_permission = false;
        for role in logged_in_user.admin_user_model.roles {
            for permission in role.permissions {
                if permission == permission_identifier {
                    has_permission = true;
                }
            }
        }

        Ok(has_permission)
    }

    pub async fn auth_admin_user(
        &self,
        (datastore, database_session): &DB,
        payload: AuthenticateAdminUserRequest,
        jwt_secret_key: &str
    ) -> Result<LoginResponseData> {

        let admin_user_model = self
            .admin_user_repository
            .find_by_email(datastore, database_session, &payload.email)
            .await?;

        let is_password_match: bool = self
            .compare_password(payload.password, admin_user_model.password.clone())?;

        if !is_password_match {
            return Err(Error::Authentication("Email and password did not match".to_string()));
        }

        let claims: TokenClaims = admin_user_model.clone().try_into()?;

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
        )?;
        let cookie = Cookie::build("token")
            .path("/")
            .same_site(SameSite::Lax)
            .http_only(true);
        let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

        response
            .headers_mut()
            .insert(AxumHeader::SET_COOKIE, cookie.to_string().parse().unwrap());
        let response_data = LoginResponseData {
            status: true,
            data: token,
            admin_user: admin_user_model,
        };

        Ok(response_data)
    }

    pub async fn find_by_id(
        &self,
        (datastore, database_session): &DB,
        id: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_id(datastore, database_session, id)
            .await
    }

    pub async fn get_password_reset_by_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
        token: String,
    ) -> Result<PasswordResetModel> {
        self.password_reset_repository
            .get_password_reset_by_email(datastore, database_session, email, token)
            .await
    }

    pub async fn update_password_by_email(
        &self,
        (datastore, database_session): &DB,
        new_password: String,
        email: String,
    ) -> Result<bool> {
        self.admin_user_repository
            .update_password_by_email(datastore, database_session, new_password, email)
            .await
    }

    pub async fn expire_password_token_by_email_and_token(
        &self,
        (datastore, database_session): &DB,
        email: String,
        token: String,
    ) -> Result<bool> {
        self.password_reset_repository
            .expire_password_token_by_email_and_token(datastore, database_session, email, token)
            .await
    }

    pub async fn update_admin_user(
        &self,
        (datastore, database_session): &DB,
        updatable_admin_user_model: UpdatableAdminUserModel,
        logged_in_user: LoggedInUser,
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self
            .admin_user_repository
            .update_admin_user(
                datastore,
                database_session,
                updatable_admin_user_model.clone(),
            )
            .await?;

        for role_id in updatable_admin_user_model.clone().role_ids {
            self.admin_user_repository
                .detach_admin_user_with_role(
                    datastore,
                    database_session,
                    admin_user_model.clone().id,
                    role_id,
                )
                .await?;
        }

        for role_id in updatable_admin_user_model.role_ids {
            let role_model = self
                .role_repository
                .find_by_id(datastore, database_session, role_id)
                .await?;
            self.admin_user_repository
                .attach_admin_user_with_role(
                    datastore,
                    database_session,
                    admin_user_model.clone().id,
                    role_model.clone().id,
                    logged_in_user.clone(),
                )
                .await?;

            admin_user_model.roles.push(role_model);
        }

        Ok(admin_user_model)
    }

    pub async fn paginate(
        &self,
        (datastore, database_session): &DB,
        current_page: i64,
        order: String,
    ) -> Result<AdminUserPagination> {
        let start = current_page * PER_PAGE;
        let to = start + PER_PAGE;

        let admin_user_model_count = self
            .admin_user_repository
            .get_total_count(datastore, database_session)
            .await?;

        let mut has_next_page = false;
        if admin_user_model_count.total > to {
            has_next_page = true;
        };
        let mut has_previous_page = false;
        if current_page > 0 {
            has_previous_page = true;
        };

        let pagination = Pagination {
            total: admin_user_model_count.total,
            per_page: PER_PAGE,
            current_page,
            from: (start + 1),
            to,
            has_previous_page,
            next_page_number: (current_page + 1),
            has_next_page,
            previous_page_number: (current_page - 1),
        };

        let mut order_column = "id";
        let mut order_type = "ASC";
        let mut parts = order.split(':');
        if parts.clone().count() == 2 {
            order_column = parts.clone().nth(0).unwrap_or("");
            order_type = parts.nth(1).unwrap_or("");
        }

        let admin_users = self
            .admin_user_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        Ok(AdminUserPagination {
            data: admin_users,
            pagination,
        })
    }

    pub async fn create_admin_user(
        &self,
        (datastore, database_session): &DB,
        creatable_admin_user_model: CreatableAdminUserModel,
        logged_in_user: LoggedInUser,
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self
            .admin_user_repository
            .create_admin_user(
                datastore,
                database_session,
                creatable_admin_user_model.clone(),
            )
            .await?;
        for role_id in creatable_admin_user_model.role_ids {
            let role_model = self
                .role_repository
                .find_by_id(datastore, database_session, role_id)
                .await?;
            self.admin_user_repository
                .attach_admin_user_with_role(
                    datastore,
                    database_session,
                    admin_user_model.clone().id,
                    role_model.clone().id,
                    logged_in_user.clone(),
                )
                .await?;

            admin_user_model.roles.push(role_model);
        }

        Ok(admin_user_model)
    }

    pub fn get_password_hash_from_raw_password(
        &self,
        raw_password: &str,
        password_salt: &str,
    ) -> Result<String> {
        let password = raw_password.as_bytes();
        let salt = SaltString::from_b64(password_salt)?;

        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        Ok(password_hash)
    }

    pub async fn count_of_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
    ) -> Result<ModelCount> {
        self.admin_user_repository
            .count_of_email(datastore, database_session, email)
            .await
    }

    pub async fn reset_password(
        &self,
        password: &str,
        password_salt: &str,
    ) -> Result<()> {

        let password_hash =
            self.get_password_hash_from_raw_password(password, password_salt)?;

        Ok(())
    }
}
