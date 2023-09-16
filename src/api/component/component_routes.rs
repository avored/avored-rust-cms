use std::sync::Arc;

use axum::{middleware, routing::get, Router};

use crate::{
    avored_state::AvoRedState, middleware::require_authentication::require_authentication,
};

use super::handlers::component_table_handler::component_table_handler;

pub fn component_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin/component", get(component_table_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .with_state(state)
}
