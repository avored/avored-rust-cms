use async_session::MemoryStore;
use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::{
    avored_state::AvoRedState,
    handlers::{
        admin_handler::admin_handler, admin_login_handler::admin_login_handler,
        authenticate_admin_user_handler::authenticate_admin_user_handler,
        home_handler::home_handler,
    },
    providers::{
        avored_config_provider::AvoRedConfigProvider, avored_session_provider::SessionLayer,
    },
};

pub fn routes(state: AvoRedState, config: AvoRedConfigProvider) -> Router {
    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .route("/admin/login", post(authenticate_admin_user_handler))
        .route("/admin/login", get(admin_login_handler))
        .nest_service("/public", static_routing_service)
        .with_state(Arc::new(state))
        .layer(session_layer)
}
