use std::sync::Arc;

use axum::{Router, routing::get};

use crate::avored_state::AvoRedState;

use super::handlers::admin_login_handler::admin_login_handler;

pub fn admin_user_routes(state: Arc<AvoRedState>) -> Router {
    Router::new().route("/admin/login", get(admin_login_handler))
    .with_state(state)
}
