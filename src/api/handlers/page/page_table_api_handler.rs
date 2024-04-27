use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::page_model::PagePagination;

pub async fn page_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<PagePagination>> {
    println!("->> {:<12} - page_table_api_handler", "HANDLER");

    let current_page = query_param.page.unwrap_or(1);
    let page_pagination = state.page_service.paginate(&state.db, current_page).await?;

    Ok(Json(page_pagination))
}
