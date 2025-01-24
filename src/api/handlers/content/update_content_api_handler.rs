use std::sync::Arc;

use crate::error::Error;
use crate::{
    avored_state::AvoRedState, error::Result,
};

use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use axum::{
    extract::{Path as AxumPath, State},
    Extension, Json,
};
use crate::api::handlers::content::request::update_content_request::UpdateContentRequest;
use crate::models::content_model::{ContentModel, UpdatableContentField, UpdatableContentModel};

pub async fn update_content_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath((content_type, content_id)): AxumPath<(String, String)>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateContentRequest>,
) -> Result<Json<ApiResponse<ContentModel>>> {
    println!("->> {:<12} - update_content_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("content_edit"))
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

    let content_model = state.content_service.find_by_id(&state.db, content_type, &content_id).await?;

    let mut updatable_content = UpdatableContentModel {
        id: content_model.id,
        name: payload.name,
        identifier: payload.identifier,
        content_type: payload.content_type,
        logged_in_username: logged_in_user.name.clone(),
        content_fields: vec![],
        created_at: content_model.created_at,
        created_by: content_model.created_by,
    };

    for payload_content_field in payload.content_fields {
        let content_field_model = UpdatableContentField {
            name: payload_content_field.name,
            identifier: payload_content_field.identifier,
            data_type: payload_content_field.data_type,
            field_type: payload_content_field.field_type,
            field_content: payload_content_field.field_content,
        };
        updatable_content.content_fields.push(content_field_model);
    }

    let updated_model = state
        .content_service
        .update_content(&state.db, updatable_content)
        .await?;

    let response = ApiResponse {
        status: true,
        data: updated_model,
    };

    Ok(Json(response))
}
