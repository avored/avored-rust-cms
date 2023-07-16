use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::ReadableSession;
use serde_derive::Serialize;
use std::{collections::HashMap, sync::Arc};

use crate::routes::AppState;

pub async fn get_admin_login_handler(
    session_read: ReadableSession,
    app_state: State<Arc<AppState>>,
) -> impl IntoResponse {
    let mut view_model = GetAdminLoginHandlerViewModel {
        validation_message: HashMap::new(),
    };
    let validation_message_string = session_read.get("validation_errors");

    if validation_message_string.is_some() {
        let validation_struct = ValidationError {
            message: validation_message_string,
        };

        view_model
            .validation_message
            .insert(String::from("email"), validation_struct);
    }

    // view_model.validation_message.insert("email", validation_message);

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
    validation_message: HashMap<String, ValidationError>,
}
