use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::fs;
use crate::api::handlers::asset::request::create_folder_request::CreateFolderRequest;
use crate::error::Error;
use crate::models::asset_model::NewAssetModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn delete_folder_api_handler(
    Path(asset_id): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_folder_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("delete_folder_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    //@todo make sure folder is empty before delete?
    // delete the asset table record
    let mut entries = fs::read_dir("public/upload").await?;
    // let counter = entries.into_iter
    let mut counter = 0;
    while let Some(entry) = entries.next_entry().await? {
        // Here, `entry` is a `DirEntry`.
        println!("{:?}: {}", entry.file_name(), entry.ino());
        counter += 1;
    }

    println!("counter: {counter}");
    // let created_asset_folder = state
    //     .asset_service
    //     .create_asset_folder(&state.db, payload.name, logged_in_user)
    //     .await?;

    let created_response = ApiResponse {
        status: true,
        data: String::from("OK")
    };

    Ok(StatusCode::OK)
}
