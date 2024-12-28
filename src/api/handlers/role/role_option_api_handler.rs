use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::role_model::RoleOptionModel;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

pub async fn role_option_api_handler(
    state: State<Arc<AvoRedState>>,
) -> Result<Json<RoleOptionResponse>> {
    println!("->> {:<12} - role_option_api_handler", "HANDLER");

    let roles = state.role_service.all(&state.db).await?;

    let role_option_response = RoleOptionResponse {
        status: true,
        options: roles,
    };

    Ok(Json(role_option_response))
}

#[derive(Serialize, Debug)]
pub struct RoleOptionResponse {
    status: bool,
    options: Vec<RoleOptionModel>,
}
