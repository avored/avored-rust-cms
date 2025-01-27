use crate::api::handlers::collection::request::update_collection_request::UpdateCollectionRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::{CollectionModel, UpdatableCollection, UpdatableCollectionField};
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

    let existing_collection = state
        .collection_service
        .find_by_id(&state.db, collection_id)
        .await?;

    let mut updatable_collection = UpdatableCollection {
        id: existing_collection.id,
        name: payload.name,
        identifier: existing_collection.identifier,
        created_at: existing_collection.created_at,
        created_by: existing_collection.created_by,
        logged_in_username: logged_in_user.email,
        collection_fields: vec![]
    };

    for payload_collection_field in payload.collection_fields {
        let creatable_content_field_model = UpdatableCollectionField {
            name: payload_collection_field.name,
            identifier: payload_collection_field.identifier,
            data_type: payload_collection_field.data_type,
            field_type: payload_collection_field.field_type,
        };
        updatable_collection.collection_fields.push(creatable_content_field_model);
    }

    let updated_model = state
        .collection_service
        .update_collection(&state.db, updatable_collection)
        .await?;
    let response = ApiResponse {
        status: true,
        data: updated_model,
    };

    Ok(Json(response))
}
