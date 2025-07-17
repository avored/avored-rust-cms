use crate::api::proto::general::general_service_server::GeneralService;
use crate::api::proto::general::{LoggedInUserRequest, LoggedInUserResponse};
use crate::avored_state::AvoRedState;
use crate::models::token_claim_model::TokenClaims;
use std::sync::Arc;
use tonic::{async_trait, Response, Status};

pub struct GeneralApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl GeneralService for GeneralApi {
    async fn logged_in_user(
        &self,
        request: tonic::Request<LoggedInUserRequest>,
    ) -> Result<Response<LoggedInUserResponse>, Status> {
        println!("->> {:<12} - logged_in_user", "gRPC_General_Service");
        let claims = request.extensions().get::<TokenClaims>().cloned().unwrap();

        match self.state.general_service.logged_in_user(claims).await {
            Ok(reply) => Ok(Response::new(reply)),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
