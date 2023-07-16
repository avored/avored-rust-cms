use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde_derive::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::routes::AppState;

pub async fn get_admin_login_handler(
    session_read: ReadableSession,
    mut _session_write: WritableSession,
    app_state: State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut view_model = GetAdminLoginHandlerViewModel::new();

    let validation_email_message = session_read.get("validation_error_email");
    let validation_password_message = session_read.get("validation_error_password");

    view_model.validation_email_message = match validation_email_message {
        Some(message) => message,
        None => String::from(""),
    };
    view_model.validation_password_message = match validation_password_message {
        Some(message) => message,
        None => String::from(""),
    };

    let handlebars = &app_state.handlebars;

    let html = handlebars
        .render("auth/login", &view_model)
        .expect("there is an issue while loading the admin template");

    Html(html).into_response()
}

#[derive(Serialize)]
pub struct ValidationError {
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct GetAdminLoginHandlerViewModel {
    validation_messages: HashMap<String, ValidationError>,
    validation_email_message: String,
    validation_password_message: String,
}

impl GetAdminLoginHandlerViewModel {
    fn new() -> Self {
        GetAdminLoginHandlerViewModel {
            validation_messages: HashMap::new(),
            validation_email_message: String::from(""),
            validation_password_message: String::from(""),
        }
    }
}
