use std::sync::Arc;
use axum::{Extension, Json};
use axum::extract::{Path, State};
use crate::api::handlers::collection::request::put_collection_identifier_request::PutCollectionRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::{CollectionModel, PutCollectionIdentifierModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;

pub async fn put_collection_identifier_api_handler(
    Path(collection_id): Path<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutCollectionRequest>,
) -> Result<Json<ApiResponse<CollectionModel>>> {
    println!("->> {:<12} - put_collection_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("collection_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate(state.clone()).await?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let put_collection_identifier = PutCollectionIdentifierModel {
        id: collection_id,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };
    let updated_collection = state
        .collection_service
        .update_collection_identifier(&state.db, put_collection_identifier)
        .await?;


    let api_response = ApiResponse {
        status: true,
        data: updated_collection,
    };

    Ok(Json(api_response))
}