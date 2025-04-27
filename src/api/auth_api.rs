use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};
use crate::api::proto::auth::{ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse};
use crate::api::proto::auth::auth_server::Auth;
use crate::avored_state::AvoRedState;
use crate::error::Error::TonicError;


pub struct AuthApi {
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Auth for AuthApi {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {

        println!("request: {:?}", request);
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
                Ok(reply) => {
                    let res = Response::new(reply);

                    // let meta_data = res.metadata();
                    // meta_data.get_all()
                    // res.metadata.into_headers()

                    Ok(res)
                },
                Err(e) => match e {
                    TonicError(status) => Err(status),
                    _ => Err(Status::internal(e.to_string()))
                }
            }

    }
    
    async fn forgot_password(
        &self, 
        request: Request<ForgotPasswordRequest>
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        let req = request.into_inner();
        
        
        match self.
            state.
            auth_service.
            forgot_password(
                &self.state.db,
                &self.state.template,
                &self.state.config.react_admin_app_url,
                &req.email,
            ).await {
            Ok(reply) => {
                let res = Response::new(reply);

                // let meta_data = res.metadata();
                // meta_data.get_all()
                // res.metadata.into_headers()

                Ok(res)
            },
            Err(e) => match e {
                TonicError(status) => Err(status),
                _ => Err(Status::internal(e.to_string()))
            }
        }
    }
}