use async_session::MemoryStore;
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::{
    avored_state::AvoRedState,
    handlers::{admin_handler::admin_handler, home_handler::home_handler},
    providers::{avored_session_provider::SessionLayer, avored_config_provider::AvoRedConfigProvider},
};

pub fn routes(state: AvoRedState, config: AvoRedConfigProvider) -> Router {
    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .nest_service("/public", static_routing_service)
        .with_state(Arc::new(state))
        .layer(session_layer)
}
