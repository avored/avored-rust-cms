use crate::api::handlers::collection::request::update_collection_request::UpdateCollectionRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::{CollectionModel, UpdatableCollection};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn update_collection_api_handler(
    state: State<Arc<AvoRedState>>,
    Path(collection_id): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Json(payload): Json<UpdateCollectionRequest>,
) -> Result<Json<ApiResponse<CollectionModel>>> {
    println!("->> {:<12} - update_collection_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("collection_update"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let creatable_collection = UpdatableCollection {
        name: payload.name,
        id: collection_id,
        logged_in_username: logged_in_user.email,
    };

    let updated_model = state
        .collection_service
        .update_collection(&state.db, creatable_collection)
        .await?;
    let response = ApiResponse {
        status: true,
        data: updated_model,
    };

    Ok(Json(response))
}
