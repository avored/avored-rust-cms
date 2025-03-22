use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;
use crate::grpc_auth::{LoginRequest, LoginResponse};
use crate::grpc_auth::auth_server::Auth;
use crate::models::token_claim_model::TokenClaims;

pub struct AuthApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Auth for AuthApi {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {

        let req = request.into_inner();


        let (valid, error_messages) = req.validate()?;

        if !valid {
            return Err(Status::invalid_argument(error_messages))
        }

        match self.
            state.
            auth_service.
            auth_user(
                req,
                &self.state.db,
                &self.state.config.jwt_secret_key
            ).await {
                Ok(reply) => Ok(Response::new(reply)),
                Err(e) => match e {
                    TonicError(status) => Err(status),
                    _ => Err(Status::internal(e.to_string()))
                }
            }

    }
}