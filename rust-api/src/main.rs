use std::net::SocketAddr;
use axum::Router;
use dotenvy::dotenv;

mod config;
mod routes;
mod app_error;
mod handlers;
mod repositories;
mod middleware;
mod responses;
mod requests;

use crate::routes::app_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app: Router = app_routes().await;

    // let database_url = std::env::var("DATABASE_URL").unwrap();
    // let db = Database::connect(&database_url)
    //     .await
    //     .expect("Failed to setup the database");

    //     let current = chrono::offset::Utc::now().naive_utc();

    // let post = admin_users::ActiveModel {
    //     id: Set(Uuid::new_v4()),
    //     name: Set(String::from("Amazing title 1")),
    //     email: Set(String::from("Lorem ipsum dolor sit amet.")),
    //     password: Set(String::from("Lorem ipsum dolor sit amet.")),
    //     created_at: Set(current),
    //     updated_at: Set(current),
    //     created_by: Set(String::from("CLI")),
    //     updated_by: Set(String::from("CLI"))
    //     ..Default::default()
    // };

    // let post: admin_users::Model = post.insert(&db).await.expect("error creating post record");




    // Json(json!({})).into_response()

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server started, listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
