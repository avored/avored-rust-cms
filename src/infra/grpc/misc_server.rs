#[cfg(feature = "ssr")]
use tonic::{Request, Response, Status};

use crate::infra::grpc::misc::{HealthCheckRequest, HealthCheckResponse, misc_server::Misc};




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
}
