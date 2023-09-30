use std::sync::Arc;

use crate::models::page_model::UpdatablePageModel;
use crate::providers::avored_view_provider::translate;
use crate::{
    api::page::requests::update_page_request::UpdatePageRequest,
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::{Path as AxumPath, State},
    response::{IntoResponse, Redirect},
};
use validator::HasLen;

pub async fn update_page_handler(
    mut session: AvoRedSession,
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    AvoRedForm(payload): AvoRedForm<UpdatePageRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_role_handler", "HANDLER");
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    let validation_error_list = payload.validate_errors(session.clone())?;

    if validation_error_list.errors().length() > 0 {
        let redirect_url = format!("/admin/edit-page/{}", page_id);
        return Ok(Redirect::to(&redirect_url).into_response());
    }

    let updateable_page_model = UpdatablePageModel {
        id: page_id,
        name: payload.name,
        identifier: payload.identifier,
        content: payload.content,
        logged_in_username: logged_in_user.email,
    };
    let _role_model = state
        .page_service
        .update_page(&state.db, updateable_page_model)
        .await?;
    session
        .insert("success_message", translate("success_updated_page"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/page").into_response())
}
