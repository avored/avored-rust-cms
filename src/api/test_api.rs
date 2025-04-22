use tonic::{async_trait, Status};
use crate::api::proto::echo::test2_server::Test2;
use crate::api::proto::echo::{Test2Reply, Test2Request};

pub struct Test2Api;

#[async_trait]
impl Test2 for Test2Api {
    async fn test2(&self, _request: tonic::Request<Test2Request>) -> Result<tonic::Response<Test2Reply>, Status> {
        Ok(tonic::Response::new(Test2Reply {
            message: "Hello, back!".to_string(),
        }))
    }
}
