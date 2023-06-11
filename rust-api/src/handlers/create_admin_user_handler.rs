use std::sync::Arc;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{response::IntoResponse, Json, extract::State};
use rand_core::OsRng;
use serde::Deserialize;

use crate::routes::AppState;

pub async fn create_admin_user_handler(
        app_state : State<Arc<AppState>>,
        Json(payload): Json<CreateAdminUserPayload>
    ) -> impl IntoResponse {

    let password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).expect("Error occurred while encrypted password").to_string();
    
    let admin_user = app_state.admin_user_repository.create(payload.email.clone(), password_hash);

    println!("Admin USER PAYLOAD {} {}", payload.email, payload.password);

    Json(admin_user).into_response()
}

#[derive(Deserialize, Debug)]
pub struct CreateAdminUserPayload {
    email: String,
    password: String
}
