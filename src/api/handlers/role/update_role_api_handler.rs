use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
    models::role_model::UpdatableRoleModel,
};
use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::api::handlers::role::request::update_role_request::UpdateRoleRequest;
use crate::models::role_model::RoleModel;
use crate::models::token_claim_model::LoggedInUser;

pub async fn update_role_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(role_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_role_api_handler", "HANDLER");

    // let validation_error_list = payload.validate_errors(session.clone())?;

    // if validation_error_list.errors().length() > 0 {
    //     let redirect_url = format!("/admin/edit-role/{}", role_id);
    //     return Ok(Redirect::to(&redirect_url).into_response());
    // }

    let updateable_role_model = UpdatableRoleModel {
        id: role_id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
        permissions: payload.permissions,
    };
    let updated_role_model = state
        .role_service
        .update_role(&state.db, updateable_role_model)
        .await?;

    let response = UpdatedRoleResponse {
        status: true,
        role_model: updated_role_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct UpdatedRoleResponse {
    pub status: bool,
    pub role_model: RoleModel
}