use std::sync::Arc;

use crate::api::page::requests::store_page_request::StorePageRequest;
use crate::models::page_model::CreatablePageModel;
use crate::providers::avored_view_provider::translate;
use crate::{
    avored_state::AvoRedState, error::Result, models::admin_user_model::AdminUserModel,
    providers::avored_session_provider::AvoRedSession,
};
use avored_better_query::AvoRedForm;
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};
use validator::HasLen;

pub async fn store_page_handler(
    state: State<Arc<AvoRedState>>,
    mut session: AvoRedSession,
    AvoRedForm(payload): AvoRedForm<StorePageRequest>,
) -> Result<impl IntoResponse> {
    let logged_in_user = match session.get("logged_in_user") {
        Some(logged_in_user) => logged_in_user,
        None => AdminUserModel::default(),
    };

    println!("Payload: {:?}", payload);

    let validation_error_list = payload.validate_errors(session.clone())?;
    if validation_error_list.errors().length() > 0 {
        return Ok(Redirect::to("/admin/create-page").into_response());
    }


    let creatable_page = CreatablePageModel {
        name: payload.name,
        identifier: payload.identifier,
        content: payload.content,
        logged_in_username: logged_in_user.email.clone(),
    };

    let _created_page = state
        .page_service
        .create_page(&state.db, creatable_page)
        .await?;

    session
        .insert("success_message", translate("success_created_page"))
        .expect("Could not store the success message into session.");

    Ok(Redirect::to("/admin/page").into_response())
}
