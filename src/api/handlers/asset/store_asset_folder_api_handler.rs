use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use crate::api::handlers::asset::request::store_asset_folder_request::StoreAssetFolderRequest;
use crate::error::Error;
use crate::models::asset_model::NewAssetModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn store_asset_folder_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreAssetFolderRequest>,
) -> Result<Json<ApiResponse<NewAssetModel>>> {
    println!("->> {:<12} - store_asset_folder_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("asset_folder_create"))
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

    let created_asset_folder = state
        .asset_service
        .create_asset_folder(&state.db, payload.name, logged_in_user)
        .await?;

    let created_response = ApiResponse {
        status: true,
        data: created_asset_folder
    };

    Ok(Json(created_response))
}
