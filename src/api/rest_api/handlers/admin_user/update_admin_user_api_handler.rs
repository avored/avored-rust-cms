use std::path::Path;
use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use axum::extract::Multipart;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use urlencoding::decode_binary;
use crate::api::rest_api::handlers::admin_user::request::update_admin_user_request::UpdateAdminUserRequest;
use crate::models::admin_user_model::{AdminUserModel, UpdatableAdminUserModel};
use crate::models::token_claim_model::LoggedInUser;

pub async fn update_admin_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(admin_user_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    mut multipart: Multipart,
) -> Result<Json<UpdatableAdminUserResponse>> {
    println!("->> {:<12} - update_admin_user_api_handler", "HANDLER");

    let mut payload = UpdateAdminUserRequest {
        full_name: String::from(""),
        is_super_admin: false,
        role_ids: vec![]
    };
    let mut profile_image = String::from("");

    while let Some(field) = multipart.next_field().await.expect("cant find next field") {
        let name = field.name().expect("field name missing");

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
            "is_super_admin" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();

                let string_super_admin = String::from_utf8_lossy(&decoded).into_owned();
                let mut bool_super_admin = false;
                if string_super_admin.eq("true") {
                    bool_super_admin = true;
                }

                payload.is_super_admin = bool_super_admin;
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

    //@todo possible validation here


    let updateable_admin_user_model = UpdatableAdminUserModel {
        id: admin_user_id,
        full_name: payload.full_name,
        profile_image,
        is_super_admin: payload.is_super_admin,
        logged_in_username: logged_in_user.email.clone(),
        role_ids: payload.role_ids
    };
    let updated_admin_user_model = state
        .admin_user_service
        .update_admin_user(&state.db, updateable_admin_user_model, logged_in_user)
        .await?;
    let response = UpdatableAdminUserResponse {
        status: true,
        admin_user_model: updated_admin_user_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct UpdatableAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel
}