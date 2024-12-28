use std::sync::Arc;

use crate::models::component_model::PutComponentIdentifierModel;
use crate::{avored_state::AvoRedState, error::Result};

use crate::api::handlers::component::request::put_component_request::PutComponentRequest;
use crate::error::Error;
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::responses::component::PutComponentIdentifierResponse;
use crate::responses::ApiResponse;
use axum::response::IntoResponse;
use axum::{
    extract::{Path as AxumPath, State},
    Extension, Json,
};

pub async fn put_component_identifier_api_handler(
    AxumPath(component_id): AxumPath<String>,
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<PutComponentRequest>,
) -> Result<impl IntoResponse> {
    println!(
        "->> {:<12} - put_component_identifier_api_handler",
        "HANDLER"
    );

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("component_edit"))
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

    let put_component_identifier = PutComponentIdentifierModel {
        id: component_id,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };
    let updated_component_model = state
        .component_service
        .update_component_identifier(&state.db, put_component_identifier)
        .await?;

    let updated_component_response = PutComponentIdentifierResponse {
        component: updated_component_model,
    };

    let api_response = ApiResponse {
        status: true,
        data: updated_component_response,
    };

    Ok(Json(api_response))
}
