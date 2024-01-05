use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use crate::api::rest_api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::asset_model::AssetPagination;

pub async fn asset_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<AssetPagination>> {
    println!("->> {:<12} - asset_table_api_handler", "HANDLER");

    let current_page = query_param.page.unwrap_or(1);
    let asset_pagination = state.asset_service.paginate(&state.db, current_page).await?;

    Ok(Json(asset_pagination))
}
