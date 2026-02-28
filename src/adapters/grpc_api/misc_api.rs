#[cfg(feature = "ssr")]
use tonic::{Request, Response, Status};

use crate::{
    application::use_cases::misc_use_case::MiscUseCase, domain::models::admin_user::StorableAdminUser, infra::grpc::misc::{
        HealthCheckRequest, HealthCheckResponse, SetupRequest, SetupResponse, misc_server::Misc
    }
};


#[derive(Clone)]
pub struct MiscGrpcApi {
    pub misc_use_case: MiscUseCase,
}

impl MiscGrpcApi {
    pub fn new(misc_use_case: MiscUseCase) -> Self {
        Self {
            misc_use_case,
        }
    }
}



#[tonic::async_trait]
impl Misc for MiscGrpcApi {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = HealthCheckResponse { status: true };

        Ok(Response::new(response))
    }

    async fn setup(
        &self,
        request: Request<SetupRequest>,
    ) -> Result<Response<SetupResponse>, Status> {
        let request = request.into_inner();

        // validate the request and convert it to a storable admin user

        let mut storable_admin_user: StorableAdminUser = match request.try_into() {
            Ok(req) => req,
            Err(e) => return Err(Status::invalid_argument(e.to_string())),
        };

        storable_admin_user.logged_in_user = "ApplicationSetupProcess".to_string();
        storable_admin_user.is_super_admin = true;

        // @todo hash the password
        
        println!("Got a setup request: {:?}", storable_admin_user);

        // Think of implementing a generic result handler
        let setup_result = match  self.misc_use_case.setup(storable_admin_user).await {
            Ok(result) => result,
            Err(e) => return Err(Status::internal(e.to_string())),
        };
        let request = SetupResponse { status: setup_result };

        Ok(Response::new(request))
    }
}
