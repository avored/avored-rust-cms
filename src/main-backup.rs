use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tonic::codegen::http::HeaderValue;
use tonic::transport::Server;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};
// use crate::api::admin_user_api::AdminUserApi;
// use crate::api::auth_api::{AuthApi};
// use crate::api::dashboard_api::DashboardApi;
use crate::api::misc_api::MiscApi;
use crate::avored_state::AvoRedState;
use crate::error::Error;
// use crate::grpc_admin_user::admin_user_server::AdminUserServer;
// use crate::grpc_auth::auth_server::AuthServer;
// use crate::grpc_dashboard::dashboard_server::DashboardServer;
use crate::grpc_misc::misc_server::MiscServer;
// use crate::middleware::grpc_auth_middleware::check_auth;

mod avored_state;
mod providers;
// mod requests;
mod models;
// mod middleware;
mod repositories;
mod api;
mod error;
mod services;

const PER_PAGE: u64 = 100;

pub mod grpc_misc {
    tonic::include_proto!("misc");
}

// pub mod grpc_auth {
//     tonic::include_proto!("auth");
// }
//
// pub mod grpc_dashboard {
//     tonic::include_proto!("dashboard");
// }
//
// pub mod grpc_admin_user {
//     tonic::include_proto!("admin_user");
// }


rust_i18n::i18n!("resources/locales");


#[tokio::main]
async fn main() -> Result<(), Error> {
    init_log();

    let port = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("127.0.0.1:{}", port).parse().map_err(|_e| Error::Generic(String::from("address parse error")))?;
    let state = Arc::new(AvoRedState::new().await?);

    let mut origins: Vec<HeaderValue> = vec![];
    for origin in &state.config.cors_allowed_app_url {
        origins.push(HeaderValue::from_str(origin).unwrap());
    }

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(Any)
        .allow_methods(Any);


    let axum_make_service = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello world!" }))
        .route("/test", axum::routing::get(|| async { "Hello test!" }))
        .into_make_service();


    // region: Grpc Service region
    let misc_api = MiscApi {state: state.clone()};
    let misc_server = MiscServer::new(misc_api);

    // let auth_api = AuthApi {state: state.clone()};
    // let auth_server = AuthServer::new(auth_api);
    //
    //
    // let dashboard_api = DashboardApi {};
    // let dashboard_server = DashboardServer::with_interceptor(dashboard_api, check_auth);
    //
    // let admin_user_api = AdminUserApi {state: state.clone()};
    // let admin_user_server = AdminUserServer::with_interceptor(admin_user_api, check_auth);
    // endregion: Grpc Service region

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");

    println!();
    println!();

    println!("Server started: http://0.0.0.0:{}", port);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(misc_server)
        // .add_service(auth_server)
        // .add_service(dashboard_server)
        // .add_service(admin_user_server)
        .serve(addr)
        .await?;

    Ok(())
}



fn init_log() {
    let stdout_log = tracing_subscriber::fmt::layer().pretty();

    // A layer that logs events to a file.
    let file = File::create(Path::new("public").join("log").join("avored.log"));
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let debug_log = tracing_subscriber::fmt::layer().with_writer(Arc::new(file));

    // A layer that collects metrics using specific events.
    let metrics_layer = /* ... */ filter::LevelFilter::INFO;

    tracing_subscriber::registry()
        .with(
            stdout_log
                // Add an `INFO` filter to the stdout logging layer
                .with_filter(filter::LevelFilter::INFO)
                // Combine the filtered `stdout_log` layer with the
                // `debug_log` layer, producing a new `Layered` layer.
                .and_then(debug_log)
                // Add a filter to *both* layers that rejects spans and
                // events whose targets start with `metrics`.
                .with_filter(filter::filter_fn(|metadata| {
                    !metadata.target().starts_with("metrics")
                })),
        )
        .with(
            // Add a filter to the metrics label that *only* enables
            // events whose targets start with `metrics`.
            metrics_layer.with_filter(filter::filter_fn(|metadata| {
                metadata.target().starts_with("metrics")
            })),
        )
        .init();

    info!(target: "metrics::cool_stuff_count", value = 42);
}












// use std::fmt;
// use tonic::{Status};
//
// // Define your custom error type
// #[derive(Debug)]
// pub enum MyError {
//     NotFound,
//     InternalError(String),
//     TonicError(Status),
// }
//
// // Implement `fmt::Display` for your custom error type
// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             MyError::NotFound => write!(f, "Resource not found"),
//             MyError::InternalError(msg) => write!(f, "Internal error: {}", msg),
//             MyError::TonicError(status) => write!(f, "Tonic error: {}", status.message()),
//         }
//     }
// }
//
// // Implement `std::error::Error` for your custom error type
// impl std::error::Error for MyError {}
//
// // Implement `From` for converting `tonic::Status` to `MyError`
// impl From<Status> for MyError {
//     fn from(status: Status) -> Self {
//         MyError::TonicError(status)
//     }
// }
//
// // Implement `From` for converting `String` to `MyError`
// impl From<String> for MyError {
//     fn from(s: String) -> Self {
//         MyError::InternalError(s)
//     }
// }
//
// // Example function that returns a `Result` and uses the `?` operator
// async fn example_function() -> Result<(), MyError> {
//     let some_condition = true;
//
//     if some_condition {
//         // Using the `?` operator with your custom error type
//         Err(MyError::NotFound)?; // This will return the error early
//     }
//
//     // Simulate a Tonic call that might return an error
//     let tonic_result: Result<(), Status> = Err(Status::internal("Something went wrong"));
//
//     // The `?` operator will use `From<Status>` to convert the error
//     tonic_result.map_err(|e| e.into())?; // This will convert the tonic error to your custom error
//
//     Ok(())
// }
//
// #[tokio::main]
// async fn main() {
//     match example_function().await {
//         Ok(_) => println!("Success!"),
//         Err(e) => println!("Error: {}", e),
//     }
// }
