use std::sync::Arc;

use crate::{avored_state::AvoRedState, error::Result};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect},
};

pub async fn authenticate_admin_user_handler(
    _state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - authenticate_admin_user_handler", "HANDLER");
    Ok(Redirect::to("/admin/login"))
}
