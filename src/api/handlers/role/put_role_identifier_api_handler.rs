use std::sync::Arc;

use crate::models::role_model::PutRoleIdentifierModel;
use crate::{
    avored_state::AvoRedState, error::Result
};

use axum::{Extension, extract::{Path as AxumPath, State}, Json};
use axum::response::IntoResponse;
use crate::api::handlers::role::request::put_role_request::PutRoleRequest;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::ApiResponse;
use crate::responses::role::PutRoleIdentifierResponse;

pub async fn put_role_identifier_api_handler(
    AxumPath(role_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutRoleRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - put_role_identifier_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("role_edit"))
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


    let put_role_identifier = PutRoleIdentifierModel {
        id: role_id,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email
    };
    let updated_role_model = state
        .role_service
        .update_role_identifier(&state.db, put_role_identifier)
        .await?;

    let updated_role_response = PutRoleIdentifierResponse {
        role: updated_role_model
    };

    let api_response = ApiResponse {
        status: true,
        data: updated_role_response
    };

    Ok(Json(api_response))
}