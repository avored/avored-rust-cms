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
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
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
        // let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
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
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
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
        _request: Request<RoleOptionRequest>
    ) -> Result<Response<RoleOptionResponse>, Status> {

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
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();

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

    async fn update_role (
        &self,
        request: Request<UpdateRoleRequest>
    ) -> Result<Response<UpdateRoleResponse>, Status>
    {
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let req = request.into_inner();
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
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
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