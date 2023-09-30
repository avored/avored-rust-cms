use async_session::MemoryStore;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{fs::File, net::SocketAddr, path::Path, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{
    filter, prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer,
};

use crate::api::page::page_routes::page_routes;
use crate::{
    api::{
        admin_user::admin_user_routes::admin_user_routes,
        component::component_routes::component_routes, setup::setup_routes::setup_routes,
    },
    avored_state::AvoRedState,
    error::Result,
    providers::{avored_config_provider::config, avored_session_provider::SessionLayer},
};

const PER_PAGE: i64 = 10;

mod api;
mod avored_state;
mod error;
mod middleware;
mod models;
mod providers;
mod repositories;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    init_log();
    let state = Arc::new(AvoRedState::new().await?);
    let store = MemoryStore::new();
    let config = config();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .merge(routes_hello(state.clone()))
        .merge(component_routes(state.clone()))
        .merge(page_routes(state.clone()))
        .merge(admin_user_routes(state.clone()))
        .merge(setup_routes(state))
        .nest_service("/public", static_routing_service)
        .layer(session_layer);

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");

    println!();
    println!();
    println!("Server started: http://localhost:8080");

    // region:    --- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("{:<12} - on {addr}\n", "LISTENING");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    // endregion: --- Start Server

    Ok(())
}

fn routes_hello(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/", get(handler_hello))
        .with_state(state)
}

async fn handler_hello(state: State<Arc<AvoRedState>>) -> Result<impl IntoResponse> {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let handlebar = &state.handlebars;
    let view_model = HomeViewModel {};
    let html = handlebar.render("home", &view_model)?;

    Ok(Html(html))
}

#[derive(serde::Serialize, Default)]
struct HomeViewModel {}

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
