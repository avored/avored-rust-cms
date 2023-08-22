#[warn(unused_imports)]

use async_session::MemoryStore;
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

use crate::{
    avored_state::AvoRedState,
    handlers::{
        admin_handler::admin_handler, admin_login_handler::admin_login_handler,
        create_admin_user_handler::create_admin_user_handler,
        edit_admin_user_handler::edit_admin_user_handler,
        store_admin_user_handler::store_admin_user_handler,
        update_admin_user_handler::update_admin_user_handler,
        admin_user_table_handler::admin_user_table_handler,
        authenticate_admin_user_handler::authenticate_admin_user_handler,
        home_handler::home_handler,
        create_page_handler::create_page_handler,
        create_component_handler::create_component_handler,
        store_component_handler::store_component_handler,
    },
    middleware::log_request::log_request,
    middleware::require_authentication::require_authentication,
    providers::{
        avored_config_provider::AvoRedConfigProvider, avored_session_provider::SessionLayer,
    },
};

pub fn routes(state: Arc<AvoRedState>, config: AvoRedConfigProvider) -> Router {
    let store = MemoryStore::new();
    let session_layer = SessionLayer::new(store, config.session_secret_key.as_bytes());

    let static_routing_service = ServeDir::new("public");

    Router::new()
        .route("/", get(home_handler))
        .route("/admin/store-component", post(store_component_handler))
        .route("/admin/create-component", get(create_component_handler))
        .route("/admin/create-page", get(create_page_handler))
        .route("/admin/store-admin-user", post(store_admin_user_handler))
        .route("/admin/edit-admin-user/:admin_user_id", get(edit_admin_user_handler))
        .route("/admin/update-admin-user/:admin_user_id", post(update_admin_user_handler))
        .route("/admin/create-admin-user", get(create_admin_user_handler))
        .route("/admin/admin-user", get(admin_user_table_handler))
        .route("/admin", get(admin_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .route("/admin/login", post(authenticate_admin_user_handler))
        .route("/admin/login", get(admin_login_handler))
        .route_layer(middleware::from_fn(log_request))
        .nest_service("/public", static_routing_service)
        .with_state(state)
        .layer(session_layer)
}
