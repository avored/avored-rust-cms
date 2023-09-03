use std::net::SocketAddr;

use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tracing::info;

use crate::error::Result;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().merge(routes_hello());

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

fn routes_hello() -> Router {
    Router::new().route("/", get(handler_hello))
}

async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    let name = String::from("Avored Rust CMS");
    Html(format!("Hello <strong>{name}</strong>"))
}
