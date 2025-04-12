use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::grpc_admin_user::admin_user_server::AdminUser;
use crate::grpc_admin_user::{AdminUserPaginateRequest, AdminUserPaginateResponse, StoreAdminUserRequest, StoreAdminUserResponse};
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
}