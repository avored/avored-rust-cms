use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use serde_json::json;
use std::sync::Arc;

use crate::routes::AppState;

pub async fn admin_login_handler(app_state: State<Arc<AppState>>) -> impl IntoResponse {
    let data = json!({});

    let handlebars = &app_state.handlebars;

    let html = handlebars.render("auth/login", &data).expect("there is an issue while loading the admin template");

    Html(html).into_response()
}
