use crate::api::proto::auth::auth_server::Auth;
use crate::api::proto::auth::{
    ForgotPasswordRequest, ForgotPasswordResponse, LoginRequest, LoginResponse,
    ResetPasswordRequest, ResetPasswordResponse,
};
use crate::avored_state::AvoRedState;
use crate::error::Error::Tonic;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tonic::{async_trait, Request, Response, Status};


/// `AuthApi` is the gRPC API for managing admin user auth.
pub struct AuthApi {
    /// The shared state of the application, containing services and configurations.
    pub state: Arc<AvoRedState>,
}

#[async_trait]
impl Auth for AuthApi {
    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        println!("->> {:<12} - login", "GRPC_Auth_API_SERVICE");

        let remote_address_sock = request.remote_addr().unwrap_or(SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 80));
        let remote_address = remote_address_sock.to_string();
        let req = request.into_inner();
        let (valid, error_messages) = req.validate()?;
        if !valid {
            return Err(Status::invalid_argument(error_messages));
        }

        match self
            .state
            .auth_service
            .auth_user(
                &req.email,
                &req.password,
                &self.state.db,
                &self.state.config.jwt_secret_key,
                remote_address,
                &self.state.config,
            )
            .await
        {
            Ok(token) => {
                let login_response = LoginResponse {
                    status: true,
                    data: token,
                };
                Ok(Response::new(login_response))
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn forgot_password(
        &self,
        request: Request<ForgotPasswordRequest>,
    ) -> Result<Response<ForgotPasswordResponse>, Status> {
        println!("->> {:<12} - forgot_password", "GRPC_Auth_API_SERVICE");

        let req = request.into_inner();
        let (valid, error_messages) = req.validate(&self.state).await?;
        if !valid {
            return Err(Status::invalid_argument(error_messages));
        }

        match self
            .state
            .auth_service
            .forgot_password(
                &self.state.db,
                &self.state.template,
                &self.state.config.react_admin_app_url,
                &req.email,
            )
            .await
        {
            Ok(sent_status) => {
                let forgot_password_response = ForgotPasswordResponse {
                    status: sent_status,
                };
                Ok(Response::new(forgot_password_response))
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }

    async fn reset_password(
        &self,
        request: Request<ResetPasswordRequest>,
    ) -> Result<Response<ResetPasswordResponse>, Status> {
        println!("->> {:<12} - reset_password", "GRPC_Auth_API_SERVICE");

        let req = request.into_inner();

        let (valid, error_messages) = req.validate(&self.state).await?;

        if !valid {
            return Err(Status::invalid_argument(error_messages));
        }

        match self
            .state
            .auth_service
            .reset_password(
                &self.state.db,
                &req.email,
                req.password,
                &self.state.config.password_salt,
                &req.token,
            )
            .await
        {
            Ok(reset_password_status) => {
                let reset_password_response = ResetPasswordResponse {
                    status: reset_password_status,
                };
                let res = Response::new(reset_password_response);

                Ok(res)
            }
            Err(e) => match e {
                Tonic(status) => Err(*status),
                _ => Err(Status::internal(e.to_string())),
            },
        }
    }
}
