use crate::api::handlers::collection::request::collection_table_request::CollectionTableRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::CollectionPagination;
use crate::models::token_claim_model::LoggedInUser;
use axum::extract::{Query, State};
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn collection_table_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Query(query_param): Query<CollectionTableRequest>,
) -> Result<Json<CollectionPagination>> {
    println!("->> {:<12} - collection_table_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("collection_table"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let current_page = query_param.page.unwrap_or(0);
    let order = query_param.order.unwrap_or(String::from(""));
    let paginated_data = state
        .collection_service
        .paginate(&state.db, current_page, order)
        .await?;

    Ok(Json(paginated_data))
}
