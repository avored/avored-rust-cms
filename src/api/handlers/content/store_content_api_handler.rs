use std::sync::Arc;


use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use crate::{avored_state::AvoRedState, error::Result};
use axum::{extract::State, Extension, Json};
use crate::api::handlers::content::request::store_content_request::StoreContentRequest;
use crate::models::content_model::{ContentModel, CreatableContentField, CreatableContentModel};

pub async fn store_content_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreContentRequest>,
) -> Result<Json<ApiResponse<ContentModel>>> {
    println!("->> {:<12} - store_content_api_handler", "HANDLER");
    let error_messages = payload.validate(&state).await?;

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("content_create"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let mut creatable_content = CreatableContentModel {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.name.clone(),
        content_fields: vec![],
    };

    for payload_content_field in payload.content_fields {
        let creatable_content_field_model = CreatableContentField {
            name: payload_content_field.name,
            identifier: payload_content_field.identifier,
            data_type: payload_content_field.data_type,
            field_type: payload_content_field.field_type,
            field_content: payload_content_field.field_content,
        };
        creatable_content.content_fields.push(creatable_content_field_model);
    }

    let created_content_model = state
        .content_service
        .create_content(&state.db, creatable_content)
        .await?;

    let response = ApiResponse {
        status: true,
        data: created_content_model,
    };

    Ok(Json(response))
}
