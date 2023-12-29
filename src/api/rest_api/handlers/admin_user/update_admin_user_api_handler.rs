use std::sync::Arc;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{extract::{Path as AxumPath, State}, Json};
use serde::Serialize;
use crate::api::rest_api::handlers::admin_user::request::update_admin_user_request::UpdateAdminUserRequest;
use crate::models::admin_user_model::{AdminUserModel, UpdatableAdminUserModel};

pub async fn update_admin_user_api_handler(
    AxumPath(admin_user_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateAdminUserRequest>,
) -> Result<Json<UpdatableAdminUserResponse>> {
    println!("->> {:<12} - update_admin_user_api_handler", "HANDLER");

    // let _validation_error_list = payload.validate_errors()?;

    // println!("Validation error list: {:?}", validation_error_list);

    let updateable_admin_user_model = UpdatableAdminUserModel {
        id: admin_user_id,
        full_name: payload.full_name,
        profile_image: "".to_string(),
        is_super_admin: payload.is_super_admin,
        logged_in_username: "admin@admin.com".to_string(),
    };
    let updated_admin_user_model = state
        .admin_user_service
        .update_admin_user(&state.db, updateable_admin_user_model)
        .await?;
    let response = UpdatableAdminUserResponse {
        status: true,
        admin_user_model: updated_admin_user_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct UpdatableAdminUserResponse {
    pub status: bool,
    pub admin_user_model: AdminUserModel
}