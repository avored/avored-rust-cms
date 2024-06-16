use std::sync::Arc;

use crate::models::role_model::RoleModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;

pub async fn fetch_role_api_handler(
    AxumPath(role_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_role_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_role"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let role_model = state
        .role_service
        .find_by_id(&state.db, role_id)
        .await?;
    let response = FetchPageResponse {
        status: true,
        role_model: role_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub role_model: RoleModel
}