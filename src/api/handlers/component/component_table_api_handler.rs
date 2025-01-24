use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::component_model::ComponentPagination;
use crate::models::token_claim_model::LoggedInUser;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn component_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<ComponentPagination>> {
    println!("->> {:<12} - component_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("component_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let current_page = query_param.page.unwrap_or(0);
    let order = query_param.order.unwrap_or(String::from(""));
    let component_pagination = state
        .component_service
        .paginate(&state.db, current_page, order)
        .await?;

    Ok(Json(component_pagination))
}
