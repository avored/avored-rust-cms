use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use crate::{error::Result, providers::avored_database_provider::DB, repositories::admin_user_repository::AdminUserRepository, PER_PAGE};
use std::path::Path;
use crate::api::proto::admin_user::{AdminUserPaginateRequest, AdminUserPaginateResponse, GetAdminUserRequest, GetAdminUserResponse, GetRoleRequest, GetRoleResponse, PutRoleIdentifierRequest, PutRoleIdentifierResponse, RoleModel, RoleOptionModel, RoleOptionResponse, RolePaginateRequest, RolePaginateResponse, StoreAdminUserRequest, StoreAdminUserResponse, StoreRoleResponse, UpdateAdminUserRequest, UpdateAdminUserResponse, UpdateRoleRequest, UpdateRoleResponse};
use crate::api::proto::admin_user::admin_user_paginate_response::{AdminUserPaginateData, AdminUserPagination};
use crate::api::proto::admin_user::role_paginate_response::{RolePaginateData, RolePagination};
use crate::models::admin_user_model::{CreatableAdminUserModel, UpdatableAdminUserModel};
use crate::models::ModelCount;
use crate::models::role_model::{CreatableRole, PutRoleIdentifierModel, UpdatableRoleModel};
use crate::repositories::role_repository::RoleRepository;

pub struct AdminUserService {
    admin_user_repository: AdminUserRepository,
    role_repository: RoleRepository
}

impl AdminUserService {

    pub fn new(
        admin_user_repository: AdminUserRepository,
        role_repository: RoleRepository
    ) -> Result<Self> {
        Ok(AdminUserService {
            admin_user_repository,
            role_repository
        })
    }

    pub async fn paginate(
        &self,
        req: AdminUserPaginateRequest,
        (datastore, database_session): &DB,
    ) -> Result<AdminUserPaginateResponse> {
        let admin_user_model_count   = self
            .admin_user_repository
            .get_total_count(datastore, database_session)
            .await?;

        let per_page: i64 = PER_PAGE as i64;
        let current_page = req.page.unwrap_or(0);
        let order = req.order.unwrap_or_default();
        
        let start = current_page * per_page;
        let mut order_column = "id";
        let mut order_type = "desc";
        if !order.is_empty() {
            let mut parts = order.split(':');
            if parts.clone().count() == 2 {
                order_column = parts.clone().nth(0).unwrap_or("");
                order_type = parts.nth(1).unwrap_or("");
            }
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

        let mut grpc_admin_users = vec![];
        admin_users.iter().for_each(|admin_user| {
            let model: crate::api::proto::admin_user::AdminUserModel = admin_user.clone().try_into().unwrap();
            grpc_admin_users.push(model);
        });

        
        let pagination = AdminUserPagination {
            total: admin_user_model_count.total,
        };
        let paginate_data = AdminUserPaginateData {
            pagination: Option::from(pagination),
            data: grpc_admin_users,
        };
        
        let res = AdminUserPaginateResponse {
            status: true,
            data: Option::from(paginate_data),
        };
        
        Ok(res)
    }

    pub async fn store (
        &self,
        req: StoreAdminUserRequest,
        logged_in_username: String,
        password_salt: &str,
        (datastore, database_session): &DB,
    ) -> Result<StoreAdminUserResponse> {

        let password_hash = self
            .get_password_hash_from_raw_password(&req.password, password_salt)?;

        let mut created_admin_user_model = CreatableAdminUserModel {
            full_name: req.full_name,
            email: req.email,
            password: password_hash,
            profile_image: String::from(""),
            is_super_admin: req.is_super_admin,
            logged_in_username,
        };

        if !req.profile_image_file_name.is_empty() {
            let profile_image = format!("upload/{}", req.profile_image_file_name.clone());
            let full_path = Path::new("public").join("upload").join(req.profile_image_file_name);

            tokio::fs::write(full_path, &req.profile_image_content).await?;

            created_admin_user_model.profile_image = profile_image;
        }

        let admin_user_model = self
            .admin_user_repository
            .create_admin_user(
                datastore,
                database_session,
                created_admin_user_model,
            )
            .await?;

        let model: crate::api::proto::admin_user::AdminUserModel = admin_user_model.clone().try_into().unwrap();


        let res = StoreAdminUserResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }

    pub async fn find_admin_user_by_id (
        &self,
        req: GetAdminUserRequest,
        (datastore, database_session): &DB,
    ) -> Result<GetAdminUserResponse> {

        let admin_user_model = self
            .admin_user_repository
            .find_by_id(
                datastore,
                database_session,
                req.admin_user_id,
            )
            .await?;

        let model: crate::api::proto::admin_user::AdminUserModel = admin_user_model.try_into().unwrap();


        let res = GetAdminUserResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }

    pub  async fn update_admin_user(
        &self,
        req: UpdateAdminUserRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<UpdateAdminUserResponse> {

        let mut updatable_admin_user_model = UpdatableAdminUserModel {
            id: req.admin_user_id,
            full_name: req.full_name,
            profile_image: String::from(""),
            is_super_admin: req.is_super_admin,
            logged_in_username: logged_in_username.clone(),
            role_ids: req.role_ids,
        };
        
        println!("UP: {:?}", updatable_admin_user_model);
        

        // needs to handle the existing image scenario

        if !req.profile_image_file_name.is_empty() {
            let profile_image = format!("upload/{}", req.profile_image_file_name.clone());
            let full_path = Path::new("public").join("upload").join(req.profile_image_file_name);

            tokio::fs::write(full_path, &req.profile_image_content).await?;

            updatable_admin_user_model.profile_image = profile_image;
        }

        let admin_user_model = self
            .admin_user_repository
            .update_admin_user(
                datastore,
                database_session,
                updatable_admin_user_model.clone()
            )
            .await?;

        let mut model: crate::api::proto::admin_user::AdminUserModel = admin_user_model.clone().try_into().unwrap();

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
        
        for role_id in &updatable_admin_user_model.role_ids {
            let role_model = self
                .role_repository
                .find_by_id(datastore, database_session, role_id.to_string())
                .await?;
            
            let grpc_role_model: RoleModel = role_model.clone().try_into().unwrap();
            let logged_in_user = logged_in_username.clone();
            self.admin_user_repository
                .attach_admin_user_with_role(
                    datastore,
                    database_session,
                    admin_user_model.clone().id,
                    role_model.clone().id,
                    &logged_in_user,
                )
                .await?;
        
            model.roles.push(grpc_role_model);
        }
        
        
        

        let res = UpdateAdminUserResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }

    pub async fn role_paginate(
        &self,
        req: RolePaginateRequest,
        (datastore, database_session): &DB,
    ) -> Result<RolePaginateResponse> {
        let role_model_count   = self
            .role_repository
            .get_total_count(datastore, database_session)
            .await?;

        let per_page: i64 = PER_PAGE as i64;
        let current_page = req.page.unwrap_or(0);
        let order = req.order.unwrap_or_default();

        let start = current_page * per_page;
        let mut order_column = "id";
        let mut order_type = "desc";
        if !order.is_empty() {
            let mut parts = order.split(':');
            if parts.clone().count() == 2 {
                order_column = parts.clone().nth(0).unwrap_or("");
                order_type = parts.nth(1).unwrap_or("");
            }
        }
        

        let roles = self
            .role_repository
            .paginate(
                datastore,
                database_session,
                start,
                order_column.to_string(),
                order_type.to_string(),
            )
            .await?;

        let mut grpc_roles = vec![];
        roles.iter().for_each(|role| {
            let model: RoleModel = role.clone().try_into().unwrap();
            grpc_roles.push(model);
        });


        let pagination = RolePagination {
            total: role_model_count.total,
        };
        let paginate_data = RolePaginateData {
            pagination: Option::from(pagination),
            data: grpc_roles,
        };

        let res = RolePaginateResponse {
            status: true,
            data: Option::from(paginate_data),
        };

        Ok(res)
    }

    pub async fn role_option(
        &self,
        (datastore, database_session): &DB,
    ) -> Result<RoleOptionResponse> {


        let roles = self
            .role_repository
            .all(
                datastore,
                database_session
            )
            .await?;

        let mut grpc_role_options = vec![];
        roles.iter().for_each(|role| {
            let model = RoleOptionModel {
                value: role.id.clone(),
                label: role.name.clone()
            };
            grpc_role_options.push(model);
        });


        let res = RoleOptionResponse {
            status: true,
            data: grpc_role_options,
        };

        Ok(res)
    }

    pub async fn store_role (
        &self,
        created_role_request: CreatableRole,
        (datastore, database_session): &DB,
    ) -> Result<StoreRoleResponse> {

       
        let admin_user_model = self
            .role_repository
            .create_role(
                datastore,
                database_session,
                created_role_request,
            )
            .await?;

        let model: RoleModel = admin_user_model.clone().try_into().unwrap();


        let res = StoreRoleResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }

    pub async fn find_role_by_id (
        &self,
        req: GetRoleRequest,
        (datastore, database_session): &DB,
    ) -> Result<GetRoleResponse> {

        let admin_user_model = self
            .role_repository
            .find_by_id(
                datastore,
                database_session,
                req.role_id,
            )
            .await?;

        let model: RoleModel = admin_user_model.try_into().unwrap();

        let res = GetRoleResponse {
            status: true,
            data: Option::from(model)
        };
        
        Ok(res)
    }

    pub  async fn update_role(
        &self,
        req: UpdateRoleRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<UpdateRoleResponse> {

        let updatable_role_model = UpdatableRoleModel {
            id: req.role_id,
            name: req.name,
            permissions: req.permissions,
            logged_in_username: logged_in_username.clone(),
        };
        

        let role_model = self
            .role_repository
            .update_role(
                datastore,
                database_session,
                updatable_role_model.clone()
            )
            .await?;

        let model: RoleModel = role_model.clone().try_into().unwrap();
        
        let res = UpdateRoleResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }

    pub  async fn put_role_identifier(
        &self,
        req: PutRoleIdentifierRequest,
        logged_in_username: String,
        (datastore, database_session): &DB,
    ) -> Result<PutRoleIdentifierResponse> {

        let updatable_role_model = PutRoleIdentifierModel {
            id: req.role_id,
            identifier: req.identifier,
            logged_in_username: logged_in_username.clone(),
        };


        let role_model = self
            .role_repository
            .update_role_identifier(
                datastore,
                database_session,
                updatable_role_model.clone()
            )
            .await?;

        let model: RoleModel = role_model.clone().try_into().unwrap();

        let res = PutRoleIdentifierResponse {
            status: true,
            data: Option::from(model)
        };
        Ok(res)
    }


    // pub async fn sent_forgot_password_email (
    //     &self,
    //     (datastore, database_session): &DB,
    //     template: &AvoRedTemplateProvider,
    //     react_admin_url: &str,
    //     to_address: &str,
    // ) -> Result<bool> {
    //
    //     let admin_user_model = self
    //         .admin_user_repository
    //         .find_by_email(datastore, database_session, to_address)
    //         .await?;
    //
    //     let from_address = String::from("info@avored.com");
    //     let email_subject = "Forgot your password?";
    //     let token = rand::rng()
    //         .sample_iter(&Alphanumeric)
    //         .take(22)
    //         .map(char::from)
    //         .collect();
    //
    //     let creatable_password_reset_model = CreatablePasswordResetModel {
    //         email: admin_user_model.email,
    //         token,
    //     };
    //
    //     //@todo before creating a token make sure expire any past token that could have been generated?
    //
    //     let password_reset_model = self
    //         .password_reset_repository
    //         .create_password_reset(datastore, database_session, creatable_password_reset_model)
    //         .await?;
    //
    //     let link = format!(
    //         "{react_admin_url}/admin/reset-password/{}",
    //         password_reset_model.token
    //     );
    //     let data = ForgotPasswordViewModel { link };
    //
    //     let forgot_password_email_content = template.handlebars.render("forgot-password", &data)?;
    //
    //     let email = Message::builder()
    //         .from(from_address.parse()?)
    //         .to(to_address.parse()?)
    //         .subject(email_subject)
    //         .multipart(
    //             MultiPart::alternative()
    //                 // .singlepart(
    //                 //     SinglePart::builder()
    //                 //         .header(header::ContentType::TEXT_PLAIN)
    //                 //         .body(String::from("Hello from Lettre! A mailer library for Rust")), // Every message should have a plain text fallback.
    //                 // )
    //                 .singlepart(
    //                     SinglePart::builder()
    //                         .header(header::ContentType::TEXT_HTML)
    //                         .body(forgot_password_email_content),
    //                 ),
    //         )?;
    //
    //     // Send the email
    //     match template.mailer.send(email).await {
    //         Ok(_) => Ok(true),
    //         Err(_) => Err(Error::Generic(String::from("error while sending an email"))),
    //     }
    // }
    //
    // pub fn compare_password(
    //     &self,
    //     plain_password: String,
    //     encrypted_password: String,
    // ) -> Result<bool> {
    //     let argon2 = Argon2::default();
    //
    //     let parsed_hash = PasswordHash::new(&encrypted_password)?;
    //
    //     Ok(argon2
    //         .verify_password(plain_password.as_bytes(), &parsed_hash)
    //         .is_ok())
    // }
    //
    pub async fn has_permission(
        &self,
        logged_in_user: crate::models::admin_user_model::AdminUserModel,
        permission_identifier: String,
    ) -> Result<bool> {
        if logged_in_user.is_super_admin {
            return Ok(true);
        }
        let mut has_permission = false;
        for role in logged_in_user.roles {
            for permission in role.permissions {
                if permission == permission_identifier {
                    has_permission = true;
                }
            }
        }
    
        Ok(has_permission)
    }
    
    // pub async fn auth_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     payload: AuthenticateAdminUserRequest,
    //     jwt_secret_key: &str
    // ) -> Result<LoginResponseData> {
    //
    //     let admin_user_model = self
    //         .admin_user_repository
    //         .find_by_email(datastore, database_session, &payload.email)
    //         .await?;
    //
    //     let is_password_match: bool = self
    //         .compare_password(payload.password, admin_user_model.password.clone())?;
    //
    //     if !is_password_match {
    //         return Err(Error::Authentication("Email and password did not match".to_string()));
    //     }
    //
    //     let claims: TokenClaims = admin_user_model.clone().try_into()?;
    //
    //     let token = encode(
    //         &Header::default(),
    //         &claims,
    //         &EncodingKey::from_secret(jwt_secret_key.as_bytes()),
    //     )?;
    //     let cookie = Cookie::build("token")
    //         .path("/")
    //         .same_site(SameSite::Lax)
    //         .http_only(true);
    //     let mut response = Response::new(json!({"status": "success", "token": token}).to_string());
    //
    //     response
    //         .headers_mut()
    //         .insert(AxumHeader::SET_COOKIE, cookie.to_string().parse().unwrap());
    //     let response_data = LoginResponseData {
    //         status: true,
    //         data: token,
    //         admin_user: admin_user_model,
    //     };
    //
    //     Ok(response_data)
    // }
    //
    // pub async fn find_by_id(
    //     &self,
    //     (datastore, database_session): &DB,
    //     id: String,
    // ) -> Result<AdminUserModel> {
    //     self.admin_user_repository
    //         .find_by_id(datastore, database_session, id)
    //         .await
    // }
    //
    // pub async fn get_password_reset_by_email(
    //     &self,
    //     (datastore, database_session): &DB,
    //     email: String,
    //     token: String,
    // ) -> Result<PasswordResetModel> {
    //     self.password_reset_repository
    //         .get_password_reset_by_email(datastore, database_session, email, token)
    //         .await
    // }
    //
    // pub async fn update_password_by_email(
    //     &self,
    //     (datastore, database_session): &DB,
    //     new_password: String,
    //     email: String,
    // ) -> Result<bool> {
    //     self.admin_user_repository
    //         .update_password_by_email(datastore, database_session, new_password, email)
    //         .await
    // }
    //
    // pub async fn expire_password_token_by_email_and_token(
    //     &self,
    //     (datastore, database_session): &DB,
    //     email: String,
    //     token: String,
    // ) -> Result<bool> {
    //     self.password_reset_repository
    //         .expire_password_token_by_email_and_token(datastore, database_session, email, token)
    //         .await
    // }
    //
    // pub async fn update_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     updatable_admin_user_model: UpdatableAdminUserModel,
    //     logged_in_user: LoggedInUser,
    // ) -> Result<AdminUserModel> {
    //     let mut admin_user_model = self
    //         .admin_user_repository
    //         .update_admin_user(
    //             datastore,
    //             database_session,
    //             updatable_admin_user_model.clone(),
    //         )
    //         .await?;
    //
    //     for role_id in updatable_admin_user_model.clone().role_ids {
    //         self.admin_user_repository
    //             .detach_admin_user_with_role(
    //                 datastore,
    //                 database_session,
    //                 admin_user_model.clone().id,
    //                 role_id,
    //             )
    //             .await?;
    //     }
    //
    //     for role_id in updatable_admin_user_model.role_ids {
    //         let role_model = self
    //             .role_repository
    //             .find_by_id(datastore, database_session, role_id)
    //             .await?;
    //         self.admin_user_repository
    //             .attach_admin_user_with_role(
    //                 datastore,
    //                 database_session,
    //                 admin_user_model.clone().id,
    //                 role_model.clone().id,
    //                 logged_in_user.clone(),
    //             )
    //             .await?;
    //
    //         admin_user_model.roles.push(role_model);
    //     }
    //
    //     Ok(admin_user_model)
    // }
    //
    // pub async fn paginate(
    //     &self,
    //     (datastore, database_session): &DB,
    //     current_page: i64,
    //     order: String,
    // ) -> Result<AdminUserPagination> {
    //     let start = current_page * PER_PAGE;
    //     let to = start + PER_PAGE;
    //
    //     let admin_user_model_count = self
    //         .admin_user_repository
    //         .get_total_count(datastore, database_session)
    //         .await?;
    //
    //     let mut has_next_page = false;
    //     if admin_user_model_count.total > to {
    //         has_next_page = true;
    //     };
    //     let mut has_previous_page = false;
    //     if current_page > 0 {
    //         has_previous_page = true;
    //     };
    //
    //     let pagination = Pagination {
    //         total: admin_user_model_count.total,
    //         per_page: PER_PAGE,
    //         current_page,
    //         from: (start + 1),
    //         to,
    //         has_previous_page,
    //         next_page_number: (current_page + 1),
    //         has_next_page,
    //         previous_page_number: (current_page - 1),
    //     };
    //
    //     let mut order_column = "id";
    //     let mut order_type = "ASC";
    //     let mut parts = order.split(':');
    //     if parts.clone().count() == 2 {
    //         order_column = parts.clone().nth(0).unwrap_or("");
    //         order_type = parts.nth(1).unwrap_or("");
    //     }
    //
    //     let admin_users = self
    //         .admin_user_repository
    //         .paginate(
    //             datastore,
    //             database_session,
    //             start,
    //             order_column.to_string(),
    //             order_type.to_string(),
    //         )
    //         .await?;
    //
    //     Ok(AdminUserPagination {
    //         data: admin_users,
    //         pagination,
    //     })
    // }
    //
    // pub async fn create_admin_user(
    //     &self,
    //     (datastore, database_session): &DB,
    //     creatable_admin_user_model: CreatableAdminUserModel,
    //     logged_in_user: LoggedInUser,
    // ) -> Result<AdminUserModel> {
    //     let mut admin_user_model = self
    //         .admin_user_repository
    //         .create_admin_user(
    //             datastore,
    //             database_session,
    //             creatable_admin_user_model.clone(),
    //         )
    //         .await?;
    //     for role_id in creatable_admin_user_model.role_ids {
    //         let role_model = self
    //             .role_repository
    //             .find_by_id(datastore, database_session, role_id)
    //             .await?;
    //         self.admin_user_repository
    //             .attach_admin_user_with_role(
    //                 datastore,
    //                 database_session,
    //                 admin_user_model.clone().id,
    //                 role_model.clone().id,
    //                 logged_in_user.clone(),
    //             )
    //             .await?;
    //
    //         admin_user_model.roles.push(role_model);
    //     }
    //
    //     Ok(admin_user_model)
    // }
    //
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
    //
    pub async fn count_of_email(
        &self,
        (datastore, database_session): &DB,
        email: String,
    ) -> Result<ModelCount> {
        self.admin_user_repository
            .count_of_email(datastore, database_session, email)
            .await
    }

    pub async fn count_of_role_identifier(
        &self,
        (datastore, database_session): &DB,
        identifier: &str,
    ) -> Result<ModelCount> {
        self.role_repository
            .count_of_identifier(datastore, database_session, identifier)
            .await
    }

    //count_of_identifier

    //
    // pub async fn reset_password(
    //     &self,
    //     password: &str,
    //     password_salt: &str,
    // ) -> Result<()> {
    //
    //     let password_hash =
    //         self.get_password_hash_from_raw_password(password, password_salt)?;
    //
    //     Ok(())
    // }
}
