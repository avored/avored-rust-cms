use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::role_model::RolePagination;
use crate::models::token_claim_model::LoggedInUser;

pub async fn role_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<RolePagination>> {
    println!("->> {:<12} - role_table_api_handler", "HANDLER");
    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("role_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let current_page = query_param.page.unwrap_or(0);
    let order = query_param.order.unwrap_or(String::from(""));
    let role_pagination = state.role_service.paginate(&state.db, current_page, order).await?;

    Ok(Json(role_pagination))
}
