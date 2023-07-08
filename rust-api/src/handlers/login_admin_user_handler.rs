use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::{NaiveDateTime, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::routes::{AppState, establish_connection};

pub async fn login_admin_user_handler(
    app_state: State<Arc<AppState>>,
    Json(payload): Json<LoginAdminUserPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<LoginAdminUserResponse>)> {
    let connection = establish_connection().await;
    let admin_user: entity::admin_users::Model = app_state
        .admin_user_repository
        .find_by_email(connection, payload.email)
        .await;

    let is_valid = match PasswordHash::new(&admin_user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = LoginAdminUserResponse {
            success: false,
            message: String::from("Invalid email or password"),
        };
        return Err((StatusCode::BAD_REQUEST, Json(error_response)));
    }

    let jwt_secret = app_state.config.jwt_secret.as_ref();

    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(app_state.config.jwt_maxage))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: admin_user.id,
        name: admin_user.name,
        email: admin_user.email,
        password: admin_user.password,
        created_at: admin_user.created_at,
        updated_at: admin_user.updated_at,
        created_by: admin_user.created_by,
        updated_by: admin_user.updated_by,
        exp: expiration as usize,
    };

    //@todo use secret from config
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret),
    )
    .unwrap();
    let response = Response::new(json!({"status": "success", "token": token}).to_string());

    Ok(response)
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginAdminUserPayload {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginAdminUserResponse {
    success: bool,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_by: String,
    pub updated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub exp: usize,
}
