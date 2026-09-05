use crate::core::domain::entities::user::TokenClaims;
use crate::error::Error;
use axum::body::Body;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{http::Request, middleware::Next, Json};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use std::borrow::ToOwned;
use std::env;

#[derive(Debug, Serialize, Default)]
/// error response struct
pub struct ErrorResponse {
    /// status of the response
    pub status: bool,
    /// message of the response
    pub message: String,
}

/// Middleware to require JWT authentication for incoming requests
pub async fn check_auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    if let Some(t) = req.headers().get("authorization") {
        let auth_value = t.to_str().map_err(|_e| Error::Authentication).unwrap();

        let jwt_token = &env::var("AVORED_JWT_SECRET")
            .map_err(|_| Error::ConfigMissing("AVORED_JWT_SECRET".to_string()))
            .unwrap();
        let token = auth_value.strip_prefix("Bearer ").map(ToOwned::to_owned);
        let claims = match decode::<TokenClaims>(
            &token.unwrap_or_default(),
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

                let json_error = ErrorResponse {
                    status: false,
                    message: format!("Authentication failed: {error_message}"),
                };
                return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
            }
        };

        req.extensions_mut().insert(claims);

        Ok(next.run(req).await)
    } else {
        let json_error = ErrorResponse {
            status: false,
            message: "You are not logged in, please provide token".to_string(),
        };
        Err((StatusCode::UNAUTHORIZED, Json(json_error)))
    }
}

/// Middleware to require Customer JWT authentication for incoming requests
pub async fn check_customer_auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    if let Some(t) = req.headers().get("authorization") {
        let auth_value = t.to_str().map_err(|_e| Error::Authentication).unwrap();

        let jwt_token = &env::var("AVORED_JWT_SECRET")
            .map_err(|_| Error::ConfigMissing("AVORED_JWT_SECRET".to_string()))
            .unwrap();
        let token = auth_value.strip_prefix("Bearer ").map(ToOwned::to_owned);
        let claims = match decode::<TokenClaims>(
            &token.unwrap_or_default(),
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

                let json_error = ErrorResponse {
                    status: false,
                    message: format!("Authentication failed: {error_message}"),
                };
                return Err((StatusCode::UNAUTHORIZED, Json(json_error)));
            }
        };

        req.extensions_mut().insert(claims);

        Ok(next.run(req).await)
    } else {
        let json_error = ErrorResponse {
            status: false,
            message: "You are not logged in, please provide token".to_string(),
        };
        Err((StatusCode::UNAUTHORIZED, Json(json_error)))
    }
}
