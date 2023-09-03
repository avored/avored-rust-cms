use std::sync::Arc;

use crate::{error::Result, avored_state::AvoRedState};
use axum::{response::{Html, IntoResponse}, extract::State};
use serde::Serialize;

pub async fn admin_login_handler(state: State<Arc<AvoRedState>>) -> Result<impl IntoResponse> {
    println!("->> {:<12} - admin_login_handler", "HANDLER");
    let view_model = AdminLoginViewModel{};

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("admin-user/auth/login", &view_model)
        .expect("there is an issue with handlerbar rendering auth/login.hbs template");

    Ok(Html(html))
}


#[derive(Serialize)]
pub struct AdminLoginViewModel {
    
}
