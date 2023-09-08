use argon2::password_hash::{rand_core::OsRng, SaltString};
use async_session::MemoryStore;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

use crate::{
    api::{admin_user::admin_user_routes::admin_user_routes, setup::setup_routes::setup_routes},
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::CreatableAdminUser,
    providers::{avored_config_provider::config, avored_session_provider::SessionLayer},
};
use argon2::Argon2;
use argon2::PasswordHasher;

const PER_PAGE: i64 = 10;

mod api;
mod avored_state;
mod error;
mod middleware;
mod models;
mod providers;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(AvoRedState::new().await?);
    let store = MemoryStore::new();
    let config = config();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    let app = Router::new()
        .merge(routes_hello(state.clone()))
        .merge(admin_user_routes(state.clone()))
        .merge(setup_routes(state))
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
        .with_state(state)
}

async fn handler_hello(_state: State<Arc<AvoRedState>>) -> Result<impl IntoResponse> {
    println!("->> {:<12} - handler_hello", "HANDLER");

    // let password = "admin123";
    // let password = password.as_bytes();
    // let salt = SaltString::generate(&mut OsRng);

    // let argon2 = Argon2::default();
    // let password_hash = argon2
    //     .hash_password(password, &salt)
    //     .expect("Error occurred while encrypted password")
    //     .to_string();

    // // region : Move this admin user creation into home page
    // let creatable_admin_user_model = CreatableAdminUser {
    //     full_name: String::from("Admin"),
    //     email: String::from("admin@admin.com"),
    //     password: password_hash,
    //     profile_image: String::from(""),
    //     is_super_admin: true,
    //     logged_in_username: String::from("CLI"),
    // };

    // let created = state
    //     .admin_user_service
    //     .create_admin_user(&state.db, creatable_admin_user_model)
    //     .await?;

    // println!("{:?}", created);

    let name = String::from("Avored Rust CMS");
    Ok(Html(format!("Hello <strong>{name}</strong>")))
}
