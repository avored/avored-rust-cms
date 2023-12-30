use std::sync::Arc;

use crate::models::role_model::RoleModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;

pub async fn fetch_role_api_handler(
    AxumPath(role_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_role_api_handler", "HANDLER");

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