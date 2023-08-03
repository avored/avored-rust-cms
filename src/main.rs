use std::net::SocketAddr;

use crate::routes::app_route;

mod handlers;
mod models;
mod routes;
mod error;

#[tokio::main]
async fn main() {
    let app = app_route();

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("Server Started: http://localhost:8080");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
