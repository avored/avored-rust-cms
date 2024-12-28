use std::sync::Arc;

use crate::api::handlers::role::request::store_role_request::StoreRoleRequest;
use crate::error::Error;
use crate::models::role_model::{CreatableRole, RoleModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::{avored_state::AvoRedState, error::Result};
use axum::{extract::State, Extension, Json};
use serde::Serialize;

pub async fn store_role_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreRoleRequest>,
) -> Result<Json<CreatedRoleResponse>> {
    println!("->> {:<12} - store_role_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("role_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let creatable_role = CreatableRole {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
        permissions: payload.permissions,
    };

    let created_role_model = state
        .role_service
        .create_role(&state.db, creatable_role)
        .await?;
    let response = CreatedRoleResponse {
        status: true,
        role_model: created_role_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct CreatedRoleResponse {
    pub status: bool,
    pub role_model: RoleModel,
}
