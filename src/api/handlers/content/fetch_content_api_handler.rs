use std::sync::Arc;
use crate::{avored_state::AvoRedState, error::Result};

use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use axum::{
    extract::{Path as AxumPath, State},
    Extension, Json,
};
use crate::models::content_model::ContentModel;
use crate::responses::ApiResponse;

pub async fn fetch_content_api_handler(
    AxumPath((content_type, content_id)): AxumPath<(String, String)>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
) -> Result<Json<ApiResponse<ContentModel>>> {
    println!("->> {:<12} - fetch_content_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_content"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }


    let model = state.content_service
        .find_by_id(&state.db, content_type, &content_id)
        .await?;

    let response = ApiResponse {
        status: true,
        data: model,
    };

    Ok(Json(response))
}

