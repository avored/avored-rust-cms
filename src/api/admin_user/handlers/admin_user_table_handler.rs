use std::sync::Arc;

use crate::{
    api::admin_user::requests::admin_user_table_query::AdminUserTableQuery,
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::{AdminUserModel, Pagination},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn admin_user_table_handler(
    mut session: AvoRedSession,
    Query(query_param): Query<AdminUserTableQuery>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - admin_user_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };
    let current_page = query_param.page.unwrap_or(1);

    let admin_user_pagination = state
        .admin_user_service
        .paginate(&state.db, current_page)
        .await?;
    let success_message = session
        .get("success_message")
        .unwrap_or(String::from(""));
        session.remove("success_message");

    let view_model = AdminUserTableViewModel {
        logged_in_user,
        success_message,
        admin_user_pagination,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/admin-user-table", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/table.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct AdminUserTableViewModel {
    pub logged_in_user: AdminUserModel,
    pub admin_user_pagination: Pagination,
    pub success_message: String
}
