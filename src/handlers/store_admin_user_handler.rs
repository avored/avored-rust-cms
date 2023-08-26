#![allow(unused)]

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::{Multipart, State};
use axum::response::{IntoResponse, Redirect};
use axum::Form;
use rand::Rng;
use rand::distributions::Alphanumeric;
use regex::Regex;
use std::collections::BTreeMap;
use std::path::Path;
use std::str::Bytes;
use std::sync::Arc;
use surrealdb::dbs::Response;
use urlencoding::decode_binary;
use validator::{HasLen, Validate, ValidationErrors, ValidationErrorsKind};

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::{AdminUser, ModelCount};
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::store_admin_user_request::StoreAdminUserRequest;

pub async fn store_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    // Form(payload): Form<StoreAdminUserRequest>,
    mut session: AvoRedSession,
    mut multipart: Multipart, // ) -> Result<impl IntoResponse, impl IntoResponse>  {
) -> Result<impl IntoResponse, impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };
    let mut payload = StoreAdminUserRequest {
        full_name: String::from(""),
        email: String::from(""),
        password: String::from(""),
        is_super_admin: false,
        confirmation_password: String::from(""),
    };
    let mut profile_image = String::from("");

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        match name.as_ref() {
            "image" => {
                let s: String = rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(32)
                                .map(char::from)
                                .collect();
                            
                let file_content_test = field.content_type().unwrap().to_string();
                let file_name = field.file_name().unwrap().to_string();

                let file_ext = file_name.split(".").last().unwrap_or(".png");

                let data = field.bytes().await.unwrap();

                let new_file_name = format!("{}.{}", s, file_ext);

              
                let file_name = Path::new(&new_file_name).file_name().unwrap();

                profile_image = format!("upload/{}", new_file_name);
                let full_path = Path::new("public").join("upload").join(file_name);
                tokio::fs::write(full_path, data).await.unwrap();
            }
            "full_name" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let full_name = String::from_utf8_lossy(&decoded).into_owned();

                payload.full_name = full_name;
                // println!("Full Name: {:?}", full_name);
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
                let mut  bool_super_admin = false;
                if string_super_admin.eq("1") {
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
            }
            &_ => continue,
        }
    }



    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();

    let mut has_error = false;
    if payload.full_name.len() <= 0 {
        has_error = true;
        session
            .insert(
                "validation_error_full_name",
                String::from("Full name is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }
    if payload.email.len() <= 0 {
        has_error = true;
        session
            .insert(
                "validation_error_email",
                String::from("Email is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }
    if payload.email.len() <= 0 {
        has_error = true;
        session
            .insert(
                "validation_error_email",
                String::from("Email is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }
    if !(email_regex.is_match(&payload.email)) {
        has_error = true;
        session
            .insert(
                "validation_error_email",
                String::from("Email is not in right format"),
            )
            .expect("Could not store the validation errors into session.");
    }


    if payload.password.len() <= 0 {
        has_error = true;
        session
            .insert(
                "validation_error_password",
                String::from("Password is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }
    if payload.confirmation_password.len() <= 0 {
        has_error = true;
        session
            .insert(
                "validation_error_confirmation_password",
                String::from("Confirmation Password is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }
    if payload.password.ne(&payload.confirmation_password) {
        has_error = true;
        session
            .insert(
                "validation_error_password",
                String::from("Password did not match confirmation password"),
            )
            .expect("Could not store the validation errors into session.");
    }

    let admin_user_count = state
        .admin_user_repository
        .has_email_address_taken(
            &state.datastore,
            &state.database_session,
            payload.email.clone()
        )
        .await
        .unwrap_or(ModelCount::new());

    if admin_user_count.count >= 1 {
        has_error = true;
        session
            .insert(
                "validation_error_email",
                String::from("Email address already take. Please use another email address"),
            )
            .expect("Could not store the validation errors into session.");
    }

    if has_error {
        println!("{:?}", payload);
        return Err(Redirect::to("/admin/create-admin-user").into_response());
    }
   
    //     let validation_error_list = match payload.validate() {
    //         Ok(_) => ValidationErrors::new(),
    //         Err(errors) => errors,
    //     };

    //     for (field_name, error) in validation_error_list.errors() {
    //         // let test = validation_error_list.errors();
    //         // let test= error::add("sdfs");
    //         match &error {
    //             ValidationErrorsKind::Field(field_errors) => {
    //                 for field_error in field_errors {
    //                     let message = match &field_error.message {
    //                         Some(message) => message,
    //                         None => continue,
    //                     };

    //                     if !message.is_empty() {
    //                         let validation_key = format!("validation_error_{}", field_name);
    //                         session
    //                             .insert(&validation_key, message)
    //                             .expect("Could not store the validation errors into session.");
    //                     }
    //                 }
    //             }
    //             ValidationErrorsKind::Struct(_) => continue,
    //             ValidationErrorsKind::List(_) => continue,
    //         }
    //     }
    //     if validation_error_list.errors().length() > 0 {
    //         return Err(Redirect::to("/admin/create-admin-user").into_response());
    //     }

    let sql = "
        CREATE admin_users CONTENT {
            full_name: $full_name,
            email: $email,
            password: $password,
            profile_image: $profile_image,
            is_super_admin: $is_super_admin,
            created_by: $logged_in_user_name,
            updated_by: $logged_in_user_name,
            created_at: time::now(),
            updated_at: time::now()
        };
    ";

    let password = payload.password;
    let password = password.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password, &salt)
        .expect("Error occurred while encrypted password")
        .to_string();

    let vars = BTreeMap::from([
        ("full_name".into(), payload.full_name.into()),
        ("email".into(), payload.email.into()),
        ("password".into(), password_hash.as_str().into()),
        ("profile_image".into(), profile_image.as_str().into()),
        ("is_super_admin".into(), payload.is_super_admin.into()),
        ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
    ]);

    let responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
        Ok(response) => response,
        Err(_) => {
            // todo improve this error
            let out: Vec<Response> = vec![];
            out
        }
    };
    
    // Ok(Redirect::to("/admin/admin-user").into_response())
    Ok(Redirect::to("/admin/admin-user").into_response())

    // Json(admin_users).into_response()
}

// pub struct  TestStruct<'a> {
//     image: Bytes<'a>
// }
