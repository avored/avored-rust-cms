use std::sync::Arc;

use axum::routing::post;
use axum::{middleware, routing::get, Router};

use crate::api::component::handlers::create_component_handler::create_component_handler;
use crate::api::component::handlers::delete_component_handler::delete_component_handler;
use crate::api::component::handlers::edit_component_handler::edit_component_handler;
use crate::api::component::handlers::show_component_handler::show_component_handler;
use crate::api::component::handlers::store_component_handler::store_component_handler;
use crate::api::component::handlers::update_component_handler::update_component_handler;
use crate::{
    avored_state::AvoRedState, middleware::require_authentication::require_authentication,
};

use super::handlers::component_table_handler::component_table_handler;

pub fn component_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin/component", get(component_table_handler))
        .route("/admin/create-component", get(create_component_handler))
        .route("/admin/store-component", post(store_component_handler))
        .route(
            "/admin/edit-component/:component_id",
            get(edit_component_handler),
        )
        .route(
            "/admin/update-component/:component_id",
            post(update_component_handler),
        )
        .route(
            "/admin/show-component/:component_id",
            get(show_component_handler),
        )
        .route(
            "/admin/delete-component/:component_id",
            post(delete_component_handler),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .with_state(state)
}
