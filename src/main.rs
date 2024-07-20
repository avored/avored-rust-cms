extern crate core;
use axum::Router;
use std::{fs::File, path::Path, sync::Arc};
use axum::extract::DefaultBodyLimit;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};
use crate::{
    avored_state::AvoRedState,
    error::Result
};
use crate::api::rest_api_routes::rest_api_routes;
use crate::providers::avored_graphql_provider::{ Context};

const PER_PAGE: i64 = 10;
mod models;
mod api;
mod middleware;
mod providers;
mod repositories;
mod services;
pub mod responses;
mod avored_state;
mod error;
mod query;

rust_i18n::i18n!("locales");

#[tokio::main]
async fn main() -> Result<()> {
    init_log();
    let state = Arc::new(AvoRedState::new().await?);
    let ctx = Arc::new(Context::new().await?);

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .merge(rest_api_routes(state.clone(), ctx))
        .nest_service("/public", static_routing_service)
        .layer(DefaultBodyLimit::max(104857600))
    ;

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");

    println!();
    println!();
    println!("Server started: http://0.0.0.0:8081");

    // region:    --- Start Server
    // let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind("0.0.0.0:8081").await.unwrap();
    info!("{:<12} - on {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener , app.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

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

    tracing::info!(target: "metrics::cool_stuff_count", value = 42);
}