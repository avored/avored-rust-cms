use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use crate::api::handlers::page::request::page_table_request::PageTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::asset_model::AssetPagination;
use crate::models::token_claim_model::LoggedInUser;

pub async fn asset_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<PageTableRequest>,
) -> Result<Json<AssetPagination>> {
    println!("->> {:<12} - asset_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("asset_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let current_page = query_param.page.unwrap_or(1);
    let asset_pagination = state.asset_service.paginate(&state.db, current_page).await?;

    Ok(Json(asset_pagination))
}
