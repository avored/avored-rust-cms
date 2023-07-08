use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;

mod app_error;
mod config;
mod handlers;
mod middleware;
mod repositories;
mod requests;
mod responses;
mod routes;

use crate::routes::app_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app: Router = app_routes().await;
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server started, listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
