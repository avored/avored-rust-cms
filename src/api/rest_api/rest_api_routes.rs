use std::sync::Arc;

use axum::{routing::get, Router};
use crate::api::rest_api::handlers::component_all_api_handler::component_all_api_handler;
use crate::api::rest_api::handlers::health_check_api_handler::health_check_api_handler;

use crate::avored_state::AvoRedState;
// use crate::middleware::require_authentication::require_authentication;

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/api/component-all", get(component_all_api_handler))
        // .route_layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     require_authentication,
        // ))
        .route("/api/health-check", get(health_check_api_handler))
        .with_state(state)
}
