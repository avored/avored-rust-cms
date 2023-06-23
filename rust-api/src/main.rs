use std::net::SocketAddr;
use axum::Router;
use dotenvy::dotenv;

mod config;
mod routes;
mod app_error;
mod handlers;
mod models;
mod schema;
mod repositories;
mod middleware;
mod responses;
mod requests;

use crate::routes::app_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app: Router = app_routes();

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
