use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use axum::http::{header, Response};
use axum::Json;
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Serialize;
use serde_json::json;
use crate::api::handlers::admin_user::request::authenticate_admin_user_request::AuthenticateAdminUserRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::token_claim_model::TokenClaims;
use crate::models::validation_error::ErrorResponse;

pub async fn admin_user_login_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<AuthenticateAdminUserRequest>,
) -> Result<Json<ResponseData>> {
    println!("->> {:<12} - admin_user_login_api_handler", "HANDLER");

    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let admin_user_model = state
        .admin_user_service
        .find_by_email(&state.db, payload.email.to_owned())
        .await?;

    let argon2 = Argon2::default();

    let parsed_hash = PasswordHash::new(&admin_user_model.password)?;
    if !argon2.verify_password(payload.password.as_bytes(), &parsed_hash).is_ok() {
        return Err(Error::AuthenticationError);
    }

    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: admin_user_model.id,
        name: admin_user_model.full_name,
        email:admin_user_model.email,
        exp,
        iat,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&state.config.jwt_secret_key.as_ref()),
    ).unwrap();
    let cookie = Cookie::build("token")
        .path("/")
        // .max_age(Duration::h)
        .same_site(SameSite::Lax)
        .http_only(true);
    let mut response = Response::new(json!({"status": "success", "token": token}).to_string());

    response
        .headers_mut()
        .insert(header::SET_COOKIE, cookie.to_string().parse().unwrap());
    let response_data = ResponseData {
        status: true,
        data: token,
    };


    Ok(Json(response_data))
}


#[derive(Serialize)]
pub struct ResponseData {
    status: bool,
    data: String
}