use std::sync::Arc;
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{response::IntoResponse, Json, extract::State, Extension};
use rand_core::OsRng;

use crate::{routes::AppState, repositories::admin_user_repository::{NewAdminUser, AdminUser}, requests::admin_user_create_request::CreateAdminUserRequest};

pub async fn create_admin_user_handler(
        Extension(current_user): Extension<AdminUser>,
        app_state : State<Arc<AppState>>,
        Json(payload): Json<CreateAdminUserRequest>
    ) -> impl IntoResponse {

        

    let password = payload.password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let current = chrono::offset::Utc::now().naive_utc();
    let logged_in_user_email = current_user.email;

    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password, &salt).expect("Error occurred while encrypted password").to_string();
    let create_admin_user_model = NewAdminUser {
        name: payload.name,
        email: payload.email,
        password: password_hash,
        created_at: current,
        updated_at: current,
        created_by: logged_in_user_email.clone(),
        updated_by: logged_in_user_email

    };

    let admin_user = app_state.admin_user_repository.create(create_admin_user_model);

    Json(admin_user).into_response()
}

