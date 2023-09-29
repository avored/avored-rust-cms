use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::avored_state::AvoRedState;

use super::handlers::{
    post_setup_avored_handler::post_setup_avored_handler,
    setup_avored_handler::setup_avored_handler,
};

pub fn setup_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/setup", get(setup_avored_handler))
        .route("/setup", post(post_setup_avored_handler))
        // .route_layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     require_authentication,
        // ))
        // .route("/admin/login", get(admin_login_handler))
        // .route("/admin/login", post(authenticate_admin_user_handler))
        .with_state(state)
}
