#[cfg(feature = "ssr")]
use tonic::{Request, Response, Status};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use helloworld::greeter_server::Greeter;
use helloworld::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello Test PP {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}
