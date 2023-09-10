use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn admin_user_table_handler(
    session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - admin_user_table_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let admin_users = state.admin_user_service.paginate(&state.db, 0).await?;

    let view_model = AdminUserTableViewModel {
        logged_in_user,
        admin_users,
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
    pub admin_users: Vec<AdminUserModel>,
}
