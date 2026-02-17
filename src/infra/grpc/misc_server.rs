#[cfg(feature = "ssr")]
use tonic::{Request, Response, Status};

use crate::infra::grpc::misc::{HealthCheckRequest, HealthCheckResponse, SetupRequest, SetupResponse, misc_server::Misc};




#[derive(Debug, Default)]
pub struct MyMisc {}

#[tonic::async_trait]
impl Misc for MyMisc {
    async fn health_check(
        &self,
        request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        println!("Got a request: {:?}", request);

        let request = HealthCheckResponse {
            status: true,
        };

        Ok(Response::new(request))
    }

    async fn setup (
        &self,
        request: Request<SetupRequest>,
    ) -> Result<Response<SetupResponse>, Status> {
        println!("Got a setup request: {:?}", request);

        let request = SetupResponse {
            status: true,
        };

        Ok(Response::new(request))
    }
}
