use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::page_model::PagePagination;
use crate::models::token_claim_model::LoggedInUser;

pub async fn page_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<PagePagination>> {
    println!("->> {:<12} - page_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("page_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let current_page = query_param.page.unwrap_or(0);
    let order = query_param.order.unwrap_or(String::from(""));
    let page_pagination = state
        .page_service
        .paginate(&state.db, current_page, order).await?;

    Ok(Json(page_pagination))
}
