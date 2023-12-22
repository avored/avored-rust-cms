use std::sync::Arc;
use axum::{http::Request, Json, middleware::Next, response::Response};
use axum::extract::State;
use axum::http::{header, StatusCode};

use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::avored_state::AvoRedState;
use crate::models::token_claim_model::TokenClaims;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String,
}

pub async fn require_jwt_authentication<T>(
    state: State<Arc<AvoRedState>>,
    cookie_jar: CookieJar,
    req: Request<T>,
    next: Next<T>,
) -> Result<Response, Json<ErrorResponse>> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        });

    let mut is_token_valid = false;

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: false,
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    }).unwrap();

    let secret = state.config.jwt_secret_key.clone();

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
        .map_err(|_| {
            let json_error = ErrorResponse {
                status: false,
                message: "Invalid token".to_string(),
            };
            (StatusCode::UNAUTHORIZED, Json(json_error))
        }).unwrap()
        .claims;

    if claims.sub.len() <= 0 {
        is_token_valid = true;
        // println!("Claim admin_user {claims:?}");
    }

    if is_token_valid {
            let json_error = ErrorResponse {
                status: false,
                message: "Invalid token".to_string(),
            };

        return Err(Json(json_error));
    }

    // println!("TOKEN {token:?}");

    Ok(next.run(req).await)
}


