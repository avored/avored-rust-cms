use argon2::{Argon2, PasswordHash, PasswordVerifier};
use lettre::{AsyncTransport, Message};
use lettre::message::{header, MultiPart, SinglePart};
use rand::distributions::{Alphanumeric, DistString};
use crate::{
    error::Result,
    models::{
        admin_user_model::{
            AdminUserModel, AdminUserPagination, UpdatableAdminUserModel,
        },
        Pagination,
    },
    providers::avored_database_provider::DB,
    repositories::admin_user_repository::AdminUserRepository,
    PER_PAGE,
};
use crate::api::handlers::admin_user::admin_user_forgot_password_api_handler::ForgotPasswordViewModel;
use crate::error::Error;
use crate::models::admin_user_model::CreatableAdminUserModel;
use crate::models::password_rest_model::{CreatablePasswordResetModel, PasswordResetModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::providers::avored_template_provider::AvoRedTemplateProvider;
use crate::repositories::password_reset_repository::PasswordResetRepository;
use crate::repositories::role_repository::RoleRepository;

pub struct AdminUserService {
    admin_user_repository: AdminUserRepository,
    role_repository: RoleRepository,
    password_reset_repository: PasswordResetRepository
}

impl AdminUserService {
    pub fn new(
        admin_user_repository: AdminUserRepository,
        role_repository: RoleRepository,
        password_reset_repository: PasswordResetRepository
    ) -> Result<Self> {
        Ok(AdminUserService {
            admin_user_repository,
            role_repository,
            password_reset_repository
        })
    }
}
impl AdminUserService {
    pub async fn sent_forgot_password_email(
        &self,
        (datastore, database_session): &DB,
        template: &AvoRedTemplateProvider,
        front_end_url: &str,
        to_address: String
    ) -> Result<bool>
    {
        let from_address = String::from("info@avored.com");
        let email_subject = "Forgot your password?";

        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 22);

        let creatable_password_reset_model = CreatablePasswordResetModel {
            email: to_address.clone(),
            token,
        };

        let password_reset_model = self
            .password_reset_repository
            .create_password_reset(datastore, database_session, creatable_password_reset_model)
            .await?;

        let link = format!("{front_end_url}/admin/reset-password/{}", password_reset_model.token);
        let data = ForgotPasswordViewModel {
            link
        };

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

    pub fn compare_password(&self, plain_password: String, encrypted_password: String) -> Result<bool>
    {
        let argon2 = Argon2::default();

        let parsed_hash = PasswordHash::new(&encrypted_password)?;

        Ok(argon2.verify_password(plain_password.as_bytes(), &parsed_hash).is_ok())
    }
    pub async fn has_permission(
        &self,
        logged_in_user: LoggedInUser,
        permission_identifier: String
    ) -> Result<bool> {
        if logged_in_user.admin_user_model.is_super_admin {
            return Ok(true)
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
    pub async fn find_by_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
    ) -> Result<AdminUserModel> {
        self.admin_user_repository
            .find_by_email(datastore, database_session, email)
            .await
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
    ) -> Result<PasswordResetModel> {
        self.password_reset_repository
            .get_password_reset_by_email(datastore, database_session, email)
            .await
    }

    pub async fn update_password_by_email(
        &self,
        (datastore, database_session): &DB,
        new_password: String,
        email: String
    ) -> Result<bool> {
        self.admin_user_repository
            .update_password_by_email(datastore, database_session, new_password, email)
            .await
    }

    pub async fn update_admin_user(
        &self,
        (datastore, database_session): &DB,
        updatable_admin_user_model: UpdatableAdminUserModel,
        logged_in_user: LoggedInUser
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self.admin_user_repository
            .update_admin_user(datastore, database_session, updatable_admin_user_model.clone())
            .await?;

        for role_id in updatable_admin_user_model.clone().role_ids {
            self.admin_user_repository
                .detach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_id)
                .await?;
        }

        for role_id in updatable_admin_user_model.role_ids {
            let role_model = self.role_repository.find_by_id(datastore, database_session, role_id).await?;
            self.admin_user_repository
                .attach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_model.clone().id, logged_in_user.clone())
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
        let mut order_type  = "ASC";
        let mut parts = order.split(':');
        if parts.clone().count() == 2 {
            order_column = parts.clone().nth(0).unwrap_or("");
            order_type = parts.nth(1).unwrap_or("");
        }

        let admin_users = self
            .admin_user_repository
            .paginate(datastore, database_session, start, order_column.to_string(), order_type.to_string())
            .await?;

        Ok(AdminUserPagination {
            data: admin_users,
            pagination,
        })
    }

    // pub async fn delete_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     admin_user_id: String,
    // ) -> Result<bool> {
    //     self.admin_user_repository
    //         .delete_admin_user(datastore, database_session, admin_user_id)
    //         .await
    // }

    pub async fn create_admin_user(
        &self,
        (datastore, database_session): &DB,
        creatable_admin_user_model: CreatableAdminUserModel,
        logged_in_user: LoggedInUser
    ) -> Result<AdminUserModel> {
        let mut admin_user_model = self.admin_user_repository
            .create_admin_user(datastore, database_session, creatable_admin_user_model.clone())
            .await?;
        for role_id in creatable_admin_user_model.role_ids {
            let role_model = self.role_repository.find_by_id(datastore, database_session, role_id).await?;
            self.admin_user_repository
                .attach_admin_user_with_role(datastore, database_session, admin_user_model.clone().id, role_model.clone().id, logged_in_user.clone())
                .await?;

            admin_user_model.roles.push(role_model);
        }

        Ok(admin_user_model)
    }
}
