use std::sync::Arc;

use crate::{
    api::admin_user::requests::update_role_request::UpdateRoleRequest,
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, role_model::UpdatableRoleModel},
    providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path as AxumPath, State},
    response::{IntoResponse, Redirect},
    Form,
};
use validator::HasLen;

pub async fn update_role_handler(
    mut session: AvoRedSession,
    AxumPath(role_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Form(payload): Form<UpdateRoleRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let validation_error_list = payload.validate_errors(session)?;

    if validation_error_list.errors().length() > 0 {
        let redirect_url = format!("/admin/edit-role/{}", role_id);
        return Ok(Redirect::to(&redirect_url).into_response());
    }

    let updateable_role_model = UpdatableRoleModel {
        id: role_id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };
    let _role_model = state
        .role_service
        .update_role(&state.db, updateable_role_model)
        .await?;
    
    session
        .insert("success_message", "User Role edited successfully!")
        .expect("Could not store the validation errors into session.");

    Ok(Redirect::to("/admin/role").into_response())
}
