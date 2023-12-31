use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{extract::State, Json};
use serde::Serialize;
use crate::api::rest_api::handlers::role::request::store_role_request::StoreRoleRequest;
use crate::models::role_model::{CreatableRole, RoleModel};


pub async fn store_role_api_handler(
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreRoleRequest>,
) -> Result<Json<CreatedRoleResponse>> {
    // let _validation_error_list = payload.validate_errors()?;

    println!("{:?}", payload);

    let creatable_role = CreatableRole {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: "admin@admin.com".to_string(),
        permissions: payload.permissions,
    };

    let created_role_model = state
        .role_service
        .create_role(&state.db, creatable_role)
        .await?;
    let response = CreatedRoleResponse {
        status: true,
        role_model: created_role_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct CreatedRoleResponse {
    pub status: bool,
    pub role_model: RoleModel
}