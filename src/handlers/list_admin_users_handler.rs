use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::ReadableSession;
use serde_derive::Serialize;
use std::sync::Arc;

use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::admin_user_list_request::AdminUsersRequest,
    responses::admin_users_paginate_response::AdminUsersPaginateResponse,
    routes::{establish_connection, AppState},
};

pub async fn list_admin_users_handler(
    app_state: State<Arc<AppState>>,
    Query(payload): Query<AdminUsersRequest>,
    session: ReadableSession,
) -> impl IntoResponse {
    let handlebars = &app_state.handlebars;

    let logged_in_user: AdminUser = session.get("logged_in_user").unwrap();
    let current_page: u64 = match payload.current_page {
        Some(current_page) => current_page,
        None => 1,
    };
    let per_page = match payload.per_page {
        Some(per_page) => per_page,
        None => 10,
    };
    let from = ((current_page - 1) * per_page) + 1;
    let to = from + per_page - 1;

    let connection = establish_connection().await;

    let paginate_data = app_state
        .admin_user_repository
        .paginate(connection, per_page, current_page)
        .await;

    let data: AdminUsersListViewModel = AdminUsersListViewModel {
        logged_in_user,
        paginate_data,
        from,
        to,
    };

    let html = handlebars.render("admin-users/list", &data).unwrap();

    Html(html).into_response()
}

#[derive(Serialize)]
struct AdminUsersListViewModel {
    logged_in_user: AdminUser,
    paginate_data: AdminUsersPaginateResponse,
    from: u64,
    to: u64,
}
