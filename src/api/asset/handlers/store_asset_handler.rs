use std::path::Path;
use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
    models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use axum::{extract::State, Json, response::IntoResponse};
use axum::extract::Multipart;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;

pub async fn store_asset_handler(
    session: AvoRedSession,
    state: State<Arc<AvoRedState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - store_asset_handler", "HANDLER");
    let _logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let mut asset_file = String::from("");

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        match name.as_ref() {
            "file" => {
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

                    asset_file = format!("upload/{}", new_file_name);
                    let full_path = Path::new("public").join("upload").join(file_name);
                    tokio::fs::write(full_path, data).await.unwrap();
                }
            },
            &_ => continue
        }
    }

    let asset_response = AssetResponseViewModel {
        asset_path: asset_file
    };

    Ok(Json(asset_response).into_response())
}

#[derive(Serialize)]
pub struct AssetResponseViewModel {
    pub asset_path: String,
}
