use crate::{avored_state::AvoRedState, error::Result};
use std::sync::Arc;

use crate::error::Error;
use crate::models::admin_user_model::AdminUserModel;
use crate::models::token_claim_model::LoggedInUser;
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Serialize;

pub async fn fetch_admin_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(admin_user_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_admin_user_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_admin_user"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let admin_user_model = state
        .admin_user_service
        .find_by_id(&state.db, admin_user_id)
        .await?;
    let response = FetchAdminUserResponse {
        status: true,
        admin_user_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct FetchAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel,
}
