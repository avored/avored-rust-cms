use std::sync::Arc;

use crate::models::page_model::PageModel;
use crate::{avored_state::AvoRedState, error::Result};

use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Serialize;

pub async fn fetch_page_api_handler(
    AxumPath(page_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_page_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_page"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let page_model = state.page_service.find_by_id(&state.db, page_id).await?;
    let response = FetchPageResponse {
        status: true,
        page_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub page_model: PageModel,
}
