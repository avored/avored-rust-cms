use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::dashboard::dashboard_server::Dashboard;
use crate::api::proto::dashboard::{DashboardRequest, DashboardResponse};
use crate::avored_state::AvoRedState;
use crate::models::token_claim_model::TokenClaims;

pub struct DashboardApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Dashboard for DashboardApi {
    async fn dashboard(&self, request: Request<DashboardRequest>) -> Result<Response<DashboardResponse>, Status> {
        
        println!("->> {:<12} - dashboard", "gRPC_Dashboard_Api_Service");

        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();
        let logged_in_user = claims.admin_user_model;

        let has_permission_bool = self
            .state
            .admin_user_service
            .has_permission(logged_in_user, String::from("dashboard"))
            .await?;
        if !has_permission_bool {
            let status =
                Status::permission_denied("You don't have permission to access this resource");
            return Err(status);
        }

        let reply = DashboardResponse { status: true };
        Ok(Response::new(reply))
    }
}