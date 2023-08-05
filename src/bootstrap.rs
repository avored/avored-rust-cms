use crate::{avored_state::AvoRedState, providers::avored_config_provider::AvoRedConfigProvider, routes::routes};
use axum::Router;
use dotenvy::dotenv;

pub async fn bootstrap() -> Router {
    dotenv().ok();
    env_logger::init();
    let config = AvoRedConfigProvider::new();
    let state = AvoRedState::new(config).await;

    // error!("this is printed by default"); example of how we can do logging

    routes(state)
}
