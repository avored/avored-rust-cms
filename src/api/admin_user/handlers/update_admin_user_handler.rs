use std::{path::Path, sync::Arc};

use crate::providers::avored_view_provider::translate;
use crate::{
    api::admin_user::requests::update_admin_user_request::UpdateAdminUserRequest,
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::{AdminUserModel, UpdatableAdminUserModel},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Multipart, Path as AxumPath, State},
    response::{IntoResponse, Redirect},
};
use rand::{distributions::Alphanumeric, Rng};
use urlencoding::decode_binary;

pub async fn update_admin_user_handler(
    mut session: AvoRedSession,
    AxumPath(admin_user_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_admin_user_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };
    let mut payload = UpdateAdminUserRequest {
        full_name: String::from(""),
        is_super_admin: false,
    };

    let mut profile_image = String::from("");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_ref() {
            "image" => {
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .map(char::from)
                    .collect();

                let _file_content_type = field.content_type().unwrap().to_string();
                let file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();

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
            }
            &_ => continue,
        }
    }

    let mut has_error = false;

    if payload.full_name.is_empty() {
        has_error = true;
        session
            .insert(
                "validation_error_full_name",
                String::from("Full name is required field"),
            )
            .expect("Could not store the validation errors into session.");
    }

    if has_error {
        let redirect_url = format!(
            "{}{}",
            String::from("/admin/edit-admin-user/"),
            admin_user_id
        );
        return Ok(Redirect::to(&redirect_url).into_response());
    }

    let updateable_admin_user_model = UpdatableAdminUserModel {
        id: admin_user_id,
        full_name: payload.full_name,
        is_super_admin: payload.is_super_admin,
        profile_image,
        logged_in_username: logged_in_user.email,
    };
    let _admin_user_model = state
        .admin_user_service
        .update_admin_user(&state.db, updateable_admin_user_model)
        .await?;

    session
        .insert("success_message", translate("success_updated_admin_user"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/admin-user").into_response())
}
