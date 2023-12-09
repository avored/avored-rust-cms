use std::sync::Arc;

use axum::{routing::get, Router, middleware};
use axum::routing::post;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::header::HeaderValue;
use crate::api::rest_api::handlers::component_all_api_handler::component_all_api_handler;
use crate::api::rest_api::handlers::health_check_api_handler::health_check_api_handler;
use crate::api::rest_api::handlers::admin_user::admin_user_login_api_handler::admin_user_login_api_handler;
use crate::avored_state::AvoRedState;
use crate::middleware::require_jwt_authentication::require_jwt_authentication;
use tower_http::cors::CorsLayer;

pub fn rest_api_routes(state: Arc<AvoRedState>) -> Router {

    let backend_url = &state.config.front_end_app_url;

    println!("BACKEND URL {backend_url}");
    let cors_layer = CorsLayer::new()
        .allow_origin(backend_url.parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::PATCH,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ]);

    Router::new()
        .route("/api/component-all", get(component_all_api_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_jwt_authentication,
        ))
        .route("/api/health-check", get(health_check_api_handler))
        .route("/api/login", post(admin_user_login_api_handler))
        .with_state(state)
        .layer(cors_layer)
}
