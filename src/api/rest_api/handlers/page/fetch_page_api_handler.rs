use std::sync::Arc;

use crate::models::page_model::PageModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;

pub async fn fetch_page_api_handler(
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_page_api_handler", "HANDLER");

    let page_model = state
        .page_service
        .find_by_id(&state.db, page_id)
        .await?;
    let response = FetchPageResponse {
        status: true,
        page_model: page_model
    };

    Ok(Json(response))
}


#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub page_model: PageModel
}