use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use tokio::fs;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::{ErrorMessage, ErrorResponse};

pub async fn delete_folder_api_handler(
    Path(asset_id): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_folder_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("delete_folder_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let asset_model = state.asset_service
        .find_by_id(&state.db, &asset_id)
        .await?;

    let folder_path = format!("./{path}", path = asset_model.path);

    // @todo return early
    if fs::try_exists(&folder_path).await? {
        let mut entries = fs::read_dir(&folder_path).await?;
        let mut counter = 0;

        while let Some(_entry) = entries.next_entry().await? {
            counter += 1;
        }

        if counter > 0 {
            let error_messages = vec![ErrorMessage {
                key: String::from("folder_existed"),
                message: String::from("folder is not empty. Please make sure folder is empty before deleting it.")
            }];
            let error_response = ErrorResponse {
                status: false,
                errors: error_messages
            };

            return Err(Error::BadRequestError(error_response));
        }
        tokio::fs::remove_dir(&folder_path).await?;
    }

    let result = state
        .asset_service
        .delete_by_id(&state.db, &asset_id)
        .await?;
    if !result {
        return Err(Error::Generic(String::from("there is an issue while deleting an folder")));
    }

    Ok(StatusCode::OK)
}
