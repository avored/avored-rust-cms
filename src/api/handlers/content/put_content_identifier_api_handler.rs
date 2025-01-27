use std::sync::Arc;
use crate::{avored_state::AvoRedState, error::Result};
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use axum::response::IntoResponse;
use axum::{
    extract::{Path as AxumPath, State},
    Extension, Json,
};
use crate::api::handlers::content::request::put_content_identifier_request::PutContentIdentifierRequest;
use crate::models::content_model::PutContentIdentifierModel;

pub async fn put_content_identifier_api_handler(
    AxumPath((content_type, content_id)): AxumPath<(String, String)>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutContentIdentifierRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - put_page_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("content_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate(state.clone(), &content_type).await?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages,
        };

        return Err(Error::BadRequest(error_response));
    }

    let put_content_identifier = PutContentIdentifierModel {
        id: content_id,
        collection_type: content_type,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };
    let updated_page_model = state
        .content_service
        .update_content_identifier(&state.db, put_content_identifier)
        .await?;

    let api_response = ApiResponse {
        status: true,
        data: updated_page_model,
    };

    Ok(Json(api_response))
}
