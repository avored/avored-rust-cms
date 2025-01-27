use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::CollectionModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::responses::ApiResponse;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn fetch_collection_by_identifier_api_handler (
    state: State<Arc<AvoRedState>>,
    Path(collection_identifier): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
) -> Result<Json<ApiResponse<CollectionModel>>> {
    println!("->> {:<12} - fetch_collection_by_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("get_model"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let collection_model = state
        .collection_service
        .find_by_identifier(&state.db, &collection_identifier)
        .await?;

    let response = ApiResponse {
        status: true,
        data: collection_model,
    };

    Ok(Json(response))
}
