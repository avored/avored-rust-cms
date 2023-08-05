use std::sync::Arc;

use axum::extract::State;
use axum::response::{Html, IntoResponse};
use serde_derive::Serialize;

use crate::avored_state::AvoRedState;
use crate::providers::avored_session_provider::AvoRedSession;

pub async fn admin_login_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
) -> impl IntoResponse {
    let mut view_model = AdminLoginHandlerViewModel::new();

    let validation_email_message = session.get("validation_error_email");
    let validation_password_message = session.get("validation_error_password");

    session.remove("validation_error_email");
    session.remove("validation_error_password");

    view_model.validation_email_message = match validation_email_message {
        Some(message) => message,
        None => String::from(""),
    };
    view_model.validation_password_message = match validation_password_message {
        Some(message) => message,
        None => String::from(""),
    };


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
