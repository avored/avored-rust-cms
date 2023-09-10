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

pub async fn dashboard_handler(
    state: State<Arc<AvoRedState>>,
    session: AvoRedSession,
) -> Result<impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    println!("->> {:<12} - dashboard_handler", "HANDLER");
    let view_model = DashboardViewModel { logged_in_user };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("dashboard", &view_model)
        .expect("there is an issue with handlerbar rendering dashboard.hbs template");

    Ok(Html(html))
}

#[derive(Serialize)]
pub struct DashboardViewModel {
    logged_in_user: AdminUserModel,
}
