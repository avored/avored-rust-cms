use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{
    avored_state::AvoRedState, middleware::require_authentication::require_authentication,
};

use super::handlers::{
    admin_login_handler::admin_login_handler, admin_user_table_handler::admin_user_table_handler,
    authenticate_admin_user_handler::authenticate_admin_user_handler,
    create_admin_user_handler::create_admin_user_handler, dashboard_handler::dashboard_handler,
};

pub fn admin_user_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin", get(dashboard_handler))
        .route("/admin/create-admin-user", get(create_admin_user_handler))
        .route("/admin/admin-user", get(admin_user_table_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .route("/admin/login", get(admin_login_handler))
        .route("/admin/login", post(authenticate_admin_user_handler))
        .with_state(state)
}
