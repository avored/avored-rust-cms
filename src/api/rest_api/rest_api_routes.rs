use std::sync::Arc;

use axum::{routing::get, Router, middleware};
use axum::routing::post;
use crate::api::rest_api::handlers::component_all_api_handler::component_all_api_handler;
use crate::api::rest_api::handlers::health_check_api_handler::health_check_api_handler;
use crate::api::rest_api::handlers::admin_user::admin_user_login_api_handler::admin_user_login_api_handler;
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/api/component-all", get(component_all_api_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .route("/api/health-check", get(health_check_api_handler))
        .route("/api/login", post(admin_user_login_api_handler))
        .with_state(state)
}
