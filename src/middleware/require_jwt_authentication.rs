use std::env;
use axum::body::Body;
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use axum::{http::Request, middleware::Next, Json};
use crate::models::token_claim_model::{LoggedInUser, TokenClaims};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use tonic::Status;
use crate::error::Error;

#[derive(Debug, Serialize, Default)]
pub struct ErrorResponse {
    pub status: bool,
    pub message: String,
}

pub async fn require_jwt_authentication(
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponse>)> {

    match req.headers().get("authorization") {
        Some(t) => {
            let auth_value = t.to_str()
                .map_err(|_e| Status::unavailable("authorization header value is not valid string")).unwrap();

            let jwt_token = &env::var("AVORED_JWT_SECRET")
                .map_err(|_| Error::ConfigMissing("AVORED_JWT_SECRET".to_string())).unwrap();
            let token = auth_value.strip_prefix("Bearer ").map(|auth| auth.to_owned());
            let claims = decode::<TokenClaims>(
                &token.unwrap_or_default(),
                &DecodingKey::from_secret(jwt_token.as_ref()),
                &Validation::default(),
            ).map_err(|_| {
                Status::unauthenticated("No valid auth token claims found")
            }).unwrap()
                .claims;

            let file_exist = true;
            let logged_in_user = LoggedInUser {
                id: claims.sub,
                name: claims.name,
                email: claims.email,
                demo_data_status: file_exist,
                admin_user_model: claims.admin_user_model,
            };
            req.extensions_mut().insert(logged_in_user);

            Ok(next.run(req).await)
        },
        _ => {
            let json_error = ErrorResponse {
                status: false,
                message: "You are not logged in, please provide token".to_string(),
            };
            Err((StatusCode::BAD_REQUEST, Json(json_error))) 
        },
    }
    // let token = cookie_jar
    //     .get("token")
    //     .map(|cookie| cookie.value().to_string())
    //     .or_else(|| {
    //         req.headers()
    //             .get(header::AUTHORIZATION)
    //             .and_then(|auth_header| auth_header.to_str().ok())
    //             .and_then(|auth_value| {
    //                 if auth_value.starts_with("Bearer ") {
    //                     match auth_value.strip_prefix("Bearer ") {
    //                         Some(auth) => Some(auth.to_owned()),
    //                         _ => None,
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             })
    //     });
    // 
    // // let mut is_token_valid = false;
    // 
    // let token = token.ok_or_else(|| {
    //     let json_error = ErrorResponse {
    //         status: false,
    //         message: "You are not logged in, please provide token".to_string(),
    //     };
    //     (StatusCode::UNAUTHORIZED, Json(json_error))
    // })?;
    // 
    // let secret = state.config.jwt_secret_key.clone();
    // 
    // let claims = decode::<TokenClaims>(
    //     &token,
    //     &DecodingKey::from_secret(secret.as_ref()),
    //     &Validation::default(),
    // )
    // .map_err(|_| {
    //     let json_error = ErrorResponse {
    //         status: false,
    //         message: "Invalid token".to_string(),
    //     };
    //     (StatusCode::UNAUTHORIZED, Json(json_error))
    // })?
    // .claims;
    // let file_exist = tokio::fs::try_exists("public/install_demo")
    //     .await
    //     .unwrap_or(false);
    // 
    // let logged_in_user = LoggedInUser {
    //     id: claims.sub,
    //     name: claims.name,
    //     email: claims.email,
    //     demo_data_status: file_exist,
    //     admin_user_model: claims.admin_user_model,
    // };
    // 
    // req.extensions_mut().insert(logged_in_user);
    // 
    // Ok(next.run(req).await)
}
