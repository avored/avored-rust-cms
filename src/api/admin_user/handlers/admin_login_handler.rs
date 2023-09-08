use std::sync::Arc;

use crate::{avored_state::AvoRedState, error::Result};
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde::Serialize;

pub async fn admin_login_handler(state: State<Arc<AvoRedState>>) -> Result<impl IntoResponse> {
    println!("->> {:<12} - admin_login_handler", "HANDLER");
    let view_model = AdminLoginViewModel {};

    // let admin_users = state.admin_user_service.all_admin_users(&state.db).await?;
    // println!("{:?}", admin_users);
    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/auth/login", &view_model)
        .expect("there is an issue with handlerbar rendering auth/login.hbs template");

    Ok(Html(html))
}

#[derive(Serialize)]
pub struct AdminLoginViewModel {}
