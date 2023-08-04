use crate::{config::Config, routes::routes, state::State};
use axum::Router;
use dotenvy::dotenv;

pub async fn bootstrap() -> Router {
    dotenv().ok();
    env_logger::init();
    let config = Config::new();
    let state = State::new(config).await;

    // error!("this is printed by default"); example of how we can do logging

    routes(state)
}
