use crate::application::use_cases::login_user_use_case::LoginUserUseCase;
use crate::infra::grpc::auth_user::{auth_server::Auth, LoginRequest, LoginResponse};
use tonic::{Request, Response, Status};

pub struct AuthUserGrpcApi {
    pub login_user_use_case: LoginUserUseCase,
}

impl AuthUserGrpcApi {
    pub fn new(login_user_use_case: LoginUserUseCase) -> Self {
        Self {
            login_user_use_case,
        }
    }
}

#[tonic::async_trait]
impl Auth for AuthUserGrpcApi {
    async fn login_user(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();

        let token = self
            .login_user_use_case
            .execute(&req.email, &req.password)
            .await?;

        let response = LoginResponse {
            status: true,
            data: token,
        };

        Ok(Response::new(response))
    }
}
