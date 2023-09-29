use std::sync::Arc;

use crate::models::component_model::UpdatableComponentModel;
use crate::providers::avored_view_provider::translate;
use crate::{
    api::component::requests::update_component_request::UpdateComponentRequest,
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::{Path as AxumPath, State},
    response::{IntoResponse, Redirect},
};
use validator::HasLen;

pub async fn update_component_handler(
    mut session: AvoRedSession,
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<UpdateComponentRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let validation_error_list = payload.validate_errors(session.clone())?;

    if validation_error_list.errors().length() > 0 {
        let redirect_url = format!("/admin/edit-component/{}", component_id);
        return Ok(Redirect::to(&redirect_url).into_response());
    }

    let updateable_component_model = UpdatableComponentModel {
        id: component_id,
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };
    let _role_model = state
        .component_service
        .update_component(&state.db, updateable_component_model)
        .await?;
    session
        .insert("success_message", translate("success_updated_component"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/component").into_response())
}
