use std::sync::Arc;

use crate::error::Error;
use crate::models::validation_error::ErrorResponse;
use crate::{
    avored_state::AvoRedState, error::Result
};
use axum::{Extension, extract::State, Json};
use serde::Serialize;
use crate::api::handlers::role::request::store_role_request::StoreRoleRequest;
use crate::models::role_model::{CreatableRole, RoleModel};
use crate::models::token_claim_model::LoggedInUser;


pub async fn store_role_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreRoleRequest>,
) -> Result<Json<CreatedRoleResponse>> {

    let error_messages = payload.validate()?;

    if error_messages.len() > 0 {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
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
        role_model: created_role_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct CreatedRoleResponse {
    pub status: bool,
    pub role_model: RoleModel
}
