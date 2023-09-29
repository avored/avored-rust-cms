use std::sync::Arc;

use crate::providers::avored_view_provider::translate;
use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

pub async fn delete_role_handler(
    mut session: AvoRedSession,
    Path(role_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_role_handler", "HANDLER");
    state.role_service.delete_role(&state.db, role_id).await?;

    session
        .insert("success_message", translate("success_deleted_role"))
        .expect("Could not store the validation errors into session.");

    Ok(Redirect::to("/admin/role").into_response())
}
