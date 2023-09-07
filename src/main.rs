use async_session::MemoryStore;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

use crate::{
    api::admin_user::admin_user_routes::admin_user_routes,
    avored_state::AvoRedState,
    error::Result,
    providers::{avored_config_provider::config, avored_session_provider::SessionLayer},
};

mod api;
mod avored_state;
mod services;
mod models;
mod error;
mod providers;

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AvoRedState::new().await?);
    let store = MemoryStore::new();
    let config = config();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .merge(routes_hello(state.clone()))
        .merge(admin_user_routes(state))
        .nest_service("/public", static_routing_service)
        .layer(session_layer);

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");
    println!("");
    println!("");
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
        .route("/admin", get(handler_hello))
        .with_state(state)
}

async fn handler_hello() -> Result<impl IntoResponse> {
    println!("->> {:<12} - handler_hello", "HANDLER");
    
    let name = String::from("Avored Rust CMS");
    Ok(Html(format!("Hello <strong>{name}</strong>")))
}
