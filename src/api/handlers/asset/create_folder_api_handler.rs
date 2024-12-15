use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use crate::api::handlers::asset::request::create_folder_request::CreateFolderRequest;
use crate::error::Error;
use crate::models::asset_model::AssetModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn create_folder_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<CreateFolderRequest>,
) -> Result<Json<ApiResponse<AssetModel>>> {
    println!("->> {:<12} - store_asset_folder_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("create_folder"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
    }

    let parent_id = payload.parent_id.unwrap_or_default();

    let created_asset_folder = state
        .asset_service
        .create_asset_folder(&state.db, payload.name, parent_id, logged_in_user)
        .await?;

    let created_response = ApiResponse {
        status: true,
        data: created_asset_folder
    };

    Ok(Json(created_response))
}
