use std::sync::Arc;

use axum::{routing::get, Router, middleware};
use crate::api::asset::handlers::asset_table_handler::asset_table_handler;

use crate::avored_state::AvoRedState;
use crate::middleware::require_authentication::require_authentication;

pub fn asset_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin/asset", get(asset_table_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .with_state(state)
}
