use std::sync::Arc;

use crate::{
    app_error::AppError, handlers::login_admin_user_handler::Claims,
    repositories::admin_user_repository::AdminUser, routes::AppState,
};
use axum::{
    extract::State,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

pub async fn require_authentication<T>(
    app_state: State<Arc<AppState>>,
    headers: HeaderMap,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let full_token = headers.get("Authorization").unwrap();

    let authorized_token = full_token.to_str().unwrap().replace("Bearer ", "");

    // let token = authorized_token[6..];

    let decoded = decode::<Claims>(
        &authorized_token,
        &DecodingKey::from_secret(app_state.config.jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    if decoded.is_ok() {
        let token_data = decoded.unwrap();

        let user = AdminUser {
            id: token_data.claims.sub,
            name: token_data.claims.name,
            email: token_data.claims.email,
            // password: token_data.claims.password,
            created_at: token_data.claims.created_at,
            updated_at: token_data.claims.updated_at,
            created_by: token_data.claims.created_by,
            updated_by: token_data.claims.updated_by,
        };

        request.extensions_mut().insert(user);
        Ok(next.run(request).await)
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "You are not authorized for this",
        ))
    }
}
