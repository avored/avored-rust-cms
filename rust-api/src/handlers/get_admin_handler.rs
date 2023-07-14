use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_sessions::extractors::{ReadableSession, WritableSession};
use serde_json::json;
use std::sync::Arc;

use crate::{repositories::admin_user_repository::AdminUser, routes::AppState};

pub async fn get_admin_handler(
    _app_state: State<Arc<AppState>>,
    session: ReadableSession,
) -> impl IntoResponse {
    // let data = json!({});

    // let handlebars = &app_state.handlebars;

    let session: AdminUser = session.get("logged_in_user").unwrap();
    println!("Session: {:?}", session);

    let html = "Admin PAge";

    Html(html).into_response()
}
