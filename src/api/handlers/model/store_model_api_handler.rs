use std::sync::Arc;

use crate::api::handlers::model::request::store_model_request::StoreModelRequest;
use crate::error::Error;
use crate::models::model_model::{CreatableModel, ModelModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;
use crate::{avored_state::AvoRedState, error::Result};
use axum::{extract::State, Extension, Json};
use serde::Serialize;

pub async fn store_model_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<StoreModelRequest>,
) -> Result<Json<CreatedModelResponse>> {
    println!("->> {:<12} - store_model_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("model_create"))
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

    let creatable_model = CreatableModel {
        name: payload.name,
        identifier: payload.identifier,
        logged_in_username: logged_in_user.email,
    };

    let created_model_model = state
        .model_service
        .create_model(&state.db, creatable_model)
        .await?;
    let response = CreatedModelResponse {
        status: true,
        model_model: created_model_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct CreatedModelResponse {
    pub status: bool,
    pub model_model: ModelModel,
}
