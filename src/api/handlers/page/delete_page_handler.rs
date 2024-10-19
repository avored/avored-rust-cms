use std::sync::Arc;
use serde::Serialize;
use crate::{ error::Result };
use axum::{Extension, extract::Path};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::avored_state::AvoRedState;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;

pub async fn delete_page_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    Path(page_id): Path<String>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_delete"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }
    state.page_service.remove_by_id(&state.db, &page_id).await?;
    Ok(StatusCode::OK)
}

#[derive(Serialize, Debug)]
pub struct RemovedPageResponse {
    pub status: bool,
}