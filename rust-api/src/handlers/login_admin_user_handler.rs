use std::sync::Arc;
use argon2::{ Argon2, PasswordVerifier, PasswordHash};
use axum::{response::IntoResponse, Json, extract::State, http::{StatusCode, Response}};
use jsonwebtoken::{Header, EncodingKey, encode};
use serde::Deserialize;
use serde_derive::Serialize;
use serde_json::json;

use crate::routes::AppState;

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
    //@todo use secret from config
    let token = encode(&Header::default(), &admin_user, &EncodingKey::from_secret("secret".as_ref())).unwrap();
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
