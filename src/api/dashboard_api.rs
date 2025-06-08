use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::dashboard::dashboard_server::Dashboard;
use crate::api::proto::dashboard::{DashboardRequest, DashboardResponse};
use crate::avored_state::AvoRedState;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;

pub struct DashboardApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Dashboard for DashboardApi {
    async fn dashboard(&self, request: Request<DashboardRequest>) -> Result<Response<DashboardResponse>, Status> {
        
        println!("->> {:<12} - dashboard", "gRPC_Dashboard_Api_Service");

        let claims = request.get_token_claim()?;
        let logged_in_user = claims.admin_user_model;
        logged_in_user
            .check_user_has_resouce_access(
                &self.state.admin_user_service,
                String::from("dashboard"),
            )
            .await?;


        let reply = DashboardResponse { status: true };
        Ok(Response::new(reply))
    }
}