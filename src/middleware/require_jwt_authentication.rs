use std::sync::Arc;
use axum::{http::Request, Json, middleware::Next};
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, StatusCode};
use axum::response::IntoResponse;

use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use crate::avored_state::AvoRedState;
use crate::models::token_claim_model::{LoggedInUser, TokenClaims};

#[derive(Debug, Serialize, Default)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String,
}

pub async fn require_jwt_authentication (
    state: State<Arc<AvoRedState>>,
    // Extension(ctx): Extension<Arc<Context>>,
    cookie_jar: CookieJar,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        match auth_value.strip_prefix("Bearer ") {
                            Some(auth) => Some(auth.to_owned()),
                            _ => None
                        }
                    } else {
                        None
                    }
                })
        });

    // let mut is_token_valid = false;

    let token = token.ok_or_else(|| {
        let json_error = ErrorResponse {
            status: false,
            message: "You are not logged in, please provide token".to_string(),
        };
        (StatusCode::UNAUTHORIZED, Json(json_error))
    })?;

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
        })?
        .claims;
    let file_exist = tokio::fs::try_exists("public/install_demo").await.unwrap_or(false);

    let logged_in_user = LoggedInUser {
        id: claims.sub,
        name: claims.name,
        email: claims.email,
        demo_data_status: file_exist,
        admin_user_model: claims.admin_user_model
    };

    req.extensions_mut().insert(logged_in_user);

    Ok(next.run(req).await)
}
