use axum::{extract::State, response::IntoResponse, Extension, Form, Json};
use serde_json::json;
use std::sync::Arc;

use crate::{
    repositories::admin_user_repository::AdminUser,
    requests::admin_user_create_request::StoreAdminUserRequest, routes::AppState,
};

pub async fn store_admin_user_handler(
    Extension(_current_user): Extension<AdminUser>,
    _app_state: State<Arc<AppState>>,
    Form(payload): Form<StoreAdminUserRequest>,
) -> impl IntoResponse {
    for part in payload.roles.split(",") {
        println!("{}", part)
    }
    // println!("{:?}", ;
    // let password = payload.password.as_bytes();
    // let salt = SaltString::generate(&mut OsRng);
    // let connection = establish_connection().await;

    // let argon2 = Argon2::default();
    // let password_hash = argon2
    //     .hash_password(password, &salt)
    //     .expect("Error occurred while encrypted password")
    //     .to_string();

    // let admin_user = app_state
    //     .admin_user_repository
    //     .create(
    //         connection,
    //         payload.name,
    //         payload.email,
    //         password_hash,
    //         current_user.email,
    //     )
    //     .await;

    Json(json!({})).into_response()
}
