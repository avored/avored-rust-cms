use std::sync::Arc;

use crate::models::page_model::PutPageIdentifierModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use axum::response::IntoResponse;
use crate::api::handlers::page::request::put_page_request::PutPageRequest;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use crate::responses::page::PutPageIdentifierResponse;

pub async fn put_page_identifier_api_handler(
    AxumPath(page_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutPageRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - put_page_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("page_edit"))
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

        return Err(Error::BadRequestError(error_response));
    }


    let put_page_identifier = PutPageIdentifierModel {
        id: page_id,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email
    };
    let updated_page_model = state
        .page_service
        .update_page_identifier(&state.db, put_page_identifier)
        .await?;

    let updated_page_response = PutPageIdentifierResponse {
        page: updated_page_model
    };

    let api_response = ApiResponse {
        status: true,
        data: updated_page_response
    };

    Ok(Json(api_response))
}