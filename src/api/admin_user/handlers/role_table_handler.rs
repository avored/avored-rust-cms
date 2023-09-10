use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, models::{admin_user_model::AdminUserModel, role_model::RoleModel},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn role_table_handler(
    session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - role_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let roles = state.role_service.paginate(&state.db, 0).await?;

    let view_model = RoleViewModel {
        logged_in_user,
        roles,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/role-table", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/role-table.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct RoleViewModel {
    pub logged_in_user: AdminUserModel,
    pub roles: Vec<RoleModel>,
}
