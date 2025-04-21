use std::env;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use axum_tonic::{NestTonic, RestGrpcService};
use tonic::{async_trait, Status};
use crate::test2_server::{Test2, Test2Server};

tonic::include_proto!("echo");

pub struct MyEcho;

#[async_trait]
impl Test2 for MyEcho {
    async fn test2(&self, _request: tonic::Request<Test2Request>) -> Result<tonic::Response<Test2Reply>, Status> {
        Ok(tonic::Response::new(Test2Reply {
            message: "Hello, back!".to_string(),
        }))
    }
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {

    let auth_api = MyEcho {};
    let auth_server = Test2Server::new(auth_api);

    let grpc_router = Router::new()
        .nest_tonic(auth_server);


    let rest_router = Router::new()
        .route("/", get(handler));


    let service = RestGrpcService::new(rest_router, grpc_router);

    let port = env::var("PORT").unwrap_or("50051".to_string());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");

    println!();
    println!();

    println!("Server started: http://0.0.0.0:{}", port);

    axum::serve(listener ,service.into_make_service())
        .await
        .unwrap();

    // let axum_make_service = axum::Router::new()
    //     .route("/", axum::routing::get(|| async { "Hello PP!" }))
    //     .into_make_service();
    //
    // let grpc_service = tonic::transport::Server::builder()
    //     .add_service(EchoServer::new(MyEcho))
    //     .into_service();
    //
    //
    // let hybrid_make_service = hybrid(axum_make_service, grpc_service);
    //
    //
    //
    // let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    // println!("listening on {}", addr);
    // let listener = TcpListener::bind(addr).await.unwrap();
    //
    // loop {
    //     let (stream, _) = listener.accept().await.unwrap();
    //     let io = TokioIo::new(stream);
    //     let svc_clone = hybrid_make_service.clone();
    //     tokio::task::spawn(async move {
    //         if let Err(err) = http1::Builder::new().serve_connection(io, svc_clone).await {
    //             println!("Failed to serve connection: {:?}", err);
    //         }
    //     });
    // }
}




// fn hybrid<MakeWeb, Grpc>(make_web: MakeWeb, grpc: Grpc) -> HybridMakeService<MakeWeb, Grpc> {
//     HybridMakeService { make_web, grpc }
// }
//
// struct HybridMakeService<MakeWeb, Grpc> {
//     make_web: MakeWeb,
//     grpc: Grpc,
// }
//
// impl<ConnInfo, MakeWeb, Grpc> Service<ConnInfo> for HybridMakeService<MakeWeb, Grpc>
// where
//     MakeWeb: Service<ConnInfo>,
//     Grpc: Clone,
// {
//     type Response = HybridService<MakeWeb::Response, Grpc>;
//     type Error = MakeWeb::Error;
//     type Future = HybridMakeServiceFuture<MakeWeb::Future, Grpc>;
//
//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         self.make_web.poll_ready(cx)
//     }
//
//     fn call(&mut self, conn_info: ConnInfo) -> Self::Future {
//         HybridMakeServiceFuture {
//             web_future: self.make_web.call(conn_info),
//             grpc: Some(self.grpc.clone()),
//         }
//     }
// }
//
// #[pin_project]
// struct HybridMakeServiceFuture<WebFuture, Grpc> {
//     #[pin]
//     web_future: WebFuture,
//     grpc: Option<Grpc>,
// }
//
// impl<WebFuture, Web, WebError, Grpc> Future for HybridMakeServiceFuture<WebFuture, Grpc>
// where
//     WebFuture: Future<Output = Result<Web, WebError>>,
// {
//     type Output = Result<HybridService<Web, Grpc>, WebError>;
//
//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
//         let this = self.project();
//         match this.web_future.poll(cx) {
//             Poll::Pending => Poll::Pending,
//             Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
//             Poll::Ready(Ok(web)) => Poll::Ready(Ok(HybridService {
//                 web,
//                 grpc: this.grpc.take().expect("Cannot poll twice!"),
//             })),
//         }
//     }
// }
//
// struct HybridService<Web, Grpc> {
//     web: Web,
//     grpc: Grpc,
// }
//
// impl<Web, Grpc, WebBody, GrpcBody> Service<Request<Incoming>> for HybridService<Web, Grpc>
// where
//     Web: Service<Request<Incoming>, Response = Response<WebBody>>,
//     Grpc: Service<Request<Incoming>, Response = Response<GrpcBody>>,
//     Web::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
//     Grpc::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
// {
//     type Response = Response<HybridBody<WebBody, GrpcBody>>;
//     type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//     type Future = HybridFuture<Web::Future, Grpc::Future>;
//
//     fn poll_ready(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Result<(), Self::Error>> {
//         match self.web.poll_ready(cx) {
//             Poll::Ready(Ok(())) => match self.grpc.poll_ready(cx) {
//                 Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
//                 Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
//                 Poll::Pending => Poll::Pending,
//             },
//             Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
//             Poll::Pending => Poll::Pending,
//         }
//     }
//
//     fn call(&mut self, req: Request<Incoming>) -> Self::Future {
//         if req.headers().get("content-type").map(|x| x.as_bytes()) == Some(b"application/grpc") {
//             HybridFuture::Grpc(self.grpc.call(req))
//         } else {
//             HybridFuture::Web(self.web.call(req))
//         }
//     }
// }
//
// #[pin_project(project = HybridBodyProj)]
// enum HybridBody<WebBody, GrpcBody> {
//     Web(#[pin] WebBody),
//     Grpc(#[pin] GrpcBody),
// }
//
// impl<WebBody, GrpcBody> HttpBody for HybridBody<WebBody, GrpcBody>
// where
//     WebBody: HttpBody + Send + Unpin,
//     GrpcBody: HttpBody<Data = WebBody::Data> + Send + Unpin,
//     WebBody::Error: std::error::Error + Send + Sync + 'static,
//     GrpcBody::Error: std::error::Error + Send + Sync + 'static,
// {
//     type Data = WebBody::Data;
//     type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
//
//     fn is_end_stream(&self) -> bool {
//         match self {
//             HybridBody::Web(b) => b.is_end_stream(),
//             HybridBody::Grpc(b) => b.is_end_stream(),
//         }
//     }
//
//     // fn poll_data(
//     //     self: Pin<&mut Self>,
//     //     cx: &mut std::task::Context,
//     // ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
//     //     match self.project() {
//     //         HybridBodyProj::Web(b) => b.poll_data(cx).map_err(|e| e.into()),
//     //         HybridBodyProj::Grpc(b) => b.poll_data(cx).map_err(|e| e.into()),
//     //     }
//     // }
//
//     // fn poll_trailers(
//     //     self: Pin<&mut Self>,
//     //     cx: &mut std::task::Context,
//     // ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
//     //     match self.project() {
//     //         HybridBodyProj::Web(b) => b.poll_trailers(cx).map_err(|e| e.into()),
//     //         HybridBodyProj::Grpc(b) => b.poll_trailers(cx).map_err(|e| e.into()),
//     //     }
//     // }
//
//     fn poll_frame(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
//         match self.project() {
//             HybridBodyProj::Web(b) => b.poll_frame(cx).map_err(|e| e.into()),
//             HybridBodyProj::Grpc(b) => b.poll_frame(cx).map_err(|e| e.into()),
//         }
//     }
// }
//
// #[pin_project(project = HybridFutureProj)]
// enum HybridFuture<WebFuture, GrpcFuture> {
//     Web(#[pin] WebFuture),
//     Grpc(#[pin] GrpcFuture),
// }
//
// impl<WebFuture, GrpcFuture, WebBody, GrpcBody, WebError, GrpcError> Future
// for HybridFuture<WebFuture, GrpcFuture>
// where
//     WebFuture: Future<Output = Result<Response<WebBody>, WebError>>,
//     GrpcFuture: Future<Output = Result<Response<GrpcBody>, GrpcError>>,
//     WebError: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
//     GrpcError: Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
// {
//     type Output = Result<
//         Response<HybridBody<WebBody, GrpcBody>>,
//         Box<dyn std::error::Error + Send + Sync + 'static>,
//     >;
//
//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Self::Output> {
//         match self.project() {
//             HybridFutureProj::Web(a) => match a.poll(cx) {
//                 Poll::Ready(Ok(res)) => Poll::Ready(Ok(res.map(HybridBody::Web))),
//                 Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
//                 Poll::Pending => Poll::Pending,
//             },
//             HybridFutureProj::Grpc(b) => match b.poll(cx) {
//                 Poll::Ready(Ok(res)) => Poll::Ready(Ok(res.map(HybridBody::Grpc))),
//                 Poll::Ready(Err(e)) => Poll::Ready(Err(e.into())),
//                 Poll::Pending => Poll::Pending,
//             },
//         }
//     }
// }
//
//
