use std::sync::Arc;
use axum::extract::{State, Path};
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn show_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    Path(admin_user_id): Path<String>,
    session: AvoRedSession,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let admin_user_model = match state.admin_user_repository.find_by_id(&state.datastore, &state.database_session, admin_user_id).await {
        Ok(admin_user) => admin_user,
        Err(_) => AdminUser::empty_admin_user(),
    };

    let mut view_model = ShowAdminUserViewModel::new();
    view_model.admin_user_model = admin_user_model;

    view_model.logged_in_user = logged_in_user;
    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/show-admin-user", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()
}

#[derive(Serialize)]
pub struct ShowAdminUserViewModel {
    logged_in_user: AdminUser,
    admin_user_model: AdminUser,
}

impl ShowAdminUserViewModel {
    fn new() -> Self {
        let logged_in_user = AdminUser::empty_admin_user();
        let empty_admin_user_model = AdminUser::empty_admin_user();
        ShowAdminUserViewModel {
            logged_in_user,
            admin_user_model: empty_admin_user_model
        }
    }
}
