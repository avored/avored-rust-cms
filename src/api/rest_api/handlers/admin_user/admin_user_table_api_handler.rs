use std::sync::Arc;
use axum::extract::{Query, State};
use axum::Json;
use serde::Serialize;
use crate::api::rest_api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::Result;
use crate::models::admin_user_model::AdminUserPagination;
pub async fn admin_user_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<AdminUserPagination>> {
    println!("->> {:<12} - admin_user_table_api_handler", "HANDLER");

    let current_page = query_param.page.unwrap_or(1);
    let admin_user_pagination = state.admin_user_service.paginate(&state.db, current_page).await?;

    Ok(Json(admin_user_pagination))
}


#[derive(Serialize, Debug)]
pub struct FetchPageResponse {
    pub status: bool,
    pub page_model: AdminUserPagination
}