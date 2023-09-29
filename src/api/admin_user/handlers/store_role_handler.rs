use std::sync::Arc;

use crate::providers::avored_view_provider::translate;
use crate::{
    api::admin_user::requests::store_role_request::StoreRoleRequest,
    avored_state::AvoRedState,
    error::Result,
    models::{admin_user_model::AdminUserModel, role_model::CreatableRole},
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use validator::HasLen;

pub async fn store_role_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
    AvoRedForm(payload): AvoRedForm<StoreRoleRequest>,
) -> Result<impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };
    let validation_error_list = payload.validate_errors(session.clone())?;
    if validation_error_list.errors().length() > 0 {
        return Ok(Redirect::to("/admin/create-role").into_response());
    }

    let creatable_role = CreatableRole {
        name: payload.name,
        identifier: payload.identifier,
        permissions: payload.permissions,
        logged_in_username: logged_in_user.email,
    };

    let _created_role = state
        .role_service
        .create_role(&state.db, creatable_role)
        .await;
    session
        .insert("success_message", translate("success_created_role"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/role").into_response())
}
