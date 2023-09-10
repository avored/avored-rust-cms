use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn admin_login_handler(
    mut session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - admin_login_handler", "HANDLER");

    // let admin_users = state.admin_user_service.all_admin_users(&state.db).await?;
    // println!("{:?}", admin_users);

    let validation_email_message = session
        .get("validation_error_email")
        .unwrap_or(String::from(""));
    let validation_password_message = session
        .get("validation_error_password")
        .unwrap_or(String::from(""));

    session.remove("validation_error_email");
    session.remove("validation_error_password");

    let view_model = AdminLoginViewModel {
        validation_email_message,
        validation_password_message,
    };

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/auth/login", &view_model)
        .expect("there is an issue with handlerbar rendering auth/login.hbs template");

    Ok(Html(html))
}

#[derive(Serialize, Default)]
pub struct AdminLoginViewModel {
    pub validation_email_message: String,
    pub validation_password_message: String,
}
