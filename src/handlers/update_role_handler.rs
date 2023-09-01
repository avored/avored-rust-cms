use std::sync::Arc;

use avored_better_query::AvoRedForm;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Redirect};
use validator::HasLen;

use crate::avored_state::AvoRedState;
use crate::models::admin_user_model::AdminUser;
use crate::models::role_model::{RoleModel, UpdatableRole};
use crate::providers::avored_session_provider::AvoRedSession;
use crate::requests::update_role_request::UpdateRoleRequest;
use crate::requests::ValidateRequest;

pub async fn update_role_handler(
    mut session: AvoRedSession,
    Path(role_id): Path<String>,
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<UpdateRoleRequest>,
) -> impl IntoResponse {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUser::empty_admin_user(),
    };

    let validation_error_list = payload.validation_error(&mut session);

    if validation_error_list.errors().length() > 0 {
        let redirect_url = format!("{}{}", String::from("/admin/edit-role/"), role_id);
        return Err(Redirect::to(&redirect_url).into_response());
    }

    let updatable_role = UpdatableRole {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_user_email: logged_in_user.email,
        id: role_id,
    };

    match state
        .role_service
        .update_role(&state.datastore, &state.database_session, updatable_role)
        .await
    {
        Ok(role_model) => role_model,
        Err(_) => RoleModel::empty(),
    };

    Ok(Redirect::to("/admin/role").into_response())
}
