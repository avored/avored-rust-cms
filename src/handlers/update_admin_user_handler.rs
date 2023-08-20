use std::collections::BTreeMap;
use std::sync::Arc;
use std::path::Path as stdPath;
use axum::extract::{State, Path, Multipart};
use axum::response::{IntoResponse, Redirect};
use rand::Rng;
use rand::distributions::Alphanumeric;
use surrealdb::dbs::Response;
use urlencoding::decode_binary;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::update_admin_user_request::UpdateAdminUserRequest;

pub async fn update_admin_user_handler(
    state: State<Arc<AvoRedState>>,
    Path(admin_user_id): Path<String>,
    mut session: AvoRedSession,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse>  {

    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let mut payload = UpdateAdminUserRequest {
        full_name: String::from(""),
        is_super_admin: false
    };

    let mut profile_image = String::from("");
    let mut existing_profile_image = String::from("");

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        match name.as_ref() {
            "image" => {
                let s: String = rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(32)
                                .map(char::from)
                                .collect();
                            
                // let file_content_test = field.content_type().unwrap().to_string();
                let file_name = field.file_name().unwrap().to_string();

                println!("file name: {}", file_name);

                let file_ext = file_name.split(".").last().unwrap_or(".png");

                let data = field.bytes().await.unwrap();

                let new_file_name = format!("{}.{}", s, file_ext);

              
                let file_name = stdPath::new(&new_file_name).file_name().unwrap();

                profile_image = format!("upload/{}", new_file_name);
                let full_path = stdPath::new("public").join("upload").join(file_name);
                tokio::fs::write(full_path, data).await.unwrap();
            }
            "full_name" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                let full_name = String::from_utf8_lossy(&decoded).into_owned();

                payload.full_name = full_name;
            }
            "existing_profile_image" => {
                let bytes = field.bytes().await.unwrap();
                let decoded = decode_binary(&bytes).into_owned();
                existing_profile_image = String::from_utf8_lossy(&decoded).into_owned();

               
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
            &_ => continue,
        }
    }

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

    if has_error {
        let redirect_url = format!("{}{}", String::from("/admin/edit-admin-user/"), admin_user_id);
        return Err(Redirect::to(&redirect_url).into_response());
    }


    if profile_image.len() <= 0 {
        profile_image = existing_profile_image;
    }

    let sql = "
    
    UPDATE type::thing($table, $id) MERGE {
        full_name: $full_name,
        profile_image: $profile_image,
        is_super_admin: $is_super_admin,
        updated_by: $logged_in_user_name,
        updated_at: time::now()
    };";

    let vars = BTreeMap::from([
        ("full_name".into(), payload.full_name.into()),
        ("logged_in_user_name".into(), logged_in_user.full_name.as_str().into()),
        ("profile_image".into(), profile_image.as_str().into()),
        ("is_super_admin".into(), payload.is_super_admin.into()),
        ("id".into(), admin_user_id.into()),
        ("table".into(), "admin_users".into()),
    ]);

    let _responses = match state.datastore.execute(sql, &state.database_session, Some(vars), false).await {
        Ok(response) => response,
        Err(_) => {
            let out: Vec<Response> = vec![];
            out
        }
    };

    // println!("{:?}", responses);
    //@try to find the error here?
    
    Ok(Redirect::to("/admin/admin-user").into_response())

    // Json(admin_users).into_response()
}
