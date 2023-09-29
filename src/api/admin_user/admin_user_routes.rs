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
    create_admin_user_handler::create_admin_user_handler, create_role_handler::create_role_handler,
    dashboard_handler::dashboard_handler, delete_admin_user_handler::delete_admin_user_handler,
    delete_role_handler::delete_role_handler, edit_admin_user_handler::edit_admin_user_handler,
    edit_role_handler::edit_role_handler, role_table_handler::role_table_handler,
    show_admin_user_handler::show_admin_user_handler, show_role_handler::show_role_handler,
    store_admin_user_handler::store_admin_user_handler, store_role_handler::store_role_handler,
    update_admin_user_handler::update_admin_user_handler, update_role_handler::update_role_handler,
};

pub fn admin_user_routes(state: Arc<AvoRedState>) -> Router {
    Router::new()
        .route("/admin", get(dashboard_handler))
        .route("/admin/create-admin-user", get(create_admin_user_handler))
        .route("/admin/store-admin-user", post(store_admin_user_handler))
        .route(
            "/admin/delete-admin-user/:admin_user_id",
            post(delete_admin_user_handler),
        )
        .route(
            "/admin/show-admin-user/:admin_user_id",
            get(show_admin_user_handler),
        )
        .route(
            "/admin/edit-admin-user/:admin_user_id",
            get(edit_admin_user_handler),
        )
        .route(
            "/admin/update-admin-user/:admin_user_id",
            post(update_admin_user_handler),
        )
        .route("/admin/admin-user", get(admin_user_table_handler))
        .route("/admin/role", get(role_table_handler))
        .route("/admin/create-role", get(create_role_handler))
        .route("/admin/store-role", post(store_role_handler))
        .route("/admin/edit-role/:role_id", get(edit_role_handler))
        .route("/admin/update-role/:role_id", post(update_role_handler))
        .route("/admin/show-role/:role_id", get(show_role_handler))
        .route("/admin/delete-role/:role_id", post(delete_role_handler))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_authentication,
        ))
        .route("/admin/login", get(admin_login_handler))
        .route("/admin/login", post(authenticate_admin_user_handler))
        .with_state(state)
}
