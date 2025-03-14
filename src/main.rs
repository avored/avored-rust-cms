use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};
use crate::api::misc_api::MiscApi;
use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::grpc_misc::misc_server::MiscServer;

mod avored_state;
mod providers;
mod requests;
mod models;
mod api;
mod error;
mod services;

pub mod grpc_misc {
    tonic::include_proto!("misc");
}

rust_i18n::i18n!("resources/locales");


#[tokio::main]
async fn main() -> Result<(), Error> {
    init_log();

    let port = env::var("PORT").unwrap_or("50051".to_string());
    let addr = format!("127.0.0.1:{}", port).parse().map_err(|_e| Error::Generic(String::from("address parse error")))?;
    let state = Arc::new(AvoRedState::new().await?);

    // region: Grpc Service region
    let misc_api = MiscApi {state: state.clone()};
    let misc_server = MiscServer::new(misc_api);
    let misc_grpc = tonic_web::enable(misc_server);
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
        .add_service(misc_grpc)
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
