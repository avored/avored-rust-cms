use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::dashboard::dashboard_server::Dashboard;
use crate::api::proto::dashboard::{DashboardRequest, DashboardResponse};
use crate::models::token_claim_model::TokenClaims;

pub struct DashboardApi {
    // pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Dashboard for DashboardApi {
    async fn dashboard(&self, request: Request<DashboardRequest>) -> Result<Response<DashboardResponse>, Status> {
        let _claims = request.extensions().get::<TokenClaims>().cloned().unwrap();

        println!("->> {:<12} - dashboard", "gRPC_Dashboard_Api_Service");

        let reply = DashboardResponse { status: true };
        Ok(Response::new(reply))
    }
}