use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use tokio::fs;
use crate::api::handlers::asset::request::rename_asset_request::RenameAssetRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::asset_model::NewAssetModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn rename_asset_api_handler(
    Path(asset_id): Path<String>,
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Json(payload): Json<RenameAssetRequest>,
) -> Result<Json<ApiResponse<NewAssetModel>>> {
    println!("->> {:<12} - rename_asset_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("rename_asset"))
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

    let asset_model = state.asset_service
        .find_by_id(&state.db, &asset_id)
        .await?;
    let old_asset_path = format!(".{}",asset_model.path);
    let new_asset_path = format!("/public/upload/{}", &payload.name);

    if fs::try_exists(&old_asset_path).await? {
        fs::rename(&old_asset_path, &format!(".{}", new_asset_path)).await?;
    }

    let updated_asset_model = state.asset_service
        .update_asset_path(&state.db, &payload.name, &new_asset_path, &asset_id, &logged_in_user.email)
        .await?;

    let response = ApiResponse {
        status: false,
        data: updated_asset_model
    };

    Ok(Json(response))
}
