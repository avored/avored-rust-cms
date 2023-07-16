use crate::repositories::admin_user_repository::AdminUser;
use axum::{
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use axum_sessions::extractors::ReadableSession;

pub async fn require_authentication<T>(
    session: ReadableSession,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, impl IntoResponse> {
    let decoded = session.get("logged_in_user");
    println!("Session: {:?}", decoded);
    if decoded.is_none() {
        return Err(Redirect::to("/admin/login").into_response());
    }

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
}
