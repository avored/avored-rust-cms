use crate::application::use_cases::login_user_use_case::LoginUserUseCase;
use crate::domain::models::validation_error::{ErrorMessage, ErrorResponse, Validate};
use crate::infra::grpc::auth_user::{auth_server::Auth, LoginRequest, LoginResponse};
use rust_i18n::t;
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

        req.validate().await?;

        let login_response_data = self
            .login_user_use_case
            .execute(&req.email, &req.password)
            .await?;

        let response = LoginResponse {
            status: true,
            data: Some(login_response_data),
        };

        Ok(Response::new(response))
    }
}




impl LoginRequest {
    /// validate
    pub async fn validate(&self) -> crate::error::Result<()> {
        let mut errors: Vec<ErrorMessage> = vec![];
        let mut valid = true;
        let locale = "en";

        if !self.email.required()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("validation_required", locale = locale, attribute = t!("email", locale = locale)).to_string(),
            };
            valid = false;
            errors.push(error_message);
        }

        if !self.email.validate_email()? {
            let error_message = ErrorMessage {
                key: String::from("email"),
                message: t!("email_address_not_valid", locale = locale).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }

    

        // if profile photo exist then certain type of photo is only allowed
        if !self.password.required()? {
            let error_message = ErrorMessage {
                key: String::from("password"),
                message: t!("validation_required", locale = locale, attribute = t!("password", locale = locale)).to_string(),
            };

            valid = false;
            errors.push(error_message);
        }


        if !valid {
            let error_response = ErrorResponse {
                status: valid,
                errors,
            };
            let error_string = serde_json::to_string(&error_response)?;
            return Err(crate::error::Error::InvalidArgument(error_string));
        }

        Ok(())
    }
}
