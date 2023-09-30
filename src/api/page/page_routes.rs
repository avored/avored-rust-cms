use crate::api::page::handlers::page_table_handler::page_table_handler;
use crate::avored_state::AvoRedState;
use axum::routing::{get, post};
use axum::{middleware, Router};
use std::sync::Arc;
use crate::api::page::handlers::create_page_handler::create_page_handler;
use crate::api::page::handlers::edit_page_handler::edit_page_handler;
use crate::api::page::handlers::store_page_handler::store_page_handler;
use crate::api::page::handlers::update_page_handler::update_page_handler;
use crate::middleware::require_authentication::require_authentication;

pub fn page_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin/page", get(page_table_handler))
        .route("/admin/create-page", get(create_page_handler))
        .route("/admin/store-page", post(store_page_handler))
        .route(
            "/admin/edit-page/:page_id",
            get(edit_page_handler),
        )
        .route(
            "/admin/update-page/:page_id",
            post(update_page_handler),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .with_state(state)
}
