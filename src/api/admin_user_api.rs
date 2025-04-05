use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
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
        let _req = request.into_inner();

        // let page_data = self
        //     .state
        //     .
        
        let a_pag = AdminUserPagination {
            total: 123
        };
        let data = vec![AdminUserModel {
            id: String::from("test from backend change")
        }];

        let test = AdminUserPaginateData {
            pagination: Option::from(a_pag),
            data
        };

        let reply = AdminUserPaginateResponse {
            status: true,
            data: Option::from(test)
        };

        println!("Admin user paginate: {:?}", &reply);

        Ok(Response::new(reply))
    }
}