use std::sync::Arc;
use std::vec;

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn admin_user_table_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let admin_users = state
        .admin_user_repository
        .paginate(&state.datastore, &state.database_session)
        .await;

    let admin_users = match admin_users {
        Ok(data) => data,
        Err(_) => panic!("no data found error"),
    };

    let mut view_model = AdminUserTableHandlerViewModel::new();
    view_model.admin_users = admin_users;

    view_model.logged_in_user = logged_in_user;

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/admin-user-table", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()

    // Json(admin_users).into_response()
}

#[derive(Serialize)]
pub struct AdminUserTableHandlerViewModel {
    logged_in_user: AdminUser,
    admin_users: Vec<AdminUser>,
}

impl AdminUserTableHandlerViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        AdminUserTableHandlerViewModel {
            logged_in_user,
            admin_users: vec![],
        }
    }
}
