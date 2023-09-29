use std::sync::Arc;

use crate::{
    api::admin_user::requests::role_table_query::RoleTableQuery,
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, role_model::RolePagination},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn role_table_handler(
    mut session: AvoRedSession,
    Query(query_param): Query<RoleTableQuery>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - role_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let current_page = query_param.page.unwrap_or(1);

    let role_pagination = state.role_service.paginate(&state.db, current_page).await?;
    let success_message = session.get("success_message").unwrap_or(String::from(""));
    session.remove("success_message");

    let view_model = RoleViewModel {
        logged_in_user,
        role_pagination,
        success_message,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/role-table", &view_model)
        .expect("there is an issue with handlebar rendering admin-user/role-table.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct RoleViewModel {
    pub logged_in_user: AdminUserModel,
    pub role_pagination: RolePagination,
    pub success_message: String,
}
