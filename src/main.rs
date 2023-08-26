#[warn(unused_imports)]

use std::net::SocketAddr;

use crate::bootstrap::bootstrap;

mod bootstrap;
mod prelude;
mod error;
mod handlers;
mod models;
mod routes;
mod avored_state;
mod repositories;
mod providers;
mod requests;
mod middleware;
mod responses;
mod services;

const PER_PAGE: i64 = 10;

#[tokio::main]
async fn main() {
    let router = bootstrap().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!(r"     _             ____          _ ");
    println!(r"    / \__   _____ |  _ \ ___  __| |");
    println!(r"   / _ \ \ / / _ \| |_) / _ \/ _` |");
    println!(r"  / ___ \ V / (_) |  _ <  __/ (_| |");
    println!(r" /_/   \_\_/ \___/|_| \_\___|\__,_|");


    println!("Server Started: http://localhost:8080");
                                    
    
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
