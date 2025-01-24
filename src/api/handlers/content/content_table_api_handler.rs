use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::token_claim_model::LoggedInUser;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use std::sync::Arc;
use crate::models::content_model::ContentPagination;
use crate::responses::ApiResponse;

pub async fn content_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<PageTableRequest>,
    Path(content_type):Path<String>,
) -> Result<Json<ApiResponse<ContentPagination>>> {
    println!("->> {:<12} - content_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("content_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let current_page = query_param.page.unwrap_or(0);
    let order = query_param.order.unwrap_or(String::from(""));
    let content_pagination = state
        .content_service
        .paginate(&state.db, &content_type, current_page, order)
        .await?;

    let response = ApiResponse {
        status: true,
        data: content_pagination
    };

    Ok(Json(response))
}
