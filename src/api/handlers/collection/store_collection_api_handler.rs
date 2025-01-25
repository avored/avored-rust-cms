use crate::api::handlers::collection::request::store_collection_request::StoreCollectionRequest;
use crate::avored_state::AvoRedState;
use crate::error::{Error, Result};
use crate::models::collection_model::{CollectionModel, CreatableCollection, CreatableCollectionField};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use axum::extract::State;
use axum::{Extension, Json};
use std::sync::Arc;

pub async fn store_collection_api_handler(
    state: State<Arc<AvoRedState>>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    Json(payload): Json<StoreCollectionRequest>,
) -> Result<Json<ApiResponse<CollectionModel>>> {
    println!("->> {:<12} - store_collection_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("collection_create"))
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

    let mut creatable_model = CreatableCollection {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
        collection_fields: vec![]
    };

    for payload_collection_field in payload.collection_fields {
        let creatable_content_field_model = CreatableCollectionField {
            name: payload_collection_field.name,
            identifier: payload_collection_field.identifier,
            data_type: payload_collection_field.data_type,
            field_type: payload_collection_field.field_type,
        };
        creatable_model.collection_fields.push(creatable_content_field_model);
    }

    let created_model = state
        .collection_service
        .create_collection(&state.db, creatable_model)
        .await?;
    let response = ApiResponse {
        status: true,
        data: created_model,
    };

    Ok(Json(response))
}
