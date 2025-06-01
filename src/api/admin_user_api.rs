use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::admin_user::admin_user_server::AdminUser;
use crate::api::proto::admin_user::{AdminUserPaginateRequest, AdminUserPaginateResponse, GetAdminUserRequest, GetAdminUserResponse, GetRoleRequest, GetRoleResponse, PutRoleIdentifierRequest, PutRoleIdentifierResponse, RoleOptionRequest, RoleOptionResponse, RolePaginateRequest, RolePaginateResponse, StoreAdminUserRequest, StoreAdminUserResponse, StoreRoleRequest, StoreRoleResponse, UpdateAdminUserRequest, UpdateAdminUserResponse, UpdateRoleRequest, UpdateRoleResponse};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::models::role_model::CreatableRole;
use crate::models::token_claim_model::TokenClaims;

pub struct AdminUserApi {
    pub state: Arc<AvoRedState>,
}
#[async_trait]
impl AdminUser for AdminUserApi {
    async fn paginate(
        &self,
        request: Request<AdminUserPaginateRequest>
    ) -> Result<Response<AdminUserPaginateResponse>, Status>
    {
        println!("->> {:<12} - paginate_admin_user", "gRPC_Admin_User_Api_Service");
        let claims = match request.extensions().get::<TokenClaims>() {
            Some(claims) => claims.clone(),
            None => {
                return Err(Status::unauthenticated("Unauthenticated"));
            }       
        };
        let logged_in_user = claims.admin_user_model;
        
        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("admin_user_table"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
        
       
        match self.
            state.
            admin_user_service.
            paginate(
                req,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn store_admin_user (
        &self,
        request: Request<StoreAdminUserRequest>
    ) -> Result<Response<StoreAdminUserResponse>, Status>
    {
        println!("->> {:<12} - store_admin_user", "gRPC_Admin_User_Api_Service");
        
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("store_admin_user"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let req = request.into_inner();
        let (valid, error_messages) = req.validate(&self.state).await?;

        if !valid {
            return Err(Status::invalid_argument(error_messages))
        }
        
        match self.
            state.
            admin_user_service.
            store(
                req,
                claims.email,
                &self.state.config.password_salt,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn get_admin_user (
        &self,
        request: Request<GetAdminUserRequest>
    ) -> Result<Response<GetAdminUserResponse>, Status>
    {
        println!("->> {:<12} - get_admin_user", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("get_admin_user"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();
        match self.
            state.
            admin_user_service.
            find_admin_user_by_id(
                req,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);
                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn update_admin_user (
        &self,
        request: Request<UpdateAdminUserRequest>
    ) -> Result<Response<UpdateAdminUserResponse>, Status>
    {
        println!("->> {:<12} - update_admin_user", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("update_admin_user"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();
        match self.
            state.
            admin_user_service.
            update_admin_user(
                req,
                claims.email,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn role_paginate(
        &self,
        request: Request<RolePaginateRequest>
    ) -> Result<Response<RolePaginateResponse>, Status>
    {
        println!("->> {:<12} - role_paginate", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("role_paginate"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();

        match self.
            state.
            admin_user_service.
            role_paginate(
                req,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn role_option(
        &self,
        request: Request<RoleOptionRequest>
    ) -> Result<Response<RoleOptionResponse>, Status> {

        println!("->> {:<12} - role_paginate", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("role_paginate"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        
        match self.
            state.
            admin_user_service.
            role_option(
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn store_role (
        &self,
        request: Request<StoreRoleRequest>
    ) -> Result<Response<StoreRoleResponse>, Status>
    {
        println!("->> {:<12} - store_role", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("store_role"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        
        let req = request.into_inner();
        
        let (valid, error_messages) = req.validate(&self.state).await?;

        if !valid {
            return Err(Status::invalid_argument(error_messages))
        }

        let created_role_request = CreatableRole {
            name: req.name,
            identifier: req.identifier,
            logged_in_username: claims.email,
            permissions: req.permissions,
        };
        
        match self.
            state.
            admin_user_service.
            store_role(
                created_role_request,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn get_role (
        &self,
        request: Request<GetRoleRequest>
    ) -> Result<Response<GetRoleResponse>, Status>
    {
        println!("->> {:<12} - get_role", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("get_role"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();
        match self.
            state.
            admin_user_service.
            find_role_by_id(
                req,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);
                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn update_role(
        &self,
        request: Request<UpdateRoleRequest>
    ) -> Result<Response<UpdateRoleResponse>, Status>
    {
        println!("->> {:<12} - update_role", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("update_role"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();
        let (valid, error_messages) = req.validate()?;

        if !valid {
            return Err(Status::invalid_argument(error_messages))
        }
        
        match self.
            state.
            admin_user_service.
            update_role(
                req,
                claims.email,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

    async fn put_role_identifier (
        &self,
        request: Request<PutRoleIdentifierRequest>
    ) -> Result<Response<PutRoleIdentifierResponse>, Status>
    {

        println!("->> {:<12} - put_role_identifier", "gRPC_Admin_User_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self.state
            .admin_user_service
            .has_permission(logged_in_user, String::from("put_role_identifier"))
            .await?;
        if !has_permission_bool {
            let status = Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }
        
        let req = request.into_inner();
        match self.
            state.
            admin_user_service.
            put_role_identifier(
                req,
                claims.email,
                &self.state.db
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }

}
