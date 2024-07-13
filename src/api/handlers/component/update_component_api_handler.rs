use std::sync::Arc;

use crate::{
    avored_state::AvoRedState,
    error::Result,
};
use axum::{Extension, extract::{Path as AxumPath, State}, Json, response::IntoResponse};
use serde::Serialize;
use crate::api::handlers::component::request::update_component_request::{UpdatableFieldDataRequest, UpdateComponentRequest};
use crate::error::Error;
use crate::models::component_model::{ComponentModel, UpdatableComponentModel};
use crate::models::field_model::{UpdatableFieldDataModel, UpdatableFieldModel};
use crate::models::token_claim_model::LoggedInUser;
use crate::models::validation_error::ErrorResponse;

pub async fn update_component_api_handler(
    Extension(logged_in_user): Extension<LoggedInUser>,
    AxumPath(component_id): AxumPath<String>,
    state: State<Arc<AvoRedState>>,
    Json(payload): Json<UpdateComponentRequest>,
) -> Result<impl IntoResponse> {
    println!("->> {:<12} - update_component_api_handler", "HANDLER");

    let has_permission_bool = state
        .admin_user_service
        .has_permission(logged_in_user.clone(), String::from("component_edit"))
        .await?;
    if !has_permission_bool {
        return Err(Error::FORBIDDEN);
    }

    let error_messages = payload.validate()?;

    if !error_messages.is_empty() {
        let error_response = ErrorResponse {
            status: false,
            errors: error_messages
        };

        return Err(Error::BadRequestError(error_response));
    }

    let updateable_component_model = UpdatableComponentModel {
        id: component_id,
        name: payload.name,
        logged_in_username: logged_in_user.email.clone(),
    };
    let mut updated_component_model = state
        .component_service
        .update_component(&state.db, updateable_component_model)
        .await?;

    for payload_field in payload.fields {

        let mut updatable_field_data: Vec<UpdatableFieldDataModel> = vec![];
        let update_field_data_requests: Vec<UpdatableFieldDataRequest> = payload_field.field_data.unwrap_or_else(std::vec::Vec::new);

        for update_field_data_request in update_field_data_requests {
            let update_field_data_option = UpdatableFieldDataModel {
                label: update_field_data_request.label,
                value: update_field_data_request.value
            };
            updatable_field_data.push(update_field_data_option);
        }

        //@todo check for field ID and if not exist then create field and attached field
        let updatable_field = UpdatableFieldModel {
            id: payload_field.id,
            name: payload_field.name,
            identifier: payload_field.identifier,
            field_type: payload_field.field_type,
            field_data: Some(updatable_field_data),
            logged_in_username: logged_in_user.email.clone(),
        };
        let updated_field = state
            .field_service
            .update_field(&state.db, updatable_field)
            .await?;

        updated_component_model.fields.push(updated_field);

    }


    let response = UpdatedComponentResponse {
        status: true,
        component_model: updated_component_model
    };

    Ok(Json(response))
}

#[derive(Serialize, Debug)]
pub struct UpdatedComponentResponse {
    pub status: bool,
    pub component_model: ComponentModel
}