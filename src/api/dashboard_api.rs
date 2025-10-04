use crate::api::proto::dashboard::dashboard_server::Dashboard;
use crate::api::proto::dashboard::{DashboardRequest, DashboardResponse, VisitByContentTypeRequest, VisitByContentTypeResponse, VisitByYearRequest, VisitByYearResponse};
use crate::avored_state::AvoRedState;
use crate::extensions::tonic_request::TonicRequest;
use crate::models::admin_user_model::AdminUserModelExtension;
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};

/// `AvoRed` Dashboard API
pub struct DashboardApi {
    /// The `AvoRed` state containing services and configurations
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Dashboard for DashboardApi {
    async fn dashboard(
        &self,
        request: Request<DashboardRequest>,
    ) -> Result<Response<DashboardResponse>, Status> {
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
    
    async fn get_visit_by_year(
        &self,
        request: Request<VisitByYearRequest>,
    ) -> Result<Response<VisitByYearResponse>, Status> {
        println!("->> {:<12} - get_visit_by_year", "gRPC_Dashboard_Api_Service");

        let claims = request.get_token_claim()?;
        let req = request.into_inner();
        
        let _logged_in_user = claims.admin_user_model;
        let visit_result =self
            .state
            .general_service
            .get_visit_by_year(
                &self.state.db,
                req.year,
            )
            .await?;

        let reply = VisitByYearResponse { status: true, data: visit_result };

        Ok(Response::new(reply))
    }

    async fn get_visit_by_content_type(
        &self,
        request: Request<VisitByContentTypeRequest>,
    ) -> Result<Response<VisitByContentTypeResponse>, Status> {
        println!("->> {:<12} - get_visit_by_year", "gRPC_Dashboard_Api_Service");

        let claims = request.get_token_claim()?;
        let req = request.into_inner();
        
        let _logged_in_user = claims.admin_user_model;
        let visit_result =self
            .state
            .general_service
            .get_visit_by_content_type(
                &self.state.db,
                req.content_type,
                req.year,
            )
            .await?;

        let reply = VisitByContentTypeResponse { status: true, data: visit_result };


        Ok(Response::new(reply))
    }
}
