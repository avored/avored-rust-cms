use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::grpc_admin_user::admin_user_paginate_response::{AdminUserModel, AdminUserPaginateData, AdminUserPagination};
use crate::grpc_admin_user::admin_user_server::AdminUser;
use crate::grpc_admin_user::{AdminUserPaginateRequest, AdminUserPaginateResponse};

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
}