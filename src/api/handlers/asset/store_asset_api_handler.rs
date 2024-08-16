use std::path::Path;
use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result
};
use axum::{Extension, extract::State, Json, response::IntoResponse};
use axum::extract::Multipart;
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use crate::error::Error;
use crate::models::asset_model::{AssetModel, CreatableAssetModel};
use crate::models::token_claim_model::LoggedInUser;

const ALLOW_TYPES: [&str; 3] = ["image/jpeg", "image/jpg", "image/png"];

pub async fn store_asset_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - store_asset_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("asset_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let mut creatable_asset_model = CreatableAssetModel::default();
    creatable_asset_model.logged_in_username = logged_in_user.email;
    let mut is_allow_file_type = true;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        match name.as_ref() {
            "file" => {
                let s: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(16)
                    .map(char::from)
                    .collect();

                let file_type = field.content_type().unwrap().to_string();
                creatable_asset_model.file_type = field.content_type().unwrap().to_string();

                is_allow_file_type = ALLOW_TYPES.contains(&file_type.as_str());

                if !is_allow_file_type {
                    continue;
                }
                let file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                creatable_asset_model.file_size = i64::try_from(data.len()).unwrap_or(0);

                if !file_name.is_empty() {
                    let file_ext = file_name.split('.').last().unwrap_or(".png");
                    let new_file_name = format!("{}.{}", s, file_ext);

                    let file_name = Path::new(&new_file_name).file_name().unwrap();

                    let asset_file = format!("public/upload/{}", new_file_name.clone());
                    creatable_asset_model.file_name = new_file_name.clone();
                    creatable_asset_model.file_path = asset_file;

                    let full_path = Path::new("public").join("upload").join(file_name);
                    tokio::fs::write(full_path, data).await.unwrap();
                }
            },
            &_ => continue
        }
    }

    if !is_allow_file_type {
        let asset_model = AssetModel::default();
        let creatable_asset_response = AssetResponseViewModel {
            asset_model,
            success: false
        };

        return Ok(Json(creatable_asset_response).into_response())
    }

    let asset_model = state.asset_service
        .create_asset(&state.db, creatable_asset_model)
        .await?;

    let creatable_asset_response = AssetResponseViewModel {
        asset_model,
        success: true
    };

    Ok(Json(creatable_asset_response).into_response())
}

#[derive(Serialize)]
pub struct AssetResponseViewModel {
    pub asset_model: AssetModel,
    pub success: bool
}
