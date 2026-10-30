use crate::avored_state::AppState;
use crate::core::domain::entities::user::TokenClaims;
use crate::core::domain::entities::ApiErrorMessageResponse;
use crate::error::unauthorized;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{http::Request, middleware::Next, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};

/// Middleware to require JWT authentication for incoming requests
pub async fn check_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorMessageResponse>)> {
    if let Some(t) = req.headers().get("authorization") {
        let Ok(auth_value) = t.to_str() else {
            return Err(unauthorized(
                "Authentication failed: Invalid authorization header",
            ));
        };

        let jwt_token = &state.config.jwt_secret_key;
        let Some(token) = auth_value.strip_prefix("Bearer ") else {
            return Err(unauthorized(
                "Authentication failed: Invalid authorization header",
            ));
        };
        let claims = match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_token.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => token_data.claims,
            Err(jwt_error) => {
                let error_message = match jwt_error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                    jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => "Invalid token signature",
                    _ => "Token validation failed",
                };

                return Err(unauthorized(format!(
                    "Authentication failed: {error_message}"
                )));
            }
        };

        req.extensions_mut().insert(claims);

        Ok(next.run(req).await)
    } else {
        Err(unauthorized("You are not logged in, please provide token"))
    }
}

/// Middleware to require Customer JWT authentication for incoming requests
pub async fn check_customer_auth(
    axum::extract::State(state): axum::extract::State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiErrorMessageResponse>)> {
    if let Some(t) = req.headers().get("authorization") {
        let Ok(auth_value) = t.to_str() else {
            return Err(unauthorized(
                "Authentication failed: Invalid authorization header",
            ));
        };

        let jwt_token = &state.config.jwt_secret_key;
        let Some(token) = auth_value.strip_prefix("Bearer ") else {
            return Err(unauthorized(
                "Authentication failed: Invalid authorization header",
            ));
        };
        let claims = match decode::<TokenClaims>(
            token,
            &DecodingKey::from_secret(jwt_token.as_ref()),
            &Validation::default(),
        ) {
            Ok(token_data) => token_data.claims,
            Err(jwt_error) => {
                let error_message = match jwt_error.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => "Token expired",
                    jsonwebtoken::errors::ErrorKind::InvalidToken => "Invalid token",
                    jsonwebtoken::errors::ErrorKind::InvalidSignature => "Invalid token signature",
                    _ => "Token validation failed",
                };

                return Err(unauthorized(format!(
                    "Authentication failed: {error_message}"
                )));
            }
        };

        req.extensions_mut().insert(claims);

        Ok(next.run(req).await)
    } else {
        Err(unauthorized("You are not logged in, please provide token"))
    }
}
