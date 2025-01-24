use std::sync::Arc;

use crate::api::handlers::model::request::update_model_request::UpdateModelRequest;
use crate::models::model_model::ModelModel;
use crate::models::token_claim_model::LoggedInUser;
use crate::{
    avored_state::AvoRedState,
    error::{Error, Result},
    models::{model_model::UpdatableModelModel, validation_error::ErrorResponse},
};
use axum::{
    extract::{Path as AxumPath, State},
    response::IntoResponse,
    Extension, Json,
};
use serde::Serialize;

pub async fn update_model_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(model_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateModelRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_model_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("model_edit"))
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

    let updateable_model_model = UpdatableModelModel {
        id: model_id,
        name: payload.name,
        logged_in_username: logged_in_user.email,
    };
    let updated_model_model = state
        .model_service
        .update_model(&state.db, updateable_model_model)
        .await?;

    let response = UpdatedModelResponse {
        status: true,
        model_model: updated_model_model,
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct UpdatedModelResponse {
    pub status: bool,
    pub model_model: ModelModel,
}
