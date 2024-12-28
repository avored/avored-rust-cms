use crate::{avored_state::AvoRedState, error::Result};
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub async fn fetch_page_cms_api_handler(
    AxumPath(page_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - fetch_page_cms_api_handler", "HANDLER");
    let page_model = state.page_service.find_by_id(&state.db, page_id).await?;

    let res = page_model.convert_to_response()?;

    Ok(Json(res))
}
