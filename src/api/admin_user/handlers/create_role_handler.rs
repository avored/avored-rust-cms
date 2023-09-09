use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, models::role_model::RoleModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn create_role_handler(
    session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - role_create_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => RoleModel::default(),
    };

    let view_model = CreateRoleViewModel { logged_in_user };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/create-role", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/create-role.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct CreateRoleViewModel {
    pub logged_in_user: RoleModel,
}
