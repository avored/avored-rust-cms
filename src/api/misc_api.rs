use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::grpc_misc::misc_server::Misc;
use crate::grpc_misc::{SetupRequest, SetupResponse, HealthCheckResponse, HealthCheckRequest};

pub struct MiscApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Misc for MiscApi {
    async fn setup(&self, request: Request<SetupRequest>) -> Result<Response<SetupResponse>, Status> {

        let req = request.into_inner();
        let (valid, error_messages) = req.validate()?;

        if !valid {
            return Err(Status::invalid_argument(error_messages))
        }

        match self.
            state.
            misc_service.
            setup(
                req,
                &self.state.config.password_salt,
                &self.state.db
            ).await {
                Ok(reply) => Ok(Response::new(reply)),
                Err(e) => Err(Status::internal(e.to_string()))
            }

    }

    async fn health_check(&self, _request: Request<HealthCheckRequest>) -> Result<Response<HealthCheckResponse>, Status> {
        let reply = HealthCheckResponse {
            status: true
        };

        Ok(Response::new(reply))
    }
}