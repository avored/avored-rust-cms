use std::sync::Arc;
use argon2::{ Argon2, PasswordVerifier, PasswordHash};
use axum::{response::IntoResponse, Json, extract::State, http::{StatusCode, Response}};
use chrono::{Utc, NaiveDateTime};
use jsonwebtoken::{Header, EncodingKey, encode};
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::json;
use uuid::Uuid;

use crate::{routes::AppState};

pub async fn login_admin_user_handler(
        app_state : State<Arc<AppState>>,
        Json(payload): Json<LoginAdminUserPayload>
    ) -> Result<impl IntoResponse, (StatusCode, Json<LoginAdminUserResponse>)> {

    let admin_user = app_state.admin_user_repository.find_by_email(payload.email.clone());

    // let parsed_hash = PasswordHash::new(&admin_user.password).expect("Error occurred while making password hash");

    let is_valid = match PasswordHash::new(&admin_user.password) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(payload.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true),
        Err(_) => false,
    };

    if !is_valid {
        let error_response = LoginAdminUserResponse{
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
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret)).unwrap();
    let response = Response::new(json!({"status": "success", "token": token}).to_string());
    
    Ok(response)

}
 
#[derive(Deserialize, Debug)]
pub struct LoginAdminUserPayload {
    email: String,
    password: String
}



#[derive(Serialize)]
pub struct LoginAdminUserResponse {
    success: bool,
    message: String
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
