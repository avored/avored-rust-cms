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
    mut session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - role_create_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => RoleModel::default(),
    };
    let validation_name_message = session
        .get("validation_error_name")
        .unwrap_or(String::from(""));
    let validation_identifier_message = session
        .get("validation_error_identifier")
        .unwrap_or(String::from(""));

    session.remove("validation_error_name");
    session.remove("validation_error_identifier");

    let view_model = CreateRoleViewModel {
        logged_in_user,
        validation_name_message,
        validation_identifier_message,
    };

    let handlebars = &state.handlebars;
    let html = handlebars
        .render("admin-user/create-role", &view_model)
        .expect("there is an issue with handlerbar rendering admin-user/create-role.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct CreateRoleViewModel {
    pub logged_in_user: RoleModel,
    pub validation_name_message: String,
    pub validation_identifier_message: String,
}
