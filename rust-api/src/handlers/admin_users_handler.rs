use std::sync::Arc;
use axum::{response::IntoResponse, Json, extract::{State, Query}};
use serde_derive::Serialize;

use crate::{
    routes::AppState, 
    responses::admin_user_response::AdminUserResponse, 
    requests::admin_user_list_request::AdminUsersRequest
};

pub async fn admin_users_handler(
        app_state : State<Arc<AppState>>,
        Query(payload): Query<AdminUsersRequest>
    ) -> impl IntoResponse {

    let current_page: i64 = payload.current_page;
    let per_page = payload.per_page;
    let offset: i64 = (current_page - 1) * per_page;
    let from: i64 = offset + 1;
    let to: i64 = offset + per_page;

    let admin_users = app_state.admin_user_repository.paginate(per_page, offset);
    let admin_user_count = app_state.admin_user_repository.count();

    let mut has_more_pages = true;
    let mut last_page = false;

    if to >= admin_user_count {
        has_more_pages = false;
        last_page = true;
    }

    let paginate = Pagination {
        total: admin_user_count,
        per_page,
        current_page,
        from,
        to,
        has_more_pages,
        last_page
    };

    let mut admin_users_response: Vec<AdminUserResponse> = Vec::new();

    for admin_user_model in admin_users {
        let admin_user_response: AdminUserResponse = admin_user_model.into();
        admin_users_response.push(admin_user_response);
    }

    let response = AdminUserListResponse {
        rows: admin_users_response,
        paginate: paginate
    };

    Json(response).into_response()
}

#[derive(Serialize)]
struct AdminUserListResponse {
    rows: Vec<AdminUserResponse>,
    paginate: Pagination
}


#[derive(Serialize)]
struct Pagination {
    total: i64,
    per_page: i64,
    current_page: i64,
    from: i64,
    to: i64,
    last_page: bool,
    has_more_pages: bool
}
