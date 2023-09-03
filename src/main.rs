use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;

use crate::{
    admin_user::admin_user_routes::admin_user_routes, avored_state::AvoRedState, error::Result,
};

mod admin_user;
mod avored_state;
mod error;
mod providers;

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AvoRedState::new().await);

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .merge(routes_hello(state.clone()))
        .merge(admin_user_routes(state))
        .nest_service("/public", static_routing_service);

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

async fn handler_hello() -> Result<impl IntoResponse >{
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = String::from("Avored Rust CMS");
    Ok(Html(format!("Hello <strong>{name}</strong>")))
}
