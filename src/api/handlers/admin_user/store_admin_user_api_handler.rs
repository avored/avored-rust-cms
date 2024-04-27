use std::path::Path;
use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::SaltString;
use axum::extract::{Multipart,  State};
use axum::{Extension, Json};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use urlencoding::decode_binary;
use crate::api::handlers::admin_user::request::store_admin_user_request::StoreAdminUserRequest;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::admin_user_model::{AdminUserModel, CreatableAdminUserModel};
use crate::models::token_claim_model::LoggedInUser;

pub async fn store_admin_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    mut multipart: Multipart
) -> Result<Json<CreateAdminUserResponse>> {
    println!("->> {:<12} - store_admin_user_api_handler", "HANDLER");

    let mut payload = StoreAdminUserRequest {
        full_name: String::from(""),
        email: String::from(""),
        password: String::from(""),
        is_super_admin: false,
        confirmation_password: String::from(""),
        role_ids: vec![]
    };
    let mut profile_image = String::from("");

    while let Some(field) = multipart.next_field().await.expect("cant find next field") {
        let name = field.name().expect("field name missing");

        println!("field name: {name}");

        match name {
            "image" => {
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect();

                let _file_content_type = field.content_type().unwrap().to_string();
                let file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.expect("data expected");

                if !file_name.is_empty() {
                    let file_ext = file_name.split(".").last().unwrap_or(".png");
                    let new_file_name = format!("{}.{}", s, file_ext);

                    let file_name = Path::new(&new_file_name).file_name().unwrap();

                    profile_image = format!("upload/{}", new_file_name);
                    let full_path = Path::new("public").join("upload").join(file_name);
                    tokio::fs::write(full_path, data).await.unwrap();
                }
            }
            "full_name" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let full_name = String::from_utf8_lossy(&decoded).into_owned();

                payload.full_name = full_name;
            }
            "email" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let email = String::from_utf8_lossy(&decoded).into_owned();

                payload.email = email;
            }
            "is_super_admin" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();

                let string_super_admin = String::from_utf8_lossy(&decoded).into_owned();
                let mut bool_super_admin = false;
                if string_super_admin.eq("true") {
                    bool_super_admin = true;
                }

                payload.is_super_admin = bool_super_admin;
            }
            "password" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let password = String::from_utf8_lossy(&decoded).into_owned();

                payload.password = password;
            }
            "confirmation_password" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let confirmation_password = String::from_utf8_lossy(&decoded).into_owned();

                payload.confirmation_password = confirmation_password;
            },
            "role_ids[]" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let role_id = String::from_utf8_lossy(&decoded).into_owned();

                payload.role_ids.push(role_id);
            }
            &_ => continue,
        }
    }

    // let mut has_error = false;
    // if payload.full_name.is_empty() {
    //     has_error = true;
    //     //@todo add validation
    // }
    // if payload.email.is_empty() {
    //     has_error = true;
    //     //@todo add validation
    // }
    // if payload.email.is_empty() {
    //     has_error = true;
    //     //@todo add validation
    // }
    //
    // if payload.password.is_empty() {
    //     has_error = true;
    //     //@todo add validation
    // }
    // if payload.confirmation_password.is_empty() {
    //     has_error = true;
    //     //@todo add validation
    // }
    // if payload.password.ne(&payload.confirmation_password) {
    //     has_error = true;
    //     //@todo return validation error response
    // }

    // println!("")

    let password = payload.password.as_bytes();
    let salt = SaltString::from_b64(&state.config.password_salt)?;

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let creatable_admin_user = CreatableAdminUserModel {
        full_name: payload.full_name,
        email: payload.email,
        password: password_hash,
        profile_image,
        is_super_admin: payload.is_super_admin,
        logged_in_username: logged_in_user.email.clone(),
        role_ids: payload.role_ids
    };

    let created_admin_user = state
        .admin_user_service
        .create_admin_user(&state.db, creatable_admin_user, logged_in_user)
        .await?;

    let create_admin_user_response = CreateAdminUserResponse {
        status: true,
        admin_user_model: created_admin_user
    };

    Ok(Json(create_admin_user_response))
}


#[derive(Serialize, Debug)]
pub struct CreateAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel
}