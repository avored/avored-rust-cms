#[cfg(feature = "ssr")]
use tonic::{Request, Response, Status};

use crate::{
    application::use_cases::misc_use_case::MiscUseCase,
    infra::grpc::misc::{
        misc_server::Misc, HealthCheckRequest, HealthCheckResponse, SetupRequest, SetupResponse,
    },
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

        let request = HealthCheckResponse { status: true };

        Ok(Response::new(request))
    }

    async fn setup(
        &self,
        request: Request<SetupRequest>,
    ) -> Result<Response<SetupResponse>, Status> {
        println!("Got a setup request: {:?}", request);

        // Think of implementing a generic result handler
        let setup_result = match  self.misc_use_case.setup().await {
            Ok(result) => result,
            Err(e) => return Err(Status::internal(e.to_string())),
        };
        let request = SetupResponse { status: setup_result };

        Ok(Response::new(request))
    }
}
