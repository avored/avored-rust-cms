use std::sync::Arc;

use crate::{repositories::admin_user_repository::AdminUser, routes::AppState};
use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use axum_sessions::extractors::ReadableSession;
use serde_json::json;

pub async fn require_authentication<T>(
    session: ReadableSession,
    app_state: State<Arc<AppState>>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, impl IntoResponse> {
    let decoded = session.get("logged_in_user");
    println!("Session: {:?}", decoded);
    if !decoded.is_none() {
        let token_data: AdminUser = decoded.unwrap();

        let user = AdminUser {
            id: token_data.id,
            name: token_data.name,
            email: token_data.email,
            created_at: token_data.created_at,
            updated_at: token_data.updated_at,
            created_by: token_data.created_by,
            updated_by: token_data.updated_by,
        };

        request.extensions_mut().insert(user);

        *request.uri_mut() = "/admin/login".parse().unwrap();

        Ok(next.run(request).await)
    } else {
        let data = json!({});
        let html = app_state.handlebars.render("401", &data).unwrap();

        Err(Html(html).into_response())
    }
}
