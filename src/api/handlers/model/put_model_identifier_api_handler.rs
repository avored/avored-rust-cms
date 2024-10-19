use std::sync::Arc;

use crate::models::model_model::PutModelIdentifierModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use axum::response::IntoResponse;
use crate::api::handlers::model::request::put_model_request::PutModelRequest;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use crate::responses::model::PutModelIdentifierResponse;

pub async fn put_model_identifier_api_handler(
    AxumPath(model_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutModelRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - put_model_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("model_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::Forbidden);
    }

    let error_messages = payload.validate(state.clone()).await?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequest(error_response));
    }


    let put_model_identifier = PutModelIdentifierModel {
        id: model_id,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email
    };
    let updated_model_model = state
        .model_service
        .update_model_identifier(&state.db, put_model_identifier)
        .await?;

    let updated_model_response = PutModelIdentifierResponse {
        model: updated_model_model
    };

    let api_response = ApiResponse {
        status: true,
        data: updated_model_response
    };

    Ok(Json(api_response))
}