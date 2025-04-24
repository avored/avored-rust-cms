use std::env;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use axum::http::HeaderValue;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use axum_tonic::{NestTonic, RestGrpcService};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{filter, Layer};
use tower_http::services::ServeDir;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::api::admin_user_api::AdminUserApi;
use crate::api::auth_api::AuthApi;
use crate::api::content_api::ContentApi;
use crate::api::dashboard_api::DashboardApi;
use crate::api::misc_api::MiscApi;
use crate::api::proto::admin_user::admin_user_server::AdminUserServer;
use crate::api::proto::auth::auth_server::AuthServer;
use crate::api::proto::content::content_server::ContentServer;
use crate::api::proto::dashboard::dashboard_server::DashboardServer;
use crate::api::proto::echo::test2_server::Test2Server;
use crate::api::proto::misc::misc_server::MiscServer;
use crate::api::test_api::Test2Api;
use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::middleware::grpc_auth_middleware::check_auth;

mod api;
mod avored_state;
mod error;
mod models;
mod providers;
mod services;
mod requests;
mod repositories;
mod middleware;

const PER_PAGE: u64 = 100;

rust_i18n::i18n!("resources/locales");

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() -> Result<(), Error>{
    init_log();

    let state = Arc::new(AvoRedState::new().await?);

    let mut origins: Vec<HeaderValue> = vec![];
    for origin in &state.config.cors_allowed_app_url {
        origins.push(HeaderValue::from_str(origin).unwrap());
    }

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_headers(Any)
        .allow_methods(Any);


    let test_api = Test2Api {};
    let test_server = Test2Server::new(test_api);

    let misc_api = MiscApi {state: state.clone()};
    let misc_server = MiscServer::new(misc_api);

    let dashboard_api = DashboardApi {};
    let dashboard_server = DashboardServer::with_interceptor(dashboard_api, check_auth);

    let auth_api = AuthApi {state: state.clone()};
    let auth_server = AuthServer::new(auth_api);

    let admin_user_api = AdminUserApi {state: state.clone()};
    let admin_user_server = AdminUserServer::with_interceptor(admin_user_api, check_auth);

    let content_api = ContentApi {state: state.clone()};
    let content_server = ContentServer::with_interceptor(content_api, check_auth);
    
    let grpc_router = Router::new()
        .nest_tonic(test_server)
        .nest_tonic(misc_server)
        .nest_tonic(auth_server)
        .nest_tonic(dashboard_server)
        .nest_tonic(admin_user_server)
        .nest_tonic(content_server)
        .layer(cors.clone());


    let static_routing_service = ServeDir::new("public");

    let rest_router = Router::new()
        .route("/", get(handler))
        .nest_service("/public", static_routing_service)
        .with_state(state)
        .layer(cors);

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
}
