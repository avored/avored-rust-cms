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

pub async fn delete_asset_api_handler(
    Path(asset_id): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_folder_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("delete_asset_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let asset_model = state.asset_service
        .find_by_id(&state.db, &asset_id)
        .await?;

    let asset_path = format!("./{path}", path = asset_model.path);

    // @todo return early
    if fs::try_exists(&asset_path).await? {
        tokio::fs::remove_file(asset_path).await?;

        let result = state
            .asset_service
            .delete_by_id(&state.db, &asset_id)
            .await?;
        if !result {
            return Err(Error::Generic(String::from("there is an issue while deleting an asset record in DB")));
        }
    }



    Ok(StatusCode::OK)
}
