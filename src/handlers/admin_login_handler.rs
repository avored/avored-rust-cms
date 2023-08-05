use std::sync::Arc;

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;


pub async fn admin_login_handler(state: State<Arc<AvoRedState>>) -> impl IntoResponse {
    let view_model = AdminLoginHandlerViewModel::new();

    let handlebars = &state.handlebars;

    let html = handlebars
        .render("auth/login", &view_model)
        .expect("there is an issue with handlerbar rendering auth/login.hbs template");

    Html(html).into_response()
}

#[derive(Serialize)]
pub struct AdminLoginHandlerViewModel {
    validation_email_message: String,
    validation_password_message: String,
}

impl AdminLoginHandlerViewModel {
    fn new() -> Self {
        AdminLoginHandlerViewModel {
            validation_email_message: String::from(""),
            validation_password_message: String::from(""),
        }
    }
}
