use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::avored_state::AvoRedState;
use crate::grpc_auth::{LoginRequest, LoginResponse};
use crate::grpc_auth::auth_server::Auth;
use crate::grpc_misc::misc_server::Misc;


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


        let reply = LoginResponse {
            status: true
        };

        Ok(Response::new(reply))
        // match self.
        //     state.
        //     misc_service.
        //     setup(
        //         req,
        //         &self.state.config.password_salt,
        //         &self.state.db
        //     ).await {
        //         Ok(reply) => Ok(Response::new(reply)),
        //         Err(e) => Err(Status::internal(e.to_string()))
        //     }

    }
}