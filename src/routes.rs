use async_session::MemoryStore;
use axum::{routing::{get, post}, Router, middleware};
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
    }, middleware::require_authentication::require_authentication,
};

pub fn routes(state: Arc<AvoRedState>, config: AvoRedConfigProvider) -> Router {
    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    Router::new()
        .route("/", get(home_handler))
        .route("/admin", get(admin_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .route("/admin/login", post(authenticate_admin_user_handler))
        .route("/admin/login", get(admin_login_handler))
        .nest_service("/public", static_routing_service)
        .with_state(state)
        .layer(session_layer)
}
