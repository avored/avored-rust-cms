use crate::api::proto::admin_user::admin_user_paginate_response::{
    AdminUserPaginateData, AdminUserPagination,
};
use crate::api::proto::admin_user::admin_user_server::AdminUser;
use crate::api::proto::admin_user::{
    AdminUserPaginateRequest, AdminUserPaginateResponse, ChangeAdminUserPasswordRequest, ChangeAdminUserPasswordResponse, DeleteAdminUserRequest, DeleteAdminUserResponse, DeleteRoleRequest, DeleteRoleResponse, GetAdminUserRequest, GetAdminUserResponse, GetRoleRequest, GetRoleResponse, PutRoleIdentifierRequest, PutRoleIdentifierResponse, RoleOptionRequest, RoleOptionResponse, RolePaginateRequest, RolePaginateResponse, StoreAdminUserRequest, StoreAdminUserResponse, StoreRoleRequest, StoreRoleResponse, UpdateAdminUserRequest, UpdateAdminUserResponse, UpdateRoleRequest, UpdateRoleResponse
};
use crate::avored_state::AvoRedState;
use crate::error::Error::Tonic;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use crate::models::role_model::CreatableRole;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use tracing::info;

/// `AdminUserApi` is the gRPC API for managing admin users and roles.
pub struct AdminUserApi {
    /// The shared state of the application, containing services and configurations.
    pub state: Arc<AvoRedState>,
}
#[async_trait]
impl AdminUser for AdminUserApi {

    async fn paginate(
        &self,
        request: Request<AdminUserPaginateRequest>,
    ) -> Result<Response<AdminUserPaginateResponse>, Status> {
        println!(
            "->> {:<12} - paginate_admin_user",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;

        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("paginate_admin_user"),
            )
            .await?;

        let req = request.into_inner();
        let page = req.page.unwrap_or_default();
        let order = req.order.unwrap_or_default();

        match self
            .state
            .admin_user_service
            .paginate(page, order, &self.state.db)
            .await
        {
            Ok(admin_user_paginate_data) => {
                let pagination = AdminUserPagination {
                    total: admin_user_paginate_data.0.total,
                };

                let paginate_data = AdminUserPaginateData {
                    pagination: Option::from(pagination),
                    data: admin_user_paginate_data.1,
                };

                let admin_user_paginate_response = AdminUserPaginateResponse {
                    status: true,
                    data: Option::from(paginate_data),
                };

                Ok(Response::new(admin_user_paginate_response))
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn store_admin_user(
        &self,
        request: Request<StoreAdminUserRequest>,
    ) -> Result<Response<StoreAdminUserResponse>, Status> {
        println!(
            "->> {:<12} - store_admin_user",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;

        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("store_admin_user"),
            )
            .await?;

        let request_data = request.into_inner();
        request_data.validate(&self.state).await?;


        match self
            .state
            .admin_user_service
            .store(
                request_data,
                claims.email,
                &self.state.config.password_salt,
                &self.state.db,
            )
            .await
        {
            Ok(reply) => {
                let response = Response::new(reply);

                Ok(response)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_admin_user(
        &self,
        request: Request<GetAdminUserRequest>,
    ) -> Result<Response<GetAdminUserResponse>, Status> {
        println!("->> {:<12} - get_admin_user", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;

        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("get_admin_user"),
            )
            .await?;

        let req = request.into_inner();
        match self
            .state
            .admin_user_service
            .find_admin_user_by_id(&req.admin_user_id, &self.state.db)
            .await
        {
            Ok(admin_user_model) => {
                let get_admin_user_response = GetAdminUserResponse {
                    status: true,
                    data: Some(admin_user_model),
                };
                let res = Response::new(get_admin_user_response);
                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_admin_user(
        &self,
        request: Request<UpdateAdminUserRequest>,
    ) -> Result<Response<UpdateAdminUserResponse>, Status> {
        println!(
            "->> {:<12} - update_admin_user",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;

        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("update_admin_user"),
            )
            .await?;

        let req = request.into_inner();
        req.validate().await?;

        match self
            .state
            .admin_user_service
            .update_admin_user(req, claims.email, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn role_paginate(
        &self,
        request: Request<RolePaginateRequest>,
    ) -> Result<Response<RolePaginateResponse>, Status> {
        println!("->> {:<12} - role_paginate", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("role_paginate"),
            )
            .await?;

        let req = request.into_inner();

        match self
            .state
            .admin_user_service
            .role_paginate(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn role_option(
        &self,
        request: Request<RoleOptionRequest>,
    ) -> Result<Response<RoleOptionResponse>, Status> {
        println!("->> {:<12} - role_paginate", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("role_option"),
            )
            .await?;

        match self
            .state
            .admin_user_service
            .role_option(&self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn store_role(
        &self,
        request: Request<StoreRoleRequest>,
    ) -> Result<Response<StoreRoleResponse>, Status> {
        println!("->> {:<12} - store_role", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("store_role"),
            )
            .await?;

        let req = request.into_inner();
        req.validate(&self.state).await?;

        let created_role_request = CreatableRole {
            name: req.name,
            identifier: req.identifier,
            logged_in_username: claims.email,
            permissions: req.permissions,
        };

        match self
            .state
            .admin_user_service
            .store_role(created_role_request, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn get_role(
        &self,
        request: Request<GetRoleRequest>,
    ) -> Result<Response<GetRoleResponse>, Status> {
        println!("->> {:<12} - get_role", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(&self.state.admin_user_service, String::from("get_role"))
            .await?;

        let req = request.into_inner();
        match self
            .state
            .admin_user_service
            .find_role_by_id(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);
                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>,
    ) -> Result<Response<UpdateRoleResponse>, Status> {
        println!("->> {:<12} - update_role", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("update_role"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .admin_user_service
            .update_role(req, claims.email, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn put_role_identifier(
        &self,
        request: Request<PutRoleIdentifierRequest>,
    ) -> Result<Response<PutRoleIdentifierResponse>, Status> {
        println!(
            "->> {:<12} - put_role_identifier",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("put_role_identifier"),
            )
            .await?;

        let req = request.into_inner();
        req.validate(&self.state).await?;

        match self
            .state
            .admin_user_service
            .put_role_identifier(req, claims.email, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn delete_role(
        &self,
        request: Request<DeleteRoleRequest>,
    ) -> Result<Response<DeleteRoleResponse>, Status> {
        println!("->> {:<12} - delete_role", "gRPC_Admin_User_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_role"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .admin_user_service
            .delete_role(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn delete_admin_user(
        &self,
        request: Request<DeleteAdminUserRequest>,
    ) -> Result<Response<DeleteAdminUserResponse>, Status> {
        println!(
            "->> {:<12} - delete_admin_user",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("delete_admin_user"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .admin_user_service
            .delete_admin_user(req, &self.state.db)
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn change_admin_user_password(
        &self,
        request: Request<ChangeAdminUserPasswordRequest>,
    ) -> Result<Response<ChangeAdminUserPasswordResponse>, Status> {
        info!(
            "->> {:<12} - change_admin_user_password",
            "gRPC_Admin_User_Api_Service"
        );

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("change_admin_user_password"),
            )
            .await?;

        let req = request.into_inner();
        req.validate()?;

        match self
            .state
            .admin_user_service
            .change_password(
                req,
                &claims.email,
                &self.state.config.password_salt,
                &self.state.db,
            )
            .await
        {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
