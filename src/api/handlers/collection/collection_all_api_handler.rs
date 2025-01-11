use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::token_claim_model::LoggedInUser;
use axum::extract::State;
use axum::{Extension, Json};
use std::sync::Arc;
use crate::models::collection_model::CollectionModel;
use crate::responses::ApiResponse;

pub async fn collection_all_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>
) -> Result<Json<ApiResponse<Vec<CollectionModel>>>> {
    println!("->> {:<12} - collection_all_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user, String::from("collection_all"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }


    let collections = state
        .collection_service
        .all_collections(&state.db)
        .await?;

    let response = ApiResponse {
        status: true,
        data: collections,
    };

    Ok(Json(response))
}
