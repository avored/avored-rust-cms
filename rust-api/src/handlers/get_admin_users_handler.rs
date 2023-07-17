use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::ReadableSession;
use serde_derive::Serialize;
use std::sync::Arc;

use crate::{repositories::admin_user_repository::AdminUser, routes::AppState};

pub async fn get_admin_users_handler(
    app_state: State<Arc<AppState>>,
    session: ReadableSession,
) -> impl IntoResponse {
    let handlebars = &app_state.handlebars;

    let logged_in_user: AdminUser = session.get("logged_in_user").unwrap();

    let data: AdminUsersListViewModel = AdminUsersListViewModel { logged_in_user };

    let html = handlebars.render("admin-users/list", &data).unwrap();

    Html(html).into_response()
}

#[derive(Debug, Serialize)]
struct AdminUsersListViewModel {
    logged_in_user: AdminUser,
}
