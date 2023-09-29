use std::sync::Arc;

use crate::providers::avored_view_provider::translate;
use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

pub async fn delete_component_handler(
    mut session: AvoRedSession,
    Path(component_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_component_handler", "HANDLER");
    state
        .component_service
        .delete_component(&state.db, component_id)
        .await?;

    session
        .insert("success_message", translate("success_deleted_component"))
        .expect("Could not store the validation errors into session.");

    Ok(Redirect::to("/admin/component").into_response())
}
