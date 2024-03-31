use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::models::admin_user_model::AdminUserModel;
use crate::models::token_claim_model::LoggedInUser;

pub async fn fetch_admin_user_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(admin_user_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_admin_user_api_handler {:?}", "HANDLER", logged_in_user);

    let admin_user_model = state
        .admin_user_service
        .find_by_id(&state.db, admin_user_id)
        .await?;
    let response = FetchAdminUserResponse {
        status: true,
        admin_user_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel
}