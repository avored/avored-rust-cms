use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::grpc_admin_user::{AdminUserPaginateRequest, AdminUserPaginateResponse};
use crate::grpc_admin_user::admin_user_server::AdminUser;

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

        let reply = AdminUserPaginateResponse {
            status: true,
            data: String::from("test value")
        };

        println!("Admin user paginate: {:?}", &reply);

        Ok(Response::new(reply))
    }
}