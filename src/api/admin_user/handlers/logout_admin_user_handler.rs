use crate::providers::avored_session_provider::AvoRedSession;
use crate::error::Result;
use axum::{
    response::{IntoResponse, Redirect},
};

pub async fn logout_admin_user_handler(
    mut session: AvoRedSession,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - logout_admin_user_handler", "HANDLER");
    session.destroy();

    Ok(Redirect::to("/admin/login").into_response())
}
