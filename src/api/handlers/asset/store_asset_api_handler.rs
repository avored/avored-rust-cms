use std::sync::Arc;

use crate::api::handlers::asset::request::store_asset_request::StoreAssetRequest;
use crate::error::Error;
use crate::models::asset_model::{AssetModel, CreatableAssetModel, MetaDataType};
use crate::models::token_claim_model::LoggedInUser;
use crate::{avored_state::AvoRedState, error::Result};
use axum::extract::{Multipart, Query};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;

const ALLOW_TYPES: [&str; 3] = ["image/jpeg", "image/jpg", "image/png"];

pub async fn store_asset_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<StoreAssetRequest>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - store_asset_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("asset_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    //@todo we need to move this logic to service
    // we need to make the parent_id works as per it path will be changed too. if exist
    let mut creatable_asset_model = CreatableAssetModel {
        logged_in_username: logged_in_user.email,
        ..Default::default()
    };
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

                // file type should be part of metadata
                // creatable_asset_model.file_type = field.content_type().unwrap().to_string();

                is_allow_file_type = ALLOW_TYPES.contains(&file_type.as_str());

                if !is_allow_file_type {
                    continue;
                }
                let file_name = field.file_name().unwrap().to_string();
                let data = field.bytes().await.unwrap();
                // file size should be part of metadata
                // creatable_asset_model.file_size = i64::try_from(data.len()).unwrap_or(0);

                if !file_name.is_empty() {
                    let file_ext = file_name.split('.').last().unwrap_or(".png");
                    let new_file_name = format!("{}.{}", s, file_ext);
                    let query_parent_id = query_param.parent_id.clone().unwrap_or_default();
                    let asset_file;

                    if !query_parent_id.is_empty() {
                        let parent_asset = &state
                            .asset_service
                            .find_by_id(&state.db, &query_parent_id)
                            .await?;

                        creatable_asset_model.parent_id = query_parent_id;
                        asset_file =
                            format!("/{}/{}", parent_asset.new_path, new_file_name.clone());
                    } else {
                        asset_file = format!("/public/upload/{}", new_file_name.clone());
                    }

                    creatable_asset_model.name = new_file_name.clone();
                    creatable_asset_model.asset_type = String::from("FILE");
                    creatable_asset_model.metadata = MetaDataType::FileTypeMetaData { file_type };

                    let full_path = format!("./{}", asset_file);

                    tokio::fs::write(full_path, data).await?;
                }
            }
            &_ => continue,
        }
    }

    if !is_allow_file_type {
        let asset_model = AssetModel::default();
        let creatable_asset_response = AssetResponseViewModel {
            asset_model,
            success: false,
        };

        return Ok(Json(creatable_asset_response).into_response());
    }

    let asset_model = state
        .asset_service
        .create_asset(&state.db, creatable_asset_model)
        .await?;

    let creatable_asset_response = AssetResponseViewModel {
        asset_model,
        success: true,
    };

    Ok(Json(creatable_asset_response).into_response())
}

#[derive(Serialize)]
pub struct AssetResponseViewModel {
    pub asset_model: AssetModel,
    pub success: bool,
}
