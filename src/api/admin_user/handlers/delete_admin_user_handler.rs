use std::sync::Arc;

use crate::{
    avored_state::AvoRedState, error::Result, providers::avored_session_provider::AvoRedSession,
};
use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
};

pub async fn delete_admin_user_handler(
    _session: AvoRedSession,
    Path(admin_user_id): Path<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - delete_admin_user_handler", "HANDLER");
    state.admin_user_service.delete_admin_user(&state.db, admin_user_id).await?;

    session
        .insert("success_message", "Admin User deleted successfully!")
        .expect("Could not store the validation errors into session.");

    Ok(Redirect::to("/admin/admin-user").into_response())
}
